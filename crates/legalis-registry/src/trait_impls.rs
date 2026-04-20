//! # ActivityAnalytics - Trait Implementations
//!
//! This module contains trait implementations for `ActivityAnalytics`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `ValidationRule`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `ValidationRule`
//! - `ValidationRule`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Default`
//! - `Debug`
//! - `Default`
//! - `From`
//! - `Default`
//! - `ValidationRule`
//! - `Default`
//! - `ValidationRule`
//! - `Default`
//! - `Debug`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use fuzzy_matcher::skim::SkimMatcherV2;
use indexmap::IndexMap;
use lru::LruCache;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroUsize;

use super::functions_2::{ValidationResult, ValidationRule};
use super::types::{
    ActivityAnalytics, CachedAnalytics, CircuitBreakerConfig, DateValidationRule, EventStore,
    ObservabilityCollector, SearchCacheConfig, StatuteArchive, TagValidationRule, WebhookManager,
};
use super::types_3::{
    BulkConfig, CircuitBreaker, LogLevel, RegistryDifference, RetentionPolicy, TagAnalytics,
};
use super::types_4::StatuteRegistry;
use super::types_5::{
    AuditReportFormat, MultiTenantRegistry, Pagination, PiiDetector, RankingConfig,
};
use super::types_6::{
    AuditReportConfig, AuditTrail, DataLineage, RateLimitConfig, RelationshipAnalytics,
    StatuteEntry, ValidJurisdictionRule, WebhookSubscription,
};
use super::types_7::{BulkOperationResult, NonEmptyTitleRule, TemporalAnalytics};
use super::types_8::{
    EnrichmentConfig, MaskingStrategy, NonEmptyIdRule, RateLimiter, StatuteSummary, ValidationError,
};

impl Default for ActivityAnalytics {
    fn default() -> Self {
        Self {
            most_modified: Vec::new(),
            recently_modified: Vec::new(),
            least_modified: Vec::new(),
            frequent_status_changes: Vec::new(),
            avg_modification_frequency_days: 0.0,
        }
    }
}

impl Default for AuditReportConfig {
    fn default() -> Self {
        Self {
            title: "Audit Report".to_string(),
            start_date: None,
            end_date: None,
            include_operations: true,
            include_events: true,
            include_quality: false,
            include_pii_scans: false,
            format: AuditReportFormat::Json,
        }
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl Default for BulkConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            continue_on_error: true,
            max_parallelism: 4,
        }
    }
}

impl Default for BulkOperationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_secs: 60,
            success_threshold: 2,
        }
    }
}

impl Default for DataLineage {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl ValidationRule for DateValidationRule {
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        match (entry.effective_date, entry.expiry_date) {
            (Some(eff), Some(exp)) if exp <= eff => Err(ValidationError::ExpiryBeforeEffective),
            _ => Ok(()),
        }
    }
    fn description(&self) -> String {
        "Expiry date must be after effective date".to_string()
    }
}

impl Default for EnrichmentConfig {
    fn default() -> Self {
        Self {
            enable_auto_tagging: true,
            enable_metadata_inference: true,
            enable_jurisdiction_inference: true,
            min_confidence: 0.7,
        }
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MultiTenantRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRule for NonEmptyIdRule {
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        if entry.statute.id.trim().is_empty() {
            Err(ValidationError::EmptyStatuteId)
        } else {
            Ok(())
        }
    }
    fn description(&self) -> String {
        "Statute ID must not be empty".to_string()
    }
}

impl ValidationRule for NonEmptyTitleRule {
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        if entry.statute.title.trim().is_empty() {
            Err(ValidationError::EmptyTitle)
        } else {
            Ok(())
        }
    }
    fn description(&self) -> String {
        "Title must not be empty".to_string()
    }
}

impl Default for ObservabilityCollector {
    fn default() -> Self {
        Self::new(10000, 10000, LogLevel::Info)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 50,
        }
    }
}

impl Default for PiiDetector {
    fn default() -> Self {
        Self {
            enabled: true,
            min_confidence: 0.7,
            masking_strategy: MaskingStrategy::Redacted,
        }
    }
}

impl Default for RankingConfig {
    fn default() -> Self {
        Self {
            title_weight: 3.0,
            id_weight: 2.0,
            tag_weight: 1.5,
            jurisdiction_weight: 1.0,
            exact_match_boost: 2.0,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 1000,
            window_secs: 60,
            enabled: true,
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

impl Default for RegistryDifference {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RelationshipAnalytics {
    fn default() -> Self {
        Self {
            most_referenced: Vec::new(),
            most_dependencies: Vec::new(),
            supersession_chains: HashMap::new(),
            orphaned_statutes: Vec::new(),
            avg_references_per_statute: 0.0,
        }
    }
}

impl Default for SearchCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 100,
            ttl_seconds: 300,
        }
    }
}

impl std::fmt::Debug for StatuteRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatuteRegistry")
            .field("statutes", &self.statutes)
            .field("versions", &self.versions)
            .field("tag_index", &self.tag_index)
            .field("jurisdiction_index", &self.jurisdiction_index)
            .field("cache", &"<LruCache>")
            .field("fuzzy_matcher", &"<SkimMatcherV2>")
            .field("event_store", &self.event_store)
            .field("webhook_manager", &self.webhook_manager)
            .field("archive", &self.archive)
            .field("retention_policy", &self.retention_policy)
            .finish()
    }
}

impl Default for StatuteRegistry {
    fn default() -> Self {
        Self {
            statutes: IndexMap::new(),
            versions: HashMap::new(),
            tag_index: HashMap::new(),
            jurisdiction_index: HashMap::new(),
            cache: LruCache::new(NonZeroUsize::new(1000).expect("invariant: 1000 is non-zero")),
            fuzzy_matcher: SkimMatcherV2::default(),
            event_store: EventStore::new(),
            webhook_manager: WebhookManager::new(),
            archive: StatuteArchive::new(),
            retention_policy: RetentionPolicy::new(),
            analytics_cache: CachedAnalytics::new(300),
        }
    }
}

impl From<&StatuteEntry> for StatuteSummary {
    fn from(entry: &StatuteEntry) -> Self {
        Self {
            registry_id: entry.registry_id,
            statute_id: entry.statute.id.clone(),
            title: entry.statute.title.clone(),
            version: entry.version,
            status: entry.status,
            jurisdiction: entry.jurisdiction.clone(),
            tags: entry.tags.clone(),
            created_at: entry.created_at,
            modified_at: entry.modified_at,
            is_active: entry.is_active(),
        }
    }
}

impl Default for TagAnalytics {
    fn default() -> Self {
        Self {
            tag_frequency: HashMap::new(),
            tag_cooccurrence: HashMap::new(),
            most_used_tags: Vec::new(),
            least_used_tags: Vec::new(),
            avg_tags_per_statute: 0.0,
        }
    }
}

impl ValidationRule for TagValidationRule {
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        let mut seen = HashSet::new();
        for tag in &entry.tags {
            if tag.trim().is_empty() {
                return Err(ValidationError::EmptyTag);
            }
            if !seen.insert(tag) {
                return Err(ValidationError::DuplicateTag(tag.clone()));
            }
        }
        Ok(())
    }
    fn description(&self) -> String {
        "Tags must not be empty and must be unique".to_string()
    }
}

impl Default for TemporalAnalytics {
    fn default() -> Self {
        Self {
            registrations_per_day: HashMap::new(),
            updates_per_day: HashMap::new(),
            avg_versions_per_statute: 0.0,
            most_versioned_statutes: Vec::new(),
            growth_rate: 0.0,
            peak_activity_date: None,
        }
    }
}

impl ValidationRule for ValidJurisdictionRule {
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        if self.allowed.contains(&entry.jurisdiction) {
            Ok(())
        } else {
            Err(ValidationError::InvalidJurisdiction(
                entry.jurisdiction.clone(),
            ))
        }
    }
    fn description(&self) -> String {
        format!("Jurisdiction must be one of: {:?}", self.allowed)
    }
}

impl Default for WebhookManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WebhookSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebhookSubscription")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("event_filter", &self.event_filter)
            .finish()
    }
}
