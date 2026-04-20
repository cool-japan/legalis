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
fn test_audit_entry_builders() {
    let entry = AuditEntry::new(
        "admin".to_string(),
        AuditOperation::Update,
        AuditResult::Success,
    )
    .with_statute_id("test-123".to_string())
    .with_source("192.168.1.1".to_string())
    .with_metadata("reason".to_string(), "compliance".to_string());

    assert_eq!(entry.statute_id, Some("test-123".to_string()));
    assert_eq!(entry.source, Some("192.168.1.1".to_string()));
    assert_eq!(
        entry.metadata.get("reason"),
        Some(&"compliance".to_string())
    );
}

#[test]
fn test_audit_result_variants() {
    let success = AuditResult::Success;
    let failure = AuditResult::Failure {
        error: "Not found".to_string(),
    };
    let partial = AuditResult::PartialSuccess {
        succeeded: 5,
        failed: 2,
    };

    let entry1 = AuditEntry::new("user1".to_string(), AuditOperation::Register, success);
    assert!(entry1.is_success());

    let entry2 = AuditEntry::new("user2".to_string(), AuditOperation::Delete, failure);
    assert!(entry2.is_failure());

    let entry3 = AuditEntry::new(
        "user3".to_string(),
        AuditOperation::BatchOperation {
            operation_type: "import".to_string(),
            count: 7,
        },
        partial,
    );
    assert!(!entry3.is_success());
    assert!(!entry3.is_failure());
}

#[test]
fn test_audit_trail_basic() {
    let mut trail = AuditTrail::new(100);
    assert_eq!(trail.count(), 0);
    assert!(trail.is_enabled());

    let entry = AuditEntry::new(
        "user1".to_string(),
        AuditOperation::Register,
        AuditResult::Success,
    );
    trail.record(entry.clone());

    assert_eq!(trail.count(), 1);
    assert_eq!(trail.entries().len(), 1);
}

#[test]
fn test_audit_trail_max_entries() {
    let mut trail = AuditTrail::new(3);

    for i in 0..5 {
        let entry = AuditEntry::new(
            format!("user{}", i),
            AuditOperation::Register,
            AuditResult::Success,
        );
        trail.record(entry);
    }

    // Should only keep last 3 entries
    assert_eq!(trail.count(), 3);
}

#[test]
fn test_audit_trail_filtering() {
    let mut trail = AuditTrail::new(100);

    // Add entries with different actors
    trail.record(
        AuditEntry::new(
            "alice".to_string(),
            AuditOperation::Register,
            AuditResult::Success,
        )
        .with_statute_id("s1".to_string()),
    );

    trail.record(
        AuditEntry::new(
            "bob".to_string(),
            AuditOperation::Update,
            AuditResult::Success,
        )
        .with_statute_id("s2".to_string()),
    );

    trail.record(
        AuditEntry::new(
            "alice".to_string(),
            AuditOperation::Delete,
            AuditResult::Failure {
                error: "Not found".to_string(),
            },
        )
        .with_statute_id("s3".to_string()),
    );

    // Test filtering by actor
    let alice_entries = trail.entries_by_actor("alice");
    assert_eq!(alice_entries.len(), 2);

    let bob_entries = trail.entries_by_actor("bob");
    assert_eq!(bob_entries.len(), 1);

    // Test filtering by statute
    let s1_entries = trail.entries_by_statute("s1");
    assert_eq!(s1_entries.len(), 1);

    // Test successful/failed operations
    let successful = trail.successful_operations();
    assert_eq!(successful.len(), 2);

    let failed = trail.failed_operations();
    assert_eq!(failed.len(), 1);
}

#[test]
fn test_audit_trail_enable_disable() {
    let mut trail = AuditTrail::new(100);
    assert!(trail.is_enabled());

    trail.disable();
    assert!(!trail.is_enabled());

    // Recording when disabled should be a no-op
    trail.record(AuditEntry::new(
        "user".to_string(),
        AuditOperation::Register,
        AuditResult::Success,
    ));
    assert_eq!(trail.count(), 0);

    trail.enable();
    trail.record(AuditEntry::new(
        "user".to_string(),
        AuditOperation::Register,
        AuditResult::Success,
    ));
    assert_eq!(trail.count(), 1);
}

#[test]
fn test_audit_trail_export_json() {
    let mut trail = AuditTrail::new(100);
    trail.record(AuditEntry::new(
        "user1".to_string(),
        AuditOperation::Register,
        AuditResult::Success,
    ));

    let json = trail.export_json().unwrap();
    assert!(json.contains("user1"));
    assert!(json.contains("Register"));
}

#[test]
fn test_health_status_methods() {
    let healthy = HealthStatus::Healthy;
    assert!(healthy.is_healthy());
    assert!(!healthy.is_degraded());
    assert!(!healthy.is_unhealthy());

    let degraded = HealthStatus::Degraded {
        issues: vec!["High load".to_string()],
    };
    assert!(!degraded.is_healthy());
    assert!(degraded.is_degraded());
    assert!(!degraded.is_unhealthy());

    let unhealthy = HealthStatus::Unhealthy {
        errors: vec!["Database down".to_string()],
    };
    assert!(!unhealthy.is_healthy());
    assert!(!unhealthy.is_degraded());
    assert!(unhealthy.is_unhealthy());
}

#[test]
fn test_component_health() {
    let healthy = ComponentHealth::healthy("cache".to_string());
    assert_eq!(healthy.name, "cache");
    assert!(healthy.healthy);
    assert!(healthy.message.is_none());

    let unhealthy = ComponentHealth::unhealthy("storage".to_string(), "Disk full".to_string());
    assert_eq!(unhealthy.name, "storage");
    assert!(!unhealthy.healthy);
    assert_eq!(unhealthy.message, Some("Disk full".to_string()));

    let with_metrics = ComponentHealth::healthy("system".to_string())
        .with_metric("cpu".to_string(), 75.0)
        .with_metric("memory".to_string(), 80.5);
    assert_eq!(with_metrics.metrics.get("cpu"), Some(&75.0));
    assert_eq!(with_metrics.metrics.get("memory"), Some(&80.5));
}

#[test]
fn test_health_check() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes
    registry
        .register(StatuteEntry::new(test_statute("h1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("h2"), "US"))
        .unwrap();

    let health = registry.health_check();

    assert_eq!(health.statute_count, 2);
    assert!(health.version_count > 0);
    assert!(health.event_count > 0);
    assert_eq!(health.archived_count, 0);
    assert!(health.memory_estimate_bytes > 0);
    // check_duration_ms is u64, so it's always >= 0

    // Check component health
    assert!(health.component_checks.contains_key("cache"));
    assert!(health.component_checks.contains_key("storage"));
    assert!(health.component_checks.contains_key("indexes"));
    assert!(health.component_checks.contains_key("event_store"));

    // All components should be healthy
    for component in health.component_checks.values() {
        assert!(component.healthy);
    }
}

#[test]
fn test_health_check_empty_registry() {
    let registry = StatuteRegistry::new();
    let health = registry.health_check();

    assert_eq!(health.statute_count, 0);
    assert!(health.status.is_degraded()); // Empty registry is degraded
}

#[test]
fn test_registry_difference_new() {
    let diff = RegistryDifference::new();
    assert_eq!(diff.difference_count(), 0);
    assert!(diff.is_identical());
    assert!(diff.only_in_left.is_empty());
    assert!(diff.only_in_right.is_empty());
    assert!(diff.different_statutes.is_empty());
    assert!(diff.identical_statutes.is_empty());
}

#[test]
fn test_registry_comparison_identical() {
    let mut registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    registry1
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();
    registry2
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();

    let diff = registry1.compare_with(&registry2);
    assert!(diff.is_identical());
    assert_eq!(diff.identical_statutes.len(), 1);
    assert_eq!(diff.difference_count(), 0);
}

#[test]
fn test_registry_comparison_only_in_left() {
    let mut registry1 = StatuteRegistry::new();
    let registry2 = StatuteRegistry::new();

    registry1
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();
    registry1
        .register(StatuteEntry::new(test_statute("c2"), "US"))
        .unwrap();

    let diff = registry1.compare_with(&registry2);
    assert!(!diff.is_identical());
    assert_eq!(diff.only_in_left.len(), 2);
    assert_eq!(diff.only_in_right.len(), 0);
    assert!(diff.only_in_left.contains(&"c1".to_string()));
    assert!(diff.only_in_left.contains(&"c2".to_string()));
}

#[test]
fn test_registry_comparison_only_in_right() {
    let registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    registry2
        .register(StatuteEntry::new(test_statute("c3"), "JP"))
        .unwrap();

    let diff = registry1.compare_with(&registry2);
    assert!(!diff.is_identical());
    assert_eq!(diff.only_in_left.len(), 0);
    assert_eq!(diff.only_in_right.len(), 1);
    assert!(diff.only_in_right.contains(&"c3".to_string()));
}

#[test]
fn test_registry_comparison_different_versions() {
    let mut registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    registry1
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();
    registry2
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();

    // Update one registry
    let existing = registry2.get("c1").unwrap().clone();
    let mut updated_statute = existing.statute.clone();
    updated_statute.title = "Updated Title".to_string();
    registry2.update("c1", updated_statute).unwrap();

    let diff = registry1.compare_with(&registry2);
    assert!(!diff.is_identical());
    assert_eq!(diff.different_statutes.len(), 1);
    assert!(
        diff.different_statutes[0]
            .differing_fields
            .contains(&"title".to_string())
    );
    assert!(
        diff.different_statutes[0]
            .differing_fields
            .contains(&"version".to_string())
    );
}

#[test]
fn test_registry_comparison_summary() {
    let mut registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    registry1
        .register(StatuteEntry::new(test_statute("c1"), "US"))
        .unwrap();
    registry1
        .register(StatuteEntry::new(test_statute("c2"), "US"))
        .unwrap();

    registry2
        .register(StatuteEntry::new(test_statute("c2"), "US"))
        .unwrap();
    registry2
        .register(StatuteEntry::new(test_statute("c3"), "JP"))
        .unwrap();

    let diff = registry1.compare_with(&registry2);
    let summary = diff.summary();

    assert!(summary.contains("Only in left: 1"));
    assert!(summary.contains("Only in right: 1"));
    assert!(summary.contains("Identical: 1"));
}

#[test]
fn test_bulk_config_default() {
    let config = BulkConfig::default();
    assert_eq!(config.batch_size, 100);
    assert!(config.continue_on_error);
    assert_eq!(config.max_parallelism, 4);
}

#[test]
fn test_bulk_config_builders() {
    let config = BulkConfig::new(50)
        .with_continue_on_error(false)
        .with_max_parallelism(8);

    assert_eq!(config.batch_size, 50);
    assert!(!config.continue_on_error);
    assert_eq!(config.max_parallelism, 8);
}

#[test]
fn test_bulk_operation_result() {
    let result = BulkOperationResult::new();
    assert_eq!(result.total_processed, 0);
    assert_eq!(result.successful, 0);
    assert_eq!(result.failed, 0);
    assert!(!result.is_all_successful());
    assert_eq!(result.success_rate(), 0.0);

    let mut result2 = BulkOperationResult::new();
    result2.total_processed = 10;
    result2.successful = 7;
    result2.failed = 3;

    assert!(!result2.is_all_successful());
    assert!((result2.success_rate() - 0.7).abs() < 0.01);
}

#[test]
fn test_bulk_register_success() {
    let mut registry = StatuteRegistry::new();
    let entries = vec![
        StatuteEntry::new(test_statute("bulk-1"), "US"),
        StatuteEntry::new(test_statute("bulk-2"), "US"),
        StatuteEntry::new(test_statute("bulk-3"), "US"),
    ];

    let config = BulkConfig::new(2);
    let result = registry.bulk_register(entries, config);

    assert_eq!(result.total_processed, 3);
    assert_eq!(result.successful, 3);
    assert_eq!(result.failed, 0);
    assert!(result.is_all_successful());
    assert_eq!(result.success_rate(), 1.0);
}

#[test]
fn test_bulk_register_partial_failure() {
    let mut registry = StatuteRegistry::new();

    // Pre-register one to cause duplicate error
    registry
        .register(StatuteEntry::new(test_statute("bulk-2"), "US"))
        .unwrap();

    let entries = vec![
        StatuteEntry::new(test_statute("bulk-1"), "US"),
        StatuteEntry::new(test_statute("bulk-2"), "US"), // Duplicate
        StatuteEntry::new(test_statute("bulk-3"), "US"),
    ];

    let config = BulkConfig::default().with_continue_on_error(true);
    let result = registry.bulk_register(entries, config);

    assert_eq!(result.total_processed, 3);
    assert_eq!(result.successful, 2);
    assert_eq!(result.failed, 1);
    assert!(!result.is_all_successful());
    assert!(result.errors.contains_key("bulk-2"));
}

#[test]
fn test_bulk_register_stop_on_error() {
    let mut registry = StatuteRegistry::new();
    registry
        .register(StatuteEntry::new(test_statute("bulk-2"), "US"))
        .unwrap();

    let entries = vec![
        StatuteEntry::new(test_statute("bulk-1"), "US"),
        StatuteEntry::new(test_statute("bulk-2"), "US"), // Duplicate
        StatuteEntry::new(test_statute("bulk-3"), "US"), // Won't be processed
    ];

    let config = BulkConfig::default().with_continue_on_error(false);
    let result = registry.bulk_register(entries, config);

    assert_eq!(result.total_processed, 2);
    assert_eq!(result.successful, 1);
    assert_eq!(result.failed, 1);
}

#[test]
fn test_bulk_delete_success() {
    let mut registry = StatuteRegistry::new();

    // Register statutes
    registry
        .register(StatuteEntry::new(test_statute("del-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("del-2"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("del-3"), "US"))
        .unwrap();

    let statute_ids = vec![
        "del-1".to_string(),
        "del-2".to_string(),
        "del-3".to_string(),
    ];

    let config = BulkConfig::default();
    let result = registry.bulk_delete_with_config(statute_ids, config);

    assert_eq!(result.total_processed, 3);
    assert_eq!(result.successful, 3);
    assert_eq!(result.failed, 0);
    assert!(result.is_all_successful());
}

#[test]
fn test_bulk_delete_partial_failure() {
    let mut registry = StatuteRegistry::new();

    // Register only 2 statutes
    registry
        .register(StatuteEntry::new(test_statute("del-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("del-3"), "US"))
        .unwrap();

    let statute_ids = vec![
        "del-1".to_string(),
        "del-2".to_string(), // Doesn't exist
        "del-3".to_string(),
    ];

    let config = BulkConfig::default();
    let result = registry.bulk_delete_with_config(statute_ids, config);

    assert_eq!(result.total_processed, 3);
    assert_eq!(result.successful, 2);
    assert_eq!(result.failed, 1);
    assert!(result.errors.contains_key("del-2"));
}

#[test]
fn test_stream_ids() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("stream-1"), "US").with_tag("civil"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("stream-2"), "JP").with_tag("criminal"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("stream-3"), "US").with_tag("civil"))
        .unwrap();

    // Stream US statutes
    let us_ids = registry.stream_ids(|entry| entry.jurisdiction == "US");
    assert_eq!(us_ids.len(), 2);
    assert!(us_ids.contains(&"stream-1".to_string()));
    assert!(us_ids.contains(&"stream-3".to_string()));

    // Stream civil statutes
    let civil_ids = registry.stream_ids(|entry| entry.tags.contains(&"civil".to_string()));
    assert_eq!(civil_ids.len(), 2);
}

#[test]
fn test_stream_entries() {
    let mut registry = StatuteRegistry::new();

    for i in 1..=10 {
        registry
            .register(StatuteEntry::new(
                test_statute(&format!("stream-{}", i)),
                "US",
            ))
            .unwrap();
    }

    // Stream all entries with batch size 3
    let batches = registry.stream_entries(|_| true, 3);
    assert_eq!(batches.len(), 4); // 3 + 3 + 3 + 1
    assert_eq!(batches[0].len(), 3);
    assert_eq!(batches[1].len(), 3);
    assert_eq!(batches[2].len(), 3);
    assert_eq!(batches[3].len(), 1);
}

#[test]
fn test_audit_operation_variants() {
    let _register = AuditOperation::Register;
    let _update = AuditOperation::Update;
    let _delete = AuditOperation::Delete;
    let _archive = AuditOperation::Archive;
    let _status_change = AuditOperation::StatusChange {
        from: StatuteStatus::Draft,
        to: StatuteStatus::Active,
    };
    let _add_tag = AuditOperation::AddTag {
        tag: "test".to_string(),
    };
    let _export = AuditOperation::Export {
        format: "json".to_string(),
    };
    let _batch = AuditOperation::BatchOperation {
        operation_type: "import".to_string(),
        count: 100,
    };
}

// ========================================================================
// Tests for Session 9: Benchmarking, Rate Limiting, Circuit Breaker, Observability
// ========================================================================

#[test]
fn test_benchmark_result_creation() {
    let durations = vec![100, 150, 120, 180, 110];
    let result = BenchmarkResult::new("test_op".to_string(), 5, durations);

    assert_eq!(result.name, "test_op");
    assert_eq!(result.iterations, 5);
    assert_eq!(result.min_duration_us, 100);
    assert_eq!(result.max_duration_us, 180);
    assert!(result.avg_duration_us > 0.0);
    assert!(result.ops_per_sec > 0.0);

    let summary = result.summary();
    assert!(summary.contains("test_op"));
    assert!(summary.contains("ops/sec"));
}

#[test]
fn test_benchmark_suite() {
    let mut suite = BenchmarkSuite::new();
    assert_eq!(suite.results().len(), 0);

    let result1 = BenchmarkResult::new("op1".to_string(), 10, vec![100; 10]);
    let result2 = BenchmarkResult::new("op2".to_string(), 5, vec![200; 5]);

    suite.add_result(result1);
    suite.add_result(result2);

    assert_eq!(suite.results().len(), 2);

    let summary = suite.summary();
    assert!(summary.contains("Benchmark Results"));
    assert!(summary.contains("op1"));
    assert!(summary.contains("op2"));

    let json = suite.export_json().unwrap();
    assert!(json.contains("op1"));
    assert!(json.contains("op2"));
}

#[test]
fn test_rate_limit_config() {
    let config = RateLimitConfig::default();
    assert_eq!(config.max_requests, 1000);
    assert_eq!(config.window_secs, 60);
    assert!(config.enabled);

    let custom = RateLimitConfig::new(100, 30);
    assert_eq!(custom.max_requests, 100);
    assert_eq!(custom.window_secs, 30);

    let disabled = RateLimitConfig::disabled();
    assert!(!disabled.enabled);
}

#[test]
fn test_rate_limiter_basic() {
    let config = RateLimitConfig::new(3, 60);
    let mut limiter = RateLimiter::new(config);

    // First 3 requests should be allowed
    assert!(limiter.check_rate_limit("user1"));
    assert!(limiter.check_rate_limit("user1"));
    assert!(limiter.check_rate_limit("user1"));

    // 4th request should be denied
    assert!(!limiter.check_rate_limit("user1"));

    // Different user should be allowed
    assert!(limiter.check_rate_limit("user2"));
}

#[test]
fn test_rate_limiter_counts() {
    let config = RateLimitConfig::new(5, 60);
    let mut limiter = RateLimiter::new(config);

    limiter.check_rate_limit("user1");
    limiter.check_rate_limit("user1");
    limiter.check_rate_limit("user1");

    assert_eq!(limiter.current_count("user1"), 3);
    assert_eq!(limiter.remaining("user1"), 2);
    assert_eq!(limiter.current_count("user2"), 0);
}

#[test]
fn test_rate_limiter_reset() {
    let config = RateLimitConfig::new(2, 60);
    let mut limiter = RateLimiter::new(config);

    limiter.check_rate_limit("user1");
    limiter.check_rate_limit("user1");
    assert!(!limiter.check_rate_limit("user1"));

    // Reset should allow new requests
    limiter.reset("user1");
    assert!(limiter.check_rate_limit("user1"));
}

#[test]
fn test_rate_limiter_disabled() {
    let config = RateLimitConfig::disabled();
    let mut limiter = RateLimiter::new(config);

    // All requests should be allowed when disabled
    for _ in 0..100 {
        assert!(limiter.check_rate_limit("user1"));
    }
}

#[test]
fn test_rate_limiter_clear_all() {
    let config = RateLimitConfig::new(5, 60);
    let mut limiter = RateLimiter::new(config);

    limiter.check_rate_limit("user1");
    limiter.check_rate_limit("user2");
    limiter.check_rate_limit("user3");

    limiter.clear_all();

    assert_eq!(limiter.current_count("user1"), 0);
    assert_eq!(limiter.current_count("user2"), 0);
    assert_eq!(limiter.current_count("user3"), 0);
}

#[test]
fn test_circuit_breaker_config() {
    let config = CircuitBreakerConfig::default();
    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.timeout_secs, 60);
    assert_eq!(config.success_threshold, 2);

    let custom = CircuitBreakerConfig::new(3, 30, 1);
    assert_eq!(custom.failure_threshold, 3);
    assert_eq!(custom.timeout_secs, 30);
    assert_eq!(custom.success_threshold, 1);
}

#[test]
fn test_circuit_breaker_closed_to_open() {
    let config = CircuitBreakerConfig::new(3, 60, 2);
    let mut breaker = CircuitBreaker::new(config);

    assert_eq!(*breaker.state(), CircuitState::Closed);
    assert!(breaker.is_request_allowed());

    // Record failures
    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.failure_count(), 1);

    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.failure_count(), 2);

    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Open);

    // Requests should be denied when open
    assert!(!breaker.is_request_allowed());
}

#[test]
fn test_circuit_breaker_success_resets_failures() {
    let config = CircuitBreakerConfig::new(5, 60, 2);
    let mut breaker = CircuitBreaker::new(config);

    breaker.record_failure();
    breaker.record_failure();
    assert_eq!(breaker.failure_count(), 2);

    breaker.record_success();
    assert_eq!(breaker.failure_count(), 0);
}

#[test]
fn test_circuit_breaker_half_open_to_closed() {
    let config = CircuitBreakerConfig::new(2, 0, 2); // 0 timeout for immediate testing
    let mut breaker = CircuitBreaker::new(config);

    // Open the circuit
    breaker.record_failure();
    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Open);

    // Should transition to half-open after timeout
    std::thread::sleep(std::time::Duration::from_millis(10));
    assert!(breaker.is_request_allowed());
    assert_eq!(*breaker.state(), CircuitState::HalfOpen);

    // Record successful requests
    breaker.record_success();
    assert_eq!(*breaker.state(), CircuitState::HalfOpen);

    breaker.record_success();
    assert_eq!(*breaker.state(), CircuitState::Closed);
}

#[test]
fn test_circuit_breaker_half_open_to_open() {
    let config = CircuitBreakerConfig::new(2, 0, 2);
    let mut breaker = CircuitBreaker::new(config);

    // Open the circuit
    breaker.record_failure();
    breaker.record_failure();

    // Transition to half-open
    std::thread::sleep(std::time::Duration::from_millis(10));
    breaker.is_request_allowed();
    assert_eq!(*breaker.state(), CircuitState::HalfOpen);

    // Failure in half-open should reopen circuit
    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Open);
}

#[test]
fn test_circuit_breaker_reset() {
    let config = CircuitBreakerConfig::new(2, 60, 2);
    let mut breaker = CircuitBreaker::new(config);

    breaker.record_failure();
    breaker.record_failure();
    assert_eq!(*breaker.state(), CircuitState::Open);

    breaker.reset();
    assert_eq!(*breaker.state(), CircuitState::Closed);
    assert_eq!(breaker.failure_count(), 0);
}

#[test]
fn test_circuit_breaker_force_open() {
    let mut breaker = CircuitBreaker::default();
    assert_eq!(*breaker.state(), CircuitState::Closed);

    breaker.force_open();
    assert_eq!(*breaker.state(), CircuitState::Open);
    assert!(!breaker.is_request_allowed());
}

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Trace < LogLevel::Debug);
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_log_entry_creation() {
    let entry = LogEntry::new(
        LogLevel::Info,
        "register".to_string(),
        "Statute registered".to_string(),
    );

    assert_eq!(entry.level, LogLevel::Info);
    assert_eq!(entry.operation, "register");
    assert_eq!(entry.message, "Statute registered");
    assert!(entry.fields.is_empty());
}

#[test]
fn test_log_entry_with_fields() {
    let entry = LogEntry::new(
        LogLevel::Warn,
        "update".to_string(),
        "Update warning".to_string(),
    )
    .with_field("statute_id".to_string(), "test-123".to_string())
    .with_field("version".to_string(), "2".to_string());

    assert_eq!(
        entry.fields.get("statute_id"),
        Some(&"test-123".to_string())
    );
    assert_eq!(entry.fields.get("version"), Some(&"2".to_string()));
}

#[test]
fn test_metric_entry_counter() {
    let metric = MetricEntry::counter("requests".to_string(), 100);
    assert_eq!(metric.name, "requests");
    assert!(matches!(
        metric.metric_type,
        MetricType::Counter { value: 100 }
    ));
}

#[test]
fn test_metric_entry_gauge() {
    let metric = MetricEntry::gauge("cpu_usage".to_string(), 75.5);
    assert_eq!(metric.name, "cpu_usage");
    assert!(
        matches!(metric.metric_type, MetricType::Gauge { value } if (value - 75.5).abs() < 0.01)
    );
}

#[test]
fn test_metric_entry_timing() {
    let metric = MetricEntry::timing("operation_duration".to_string(), 12345);
    assert_eq!(metric.name, "operation_duration");
    assert!(matches!(
        metric.metric_type,
        MetricType::Timing { duration_us: 12345 }
    ));
}

#[test]
fn test_metric_entry_with_labels() {
    let metric = MetricEntry::counter("http_requests".to_string(), 50)
        .with_label("method".to_string(), "GET".to_string())
        .with_label("status".to_string(), "200".to_string());

    assert_eq!(metric.labels.get("method"), Some(&"GET".to_string()));
    assert_eq!(metric.labels.get("status"), Some(&"200".to_string()));
}

#[test]
fn test_observability_collector_basic() {
    let mut collector = ObservabilityCollector::default();

    let log = LogEntry::new(
        LogLevel::Info,
        "test".to_string(),
        "Test message".to_string(),
    );
    collector.log(log);

    assert_eq!(collector.logs().len(), 1);

    let metric = MetricEntry::counter("test_metric".to_string(), 1);
    collector.metric(metric);

    assert_eq!(collector.metrics().len(), 1);
}

#[test]
fn test_observability_collector_log_level_filtering() {
    let mut collector = ObservabilityCollector::new(100, 100, LogLevel::Warn);

    // Debug and Info should be filtered out
    collector.log(LogEntry::new(
        LogLevel::Debug,
        "op".to_string(),
        "debug".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Info,
        "op".to_string(),
        "info".to_string(),
    ));
    assert_eq!(collector.logs().len(), 0);

    // Warn and Error should be collected
    collector.log(LogEntry::new(
        LogLevel::Warn,
        "op".to_string(),
        "warn".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Error,
        "op".to_string(),
        "error".to_string(),
    ));
    assert_eq!(collector.logs().len(), 2);
}

#[test]
fn test_observability_collector_log_rotation() {
    let mut collector = ObservabilityCollector::new(3, 10, LogLevel::Info);

    // Add 5 logs, should only keep last 3
    for i in 0..5 {
        collector.log(LogEntry::new(
            LogLevel::Info,
            "op".to_string(),
            format!("Log {}", i),
        ));
    }

    assert_eq!(collector.logs().len(), 3);
}

#[test]
fn test_observability_collector_metric_rotation() {
    let mut collector = ObservabilityCollector::new(10, 3, LogLevel::Info);

    // Add 5 metrics, should only keep last 3
    for i in 0..5 {
        collector.metric(MetricEntry::counter(format!("metric_{}", i), i as u64));
    }

    assert_eq!(collector.metrics().len(), 3);
}

#[test]
fn test_observability_collector_logs_by_level() {
    let mut collector = ObservabilityCollector::default();

    collector.log(LogEntry::new(
        LogLevel::Info,
        "op".to_string(),
        "info1".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Warn,
        "op".to_string(),
        "warn1".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Info,
        "op".to_string(),
        "info2".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Error,
        "op".to_string(),
        "error1".to_string(),
    ));

    let info_logs = collector.logs_by_level(LogLevel::Info);
    assert_eq!(info_logs.len(), 2);

    let warn_logs = collector.logs_by_level(LogLevel::Warn);
    assert_eq!(warn_logs.len(), 1);
}

#[test]
fn test_observability_collector_logs_by_operation() {
    let mut collector = ObservabilityCollector::default();

    collector.log(LogEntry::new(
        LogLevel::Info,
        "register".to_string(),
        "msg1".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Info,
        "update".to_string(),
        "msg2".to_string(),
    ));
    collector.log(LogEntry::new(
        LogLevel::Info,
        "register".to_string(),
        "msg3".to_string(),
    ));

    let register_logs = collector.logs_by_operation("register");
    assert_eq!(register_logs.len(), 2);

    let update_logs = collector.logs_by_operation("update");
    assert_eq!(update_logs.len(), 1);
}

#[test]
fn test_observability_collector_metrics_by_name() {
    let mut collector = ObservabilityCollector::default();

    collector.metric(MetricEntry::counter("requests".to_string(), 10));
    collector.metric(MetricEntry::gauge("cpu".to_string(), 50.0));
    collector.metric(MetricEntry::counter("requests".to_string(), 20));

    let request_metrics = collector.metrics_by_name("requests");
    assert_eq!(request_metrics.len(), 2);

    let cpu_metrics = collector.metrics_by_name("cpu");
    assert_eq!(cpu_metrics.len(), 1);
}

#[test]
fn test_observability_collector_clear() {
    let mut collector = ObservabilityCollector::default();

    collector.log(LogEntry::new(
        LogLevel::Info,
        "op".to_string(),
        "msg".to_string(),
    ));
    collector.metric(MetricEntry::counter("test".to_string(), 1));

    collector.clear_logs();
    assert_eq!(collector.logs().len(), 0);
    assert_eq!(collector.metrics().len(), 1);

    collector.clear_metrics();
    assert_eq!(collector.metrics().len(), 0);
}

#[test]
fn test_observability_collector_export_json() {
    let mut collector = ObservabilityCollector::default();

    collector.log(LogEntry::new(
        LogLevel::Info,
        "test".to_string(),
        "message".to_string(),
    ));
    collector.metric(MetricEntry::counter("test_metric".to_string(), 42));

    let logs_json = collector.export_logs_json().unwrap();
    assert!(logs_json.contains("test"));
    assert!(logs_json.contains("message"));

    let metrics_json = collector.export_metrics_json().unwrap();
    assert!(metrics_json.contains("test_metric"));
    assert!(metrics_json.contains("42"));
}

// ========================================================================
// Data Quality Tests
// ========================================================================

#[test]
fn test_quality_score_creation() {
    let score = QualityScore::new(80.0, 90.0, 70.0, 85.0);

    // Weighted average: 80*0.4 + 90*0.3 + 70*0.2 + 85*0.1 = 32 + 27 + 14 + 8.5 = 81.5
    assert!((score.overall - 81.5).abs() < 0.1);
    assert_eq!(score.completeness, 80.0);
    assert_eq!(score.consistency, 90.0);
    assert_eq!(score.metadata_richness, 70.0);
    assert_eq!(score.documentation_quality, 85.0);
}

#[test]
fn test_quality_score_grade() {
    assert_eq!(QualityScore::new(95.0, 95.0, 95.0, 95.0).grade(), 'A');
    assert_eq!(QualityScore::new(85.0, 85.0, 85.0, 85.0).grade(), 'B');
    assert_eq!(QualityScore::new(75.0, 75.0, 75.0, 75.0).grade(), 'C');
    assert_eq!(QualityScore::new(65.0, 65.0, 65.0, 65.0).grade(), 'D');
    assert_eq!(QualityScore::new(50.0, 50.0, 50.0, 50.0).grade(), 'F');
}

#[test]
fn test_quality_score_meets_threshold() {
    let score = QualityScore::new(80.0, 80.0, 80.0, 80.0);
    assert!(score.meets_threshold(70.0));
    assert!(score.meets_threshold(80.0));
    assert!(!score.meets_threshold(85.0));
}

#[test]
fn test_quality_assessment_creation() {
    let score = QualityScore::new(75.0, 85.0, 65.0, 70.0);
    let assessment = QualityAssessment::new("test-1".to_string(), score);

    assert_eq!(assessment.statute_id, "test-1");
    assert_eq!(assessment.score.overall, score.overall);
    assert_eq!(assessment.issues.len(), 0);
    assert_eq!(assessment.suggestions.len(), 0);
    assert!(!assessment.has_issues());
}

#[test]
fn test_quality_assessment_with_issues() {
    let score = QualityScore::new(50.0, 60.0, 40.0, 50.0);
    let assessment = QualityAssessment::new("test-1".to_string(), score)
        .with_issue("Missing metadata".to_string())
        .with_suggestion("Add description field".to_string())
        .with_issue("Title too short".to_string());

    assert_eq!(assessment.issues.len(), 2);
    assert_eq!(assessment.suggestions.len(), 1);
    assert!(assessment.has_issues());
    assert!(assessment.issues.contains(&"Missing metadata".to_string()));
}

#[test]
fn test_calculate_quality_score() {
    let registry = StatuteRegistry::new();

    // Create a high-quality statute
    let entry = StatuteEntry::new(test_statute("high-quality"), "US")
        .with_tag("civil")
        .with_tag("rights")
        .with_metadata(
            "description".to_string(),
            "A comprehensive statute".to_string(),
        )
        .with_metadata("author".to_string(), "Legislature".to_string());

    let score = registry.calculate_quality_score(&entry);

    // Should have high scores due to tags and metadata
    assert!(score.overall > 60.0);
    assert!(score.completeness > 50.0);
    assert_eq!(score.consistency, 100.0); // No date inconsistencies
    assert!(score.metadata_richness > 0.0);
}

#[test]
fn test_assess_quality() {
    let mut registry = StatuteRegistry::new();

    // Create a statute with issues
    let entry = StatuteEntry::new(test_statute("test-1"), "US");
    registry.register(entry).unwrap();

    let assessment = registry.assess_quality("test-1").unwrap();

    assert_eq!(assessment.statute_id, "test-1");
    assert!(assessment.has_issues());
    // Should flag missing tags and metadata
    assert!(
        assessment
            .issues
            .iter()
            .any(|i| i.contains("tags") || i.contains("metadata"))
    );
}

#[test]
fn test_assess_quality_nonexistent() {
    let registry = StatuteRegistry::new();
    let result = registry.assess_quality("nonexistent");

    assert!(result.is_err());
    assert!(matches!(result, Err(RegistryError::StatuteNotFound(_))));
}

#[test]
fn test_assess_all_quality() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("test-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-2"), "UK"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-3"), "JP"))
        .unwrap();

    let assessments = registry.assess_all_quality();
    assert_eq!(assessments.len(), 3);
}

#[test]
fn test_similarity_score_creation() {
    let score = SimilarityScore::new(0.8, 0.9, 0.7);

    // Weighted average: 0.8*0.4 + 0.9*0.5 + 0.7*0.1 = 0.32 + 0.45 + 0.07 = 0.84
    assert!((score.overall - 0.84).abs() < 0.01);
    assert_eq!(score.title, 0.8);
    assert_eq!(score.content, 0.9);
    assert_eq!(score.metadata, 0.7);
}

#[test]
fn test_similarity_score_likely_duplicate() {
    let high_sim = SimilarityScore::new(0.9, 0.95, 0.85);
    let medium_sim = SimilarityScore::new(0.7, 0.75, 0.65);
    let low_sim = SimilarityScore::new(0.3, 0.4, 0.2);

    assert!(high_sim.is_likely_duplicate(0.85));
    assert!(!medium_sim.is_likely_duplicate(0.85));
    assert!(!low_sim.is_likely_duplicate(0.85));
}
