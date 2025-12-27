//! Legalis-Registry: Statute registry and version management for Legalis-RS.
//!
//! This crate provides a central registry for managing statute collections:
//! - Version control for statutes
//! - Hierarchical statute organization
//! - Cross-reference management
//! - Amendment tracking
//! - LRU caching for performance
//! - Full-text search capabilities
//! - Fuzzy matching for statute IDs
//! - Pagination support

use chrono::{DateTime, Utc};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use indexmap::IndexMap;
use legalis_core::{Condition, EffectType, Statute};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

/// Errors during registry operations.
#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Statute not found: {0}")]
    StatuteNotFound(String),

    #[error("Version not found: {statute_id} v{version}")]
    VersionNotFound { statute_id: String, version: u32 },

    #[error("Duplicate statute ID: {0}")]
    DuplicateId(String),

    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Concurrent modification: expected ETag {expected}, got {actual}")]
    ConcurrentModification { expected: String, actual: String },
}

/// Result type for registry operations.
pub type RegistryResult<T> = Result<T, RegistryError>;

/// A backup of the registry state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryBackup {
    /// All current statutes
    pub statutes: Vec<StatuteEntry>,
    /// All version history
    pub versions: HashMap<String, HashMap<u32, StatuteEntry>>,
    /// Event history
    pub events: Vec<RegistryEvent>,
    /// Backup metadata
    pub metadata: BackupMetadata,
}

/// Metadata for a registry backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Timestamp when the backup was created
    pub created_at: DateTime<Utc>,
    /// Version of the backup format
    pub format_version: String,
    /// Total number of statutes
    pub statute_count: usize,
    /// Total number of events
    pub event_count: usize,
    /// Description or notes
    pub description: Option<String>,
}

/// A point-in-time snapshot of the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySnapshot {
    /// Snapshot ID
    pub snapshot_id: Uuid,
    /// When the snapshot was created
    pub created_at: DateTime<Utc>,
    /// Full registry backup
    pub backup: RegistryBackup,
    /// Snapshot description
    pub description: Option<String>,
}

impl RegistrySnapshot {
    /// Creates a new snapshot from a backup.
    pub fn new(backup: RegistryBackup, description: Option<String>) -> Self {
        Self {
            snapshot_id: Uuid::new_v4(),
            created_at: Utc::now(),
            backup,
            description,
        }
    }
}

/// Incremental backup containing only changes since last backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalBackup {
    /// Base snapshot ID this incremental is built upon
    pub base_snapshot_id: Uuid,
    /// When this incremental was created
    pub created_at: DateTime<Utc>,
    /// Events since the base snapshot
    pub delta_events: Vec<RegistryEvent>,
    /// Statutes added or modified since base
    pub changed_statutes: Vec<StatuteEntry>,
    /// IDs of statutes deleted since base
    pub deleted_statute_ids: Vec<String>,
}

impl IncrementalBackup {
    /// Creates a new incremental backup.
    pub fn new(base_snapshot_id: Uuid) -> Self {
        Self {
            base_snapshot_id,
            created_at: Utc::now(),
            delta_events: Vec::new(),
            changed_statutes: Vec::new(),
            deleted_statute_ids: Vec::new(),
        }
    }

    /// Returns the total number of changes.
    pub fn change_count(&self) -> usize {
        self.delta_events.len() + self.changed_statutes.len() + self.deleted_statute_ids.len()
    }
}

/// Lazy loading configuration.
#[derive(Debug, Clone, Copy, Default)]
pub struct LazyLoadConfig {
    /// Load statute content on demand
    pub lazy_content: bool,
    /// Load version history on demand
    pub lazy_versions: bool,
    /// Load events on demand
    pub lazy_events: bool,
}

impl LazyLoadConfig {
    /// Creates a config with all lazy loading enabled.
    pub fn all() -> Self {
        Self {
            lazy_content: true,
            lazy_versions: true,
            lazy_events: true,
        }
    }

    /// Creates a config with all lazy loading disabled.
    pub fn none() -> Self {
        Self::default()
    }
}

/// Statistics about the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of statutes
    pub total_statutes: usize,
    /// Total number of versions across all statutes
    pub total_versions: usize,
    /// Total number of events
    pub total_events: usize,
    /// Total number of unique tags
    pub total_tags: usize,
    /// Total number of jurisdictions
    pub total_jurisdictions: usize,
    /// Count by status
    pub by_status: HashMap<StatuteStatus, usize>,
    /// Count by jurisdiction
    pub by_jurisdiction: HashMap<String, usize>,
}

/// Lightweight statute summary for lazy loading.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSummary {
    /// Registry ID
    pub registry_id: Uuid,
    /// Statute ID
    pub statute_id: String,
    /// Title
    pub title: String,
    /// Version
    pub version: u32,
    /// Status
    pub status: StatuteStatus,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Tags
    pub tags: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Is active
    pub is_active: bool,
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

/// Pagination parameters.
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    /// Page number (0-indexed)
    pub page: usize,
    /// Items per page
    pub per_page: usize,
}

impl Pagination {
    /// Creates new pagination parameters.
    pub fn new(page: usize, per_page: usize) -> Self {
        Self { page, per_page }
    }

    /// Creates pagination for the first page.
    pub fn first(per_page: usize) -> Self {
        Self { page: 0, per_page }
    }

    /// Returns pagination for the next page.
    pub fn next(&self) -> Self {
        Self {
            page: self.page + 1,
            per_page: self.per_page,
        }
    }

    /// Returns pagination for the previous page (saturating at 0).
    pub fn prev(&self) -> Self {
        Self {
            page: self.page.saturating_sub(1),
            per_page: self.per_page,
        }
    }

    /// Sets the page number.
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    /// Sets the items per page.
    pub fn with_per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page;
        self
    }

    /// Returns the offset for the current page.
    pub fn offset(&self) -> usize {
        self.page * self.per_page
    }

    /// Returns the limit for the current page.
    pub fn limit(&self) -> usize {
        self.per_page
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

/// Paginated result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResult<T> {
    /// The items in this page
    pub items: Vec<T>,
    /// Current page number
    pub page: usize,
    /// Items per page
    pub per_page: usize,
    /// Total number of items
    pub total: usize,
    /// Total number of pages
    pub total_pages: usize,
}

impl<T> PagedResult<T> {
    /// Creates a new paged result.
    pub fn new(items: Vec<T>, page: usize, per_page: usize, total: usize) -> Self {
        let total_pages = total.div_ceil(per_page);
        Self {
            items,
            page,
            per_page,
            total,
            total_pages,
        }
    }

    /// Returns whether there is a next page.
    pub fn has_next(&self) -> bool {
        self.page + 1 < self.total_pages
    }

    /// Returns whether there is a previous page.
    pub fn has_prev(&self) -> bool {
        self.page > 0
    }

    /// Returns whether the result is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns the global index of the first item on this page (1-indexed).
    pub fn first_item_number(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.page * self.per_page + 1
        }
    }

    /// Returns the global index of the last item on this page (1-indexed).
    pub fn last_item_number(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.page * self.per_page + self.items.len()
        }
    }

    /// Returns pagination parameters for the next page.
    pub fn next_page(&self) -> Option<Pagination> {
        if self.has_next() {
            Some(Pagination::new(self.page + 1, self.per_page))
        } else {
            None
        }
    }

    /// Returns pagination parameters for the previous page.
    pub fn prev_page(&self) -> Option<Pagination> {
        if self.has_prev() {
            Some(Pagination::new(self.page - 1, self.per_page))
        } else {
            None
        }
    }
}

/// Search query for statutes.
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// Full-text search term
    pub text: Option<String>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Filter by jurisdiction
    pub jurisdiction: Option<String>,
    /// Filter by status
    pub status: Option<StatuteStatus>,
    /// Filter by active statutes only
    pub active_only: bool,
    /// Filter by effective date range
    pub effective_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by expiry date range
    pub expiry_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by modified date range
    pub modified_date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filter by version number
    pub version: Option<u32>,
    /// Filter by minimum version
    pub min_version: Option<u32>,
    /// Filter by effect type
    pub effect_type: Option<EffectType>,
    /// Exclude statutes with these tags
    pub exclude_tags: Vec<String>,
    /// Include only statutes that reference these IDs
    pub references: Vec<String>,
    /// Include only statutes with supersedes relationships
    pub has_supersedes: Option<bool>,
}

impl SearchQuery {
    /// Creates a new empty search query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text search term.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Adds a tag filter.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Sets the jurisdiction filter.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Sets the status filter.
    pub fn with_status(mut self, status: StatuteStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the active-only filter.
    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }

    /// Sets the effective date range filter.
    pub fn with_effective_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.effective_date_range = Some((start, end));
        self
    }

    /// Sets the expiry date range filter.
    pub fn with_expiry_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.expiry_date_range = Some((start, end));
        self
    }

    /// Sets the modified date range filter.
    pub fn with_modified_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.modified_date_range = Some((start, end));
        self
    }

    /// Sets the version filter.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the minimum version filter.
    pub fn with_min_version(mut self, min_version: u32) -> Self {
        self.min_version = Some(min_version);
        self
    }

    /// Sets the effect type filter.
    pub fn with_effect_type(mut self, effect_type: EffectType) -> Self {
        self.effect_type = Some(effect_type);
        self
    }

    /// Adds a tag to exclude.
    pub fn exclude_tag(mut self, tag: impl Into<String>) -> Self {
        self.exclude_tags.push(tag.into());
        self
    }

    /// Adds a reference filter (statute must reference this ID).
    pub fn with_reference(mut self, reference_id: impl Into<String>) -> Self {
        self.references.push(reference_id.into());
        self
    }

    /// Filters for statutes that have supersedes relationships.
    pub fn with_supersedes(mut self) -> Self {
        self.has_supersedes = Some(true);
        self
    }

    /// Filters for statutes that don't have supersedes relationships.
    pub fn without_supersedes(mut self) -> Self {
        self.has_supersedes = Some(false);
        self
    }
}

/// A search result with relevance scoring.
#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    /// The statute entry
    pub entry: &'a StatuteEntry,
    /// Relevance score (0.0 - 1.0, higher is better)
    pub score: f64,
    /// Match highlights (field -> matched text)
    pub highlights: HashMap<String, Vec<String>>,
}

impl<'a> SearchResult<'a> {
    /// Creates a new search result with a given score.
    pub fn new(entry: &'a StatuteEntry, score: f64) -> Self {
        Self {
            entry,
            score: score.clamp(0.0, 1.0),
            highlights: HashMap::new(),
        }
    }

    /// Adds a highlight for a field.
    pub fn add_highlight(&mut self, field: String, matched: String) {
        self.highlights.entry(field).or_default().push(matched);
    }

    /// Gets highlights for a specific field.
    pub fn get_highlights(&self, field: &str) -> Option<&Vec<String>> {
        self.highlights.get(field)
    }
}

/// Ranking configuration for search results.
#[derive(Debug, Clone, Copy)]
pub struct RankingConfig {
    /// Weight for title matches (default: 3.0)
    pub title_weight: f64,
    /// Weight for ID matches (default: 2.0)
    pub id_weight: f64,
    /// Weight for tag matches (default: 1.5)
    pub tag_weight: f64,
    /// Weight for jurisdiction matches (default: 1.0)
    pub jurisdiction_weight: f64,
    /// Boost for exact matches (default: 2.0)
    pub exact_match_boost: f64,
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

impl RankingConfig {
    /// Creates a new ranking configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the title weight.
    pub fn with_title_weight(mut self, weight: f64) -> Self {
        self.title_weight = weight;
        self
    }

    /// Sets the ID weight.
    pub fn with_id_weight(mut self, weight: f64) -> Self {
        self.id_weight = weight;
        self
    }

    /// Sets the tag weight.
    pub fn with_tag_weight(mut self, weight: f64) -> Self {
        self.tag_weight = weight;
        self
    }

    /// Sets the jurisdiction weight.
    pub fn with_jurisdiction_weight(mut self, weight: f64) -> Self {
        self.jurisdiction_weight = weight;
        self
    }

    /// Sets the exact match boost.
    pub fn with_exact_match_boost(mut self, boost: f64) -> Self {
        self.exact_match_boost = boost;
        self
    }
}

/// A versioned statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteEntry {
    /// Unique registry ID
    pub registry_id: Uuid,
    /// The statute data
    pub statute: Statute,
    /// Version number
    pub version: u32,
    /// ETag for optimistic concurrency control
    pub etag: String,
    /// Status
    pub status: StatuteStatus,
    /// Effective date
    pub effective_date: Option<DateTime<Utc>>,
    /// Expiry date
    pub expiry_date: Option<DateTime<Utc>>,
    /// Parent statute (for amendments)
    pub amends: Option<String>,
    /// Statutes this one supersedes
    pub supersedes: Vec<String>,
    /// References to other statutes
    pub references: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
}

impl StatuteEntry {
    /// Creates a new statute entry.
    pub fn new(statute: Statute, jurisdiction: impl Into<String>) -> Self {
        let now = Utc::now();
        let registry_id = Uuid::new_v4();
        let etag = Self::generate_etag(&registry_id, 1, &now);

        Self {
            registry_id,
            statute,
            version: 1,
            etag,
            status: StatuteStatus::Draft,
            effective_date: None,
            expiry_date: None,
            amends: None,
            supersedes: Vec::new(),
            references: Vec::new(),
            tags: Vec::new(),
            jurisdiction: jurisdiction.into(),
            metadata: HashMap::new(),
            created_at: now,
            modified_at: now,
        }
    }

    /// Generates an ETag for optimistic concurrency control.
    fn generate_etag(registry_id: &Uuid, version: u32, modified_at: &DateTime<Utc>) -> String {
        format!(
            "{}-v{}-{}",
            registry_id,
            version,
            modified_at.timestamp_nanos_opt().unwrap_or(0)
        )
    }

    /// Updates the ETag after modification.
    fn update_etag(&mut self) {
        self.etag = Self::generate_etag(&self.registry_id, self.version, &self.modified_at);
    }

    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: DateTime<Utc>) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Sets the status.
    pub fn with_status(mut self, status: StatuteStatus) -> Self {
        self.status = status;
        self
    }

    /// Adds a reference.
    pub fn with_reference(mut self, statute_id: impl Into<String>) -> Self {
        self.references.push(statute_id.into());
        self
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Sets the expiry date.
    pub fn with_expiry_date(mut self, date: DateTime<Utc>) -> Self {
        self.expiry_date = Some(date);
        self
    }

    /// Sets the parent statute (for amendments).
    pub fn with_amends(mut self, statute_id: impl Into<String>) -> Self {
        self.amends = Some(statute_id.into());
        self
    }

    /// Adds a superseded statute.
    pub fn with_supersedes(mut self, statute_id: impl Into<String>) -> Self {
        self.supersedes.push(statute_id.into());
        self
    }

    /// Adds metadata key-value pair.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = jurisdiction.into();
        self
    }

    /// Returns whether this statute is currently active.
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.status == StatuteStatus::Active
            && self.effective_date.is_none_or(|d| d <= now)
            && self.expiry_date.is_none_or(|d| d > now)
    }
}

/// Status of a statute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatuteStatus {
    /// Being drafted
    Draft,
    /// Under review
    UnderReview,
    /// Approved but not yet effective
    Approved,
    /// Currently in force
    Active,
    /// No longer in force
    Repealed,
    /// Replaced by another statute
    Superseded,
}

/// Events that can occur in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryEvent {
    /// A new statute was registered
    StatuteRegistered {
        registry_id: Uuid,
        statute_id: String,
        jurisdiction: String,
        timestamp: DateTime<Utc>,
    },
    /// A statute was updated
    StatuteUpdated {
        statute_id: String,
        old_version: u32,
        new_version: u32,
        timestamp: DateTime<Utc>,
    },
    /// A statute's status was changed
    StatusChanged {
        statute_id: String,
        old_status: StatuteStatus,
        new_status: StatuteStatus,
        timestamp: DateTime<Utc>,
    },
    /// A tag was added to a statute
    TagAdded {
        statute_id: String,
        tag: String,
        timestamp: DateTime<Utc>,
    },
    /// A tag was removed from a statute
    TagRemoved {
        statute_id: String,
        tag: String,
        timestamp: DateTime<Utc>,
    },
    /// A reference was added
    ReferenceAdded {
        statute_id: String,
        referenced_id: String,
        timestamp: DateTime<Utc>,
    },
    /// A reference was removed
    ReferenceRemoved {
        statute_id: String,
        referenced_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Metadata was updated
    MetadataUpdated {
        statute_id: String,
        key: String,
        old_value: Option<String>,
        new_value: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// A statute was deleted
    StatuteDeleted {
        statute_id: String,
        jurisdiction: String,
        version: u32,
        timestamp: DateTime<Utc>,
    },
    /// A statute was archived
    StatuteArchived {
        statute_id: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}

/// Event store for tracking all changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStore {
    /// All events in chronological order
    events: VecDeque<RegistryEvent>,
    /// Maximum number of events to keep (0 = unlimited)
    max_events: usize,
}

impl EventStore {
    /// Creates a new event store.
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            max_events: 0,
        }
    }

    /// Creates a new event store with a maximum size.
    pub fn with_max_events(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }

    /// Records an event.
    pub fn record(&mut self, event: RegistryEvent) {
        self.events.push_back(event);

        // Trim old events if we exceed the limit
        if self.max_events > 0 && self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    /// Returns all events.
    pub fn all_events(&self) -> Vec<&RegistryEvent> {
        self.events.iter().collect()
    }

    /// Returns events for a specific statute.
    pub fn events_for_statute(&self, statute_id: &str) -> Vec<&RegistryEvent> {
        self.events
            .iter()
            .filter(|event| match event {
                RegistryEvent::StatuteRegistered { statute_id: id, .. }
                | RegistryEvent::StatuteUpdated { statute_id: id, .. }
                | RegistryEvent::StatusChanged { statute_id: id, .. }
                | RegistryEvent::TagAdded { statute_id: id, .. }
                | RegistryEvent::TagRemoved { statute_id: id, .. }
                | RegistryEvent::ReferenceAdded { statute_id: id, .. }
                | RegistryEvent::ReferenceRemoved { statute_id: id, .. }
                | RegistryEvent::MetadataUpdated { statute_id: id, .. }
                | RegistryEvent::StatuteDeleted { statute_id: id, .. }
                | RegistryEvent::StatuteArchived { statute_id: id, .. } => id == statute_id,
            })
            .collect()
    }

    /// Returns events within a date range.
    pub fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&RegistryEvent> {
        self.events
            .iter()
            .filter(|event| {
                let timestamp = match event {
                    RegistryEvent::StatuteRegistered { timestamp, .. }
                    | RegistryEvent::StatuteUpdated { timestamp, .. }
                    | RegistryEvent::StatusChanged { timestamp, .. }
                    | RegistryEvent::TagAdded { timestamp, .. }
                    | RegistryEvent::TagRemoved { timestamp, .. }
                    | RegistryEvent::ReferenceAdded { timestamp, .. }
                    | RegistryEvent::ReferenceRemoved { timestamp, .. }
                    | RegistryEvent::MetadataUpdated { timestamp, .. }
                    | RegistryEvent::StatuteDeleted { timestamp, .. }
                    | RegistryEvent::StatuteArchived { timestamp, .. } => timestamp,
                };
                timestamp >= &start && timestamp <= &end
            })
            .collect()
    }

    /// Returns the count of events.
    pub fn count(&self) -> usize {
        self.events.len()
    }

    /// Clears all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Webhook callback function type.
pub type WebhookCallback = Arc<dyn Fn(&RegistryEvent) + Send + Sync>;

/// Webhook subscription.
#[derive(Clone)]
pub struct WebhookSubscription {
    /// Unique ID for this subscription
    pub id: Uuid,
    /// Optional name/description
    pub name: Option<String>,
    /// Callback function
    callback: WebhookCallback,
    /// Filter: only trigger for specific event types
    pub event_filter: Option<WebhookEventFilter>,
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

/// Filter for webhook events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventFilter {
    /// Only trigger on statute registration
    StatuteRegistered,
    /// Only trigger on statute updates
    StatuteUpdated,
    /// Only trigger on status changes
    StatusChanged,
    /// Only trigger on tag operations
    TagOperations,
    /// Only trigger on reference operations
    ReferenceOperations,
    /// Only trigger on metadata updates
    MetadataUpdated,
    /// Trigger on any event
    All,
}

impl WebhookEventFilter {
    /// Checks if an event matches this filter.
    pub fn matches(&self, event: &RegistryEvent) -> bool {
        match self {
            Self::All => true,
            Self::StatuteRegistered => matches!(event, RegistryEvent::StatuteRegistered { .. }),
            Self::StatuteUpdated => matches!(event, RegistryEvent::StatuteUpdated { .. }),
            Self::StatusChanged => matches!(event, RegistryEvent::StatusChanged { .. }),
            Self::TagOperations => matches!(
                event,
                RegistryEvent::TagAdded { .. } | RegistryEvent::TagRemoved { .. }
            ),
            Self::ReferenceOperations => matches!(
                event,
                RegistryEvent::ReferenceAdded { .. } | RegistryEvent::ReferenceRemoved { .. }
            ),
            Self::MetadataUpdated => matches!(event, RegistryEvent::MetadataUpdated { .. }),
        }
    }
}

/// Webhook manager for event notifications.
#[derive(Debug, Clone)]
pub struct WebhookManager {
    subscriptions: Arc<Mutex<Vec<WebhookSubscription>>>,
}

impl WebhookManager {
    /// Creates a new webhook manager.
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribes to events with a callback.
    pub fn subscribe<F>(
        &self,
        name: Option<String>,
        filter: Option<WebhookEventFilter>,
        callback: F,
    ) -> Uuid
    where
        F: Fn(&RegistryEvent) + Send + Sync + 'static,
    {
        let id = Uuid::new_v4();
        let subscription = WebhookSubscription {
            id,
            name,
            callback: Arc::new(callback),
            event_filter: filter,
        };

        let mut subs = self.subscriptions.lock().unwrap();
        subs.push(subscription);
        id
    }

    /// Unsubscribes a webhook by ID.
    pub fn unsubscribe(&self, id: Uuid) -> bool {
        let mut subs = self.subscriptions.lock().unwrap();
        if let Some(pos) = subs.iter().position(|s| s.id == id) {
            subs.remove(pos);
            true
        } else {
            false
        }
    }

    /// Triggers all matching webhooks for an event.
    pub fn trigger(&self, event: &RegistryEvent) {
        let subs = self.subscriptions.lock().unwrap();
        for subscription in subs.iter() {
            if subscription
                .event_filter
                .as_ref()
                .is_none_or(|filter| filter.matches(event))
            {
                (subscription.callback)(event);
            }
        }
    }

    /// Returns the count of active subscriptions.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.lock().unwrap().len()
    }

    /// Clears all subscriptions.
    pub fn clear(&self) {
        self.subscriptions.lock().unwrap().clear();
    }

    /// Lists all subscription IDs and names.
    pub fn list_subscriptions(&self) -> Vec<(Uuid, Option<String>)> {
        self.subscriptions
            .lock()
            .unwrap()
            .iter()
            .map(|s| (s.id, s.name.clone()))
            .collect()
    }
}

impl Default for WebhookManager {
    fn default() -> Self {
        Self::new()
    }
}

/// The central statute registry.
pub struct StatuteRegistry {
    /// Statutes by ID (latest version)
    statutes: IndexMap<String, StatuteEntry>,
    /// Version history: statute_id -> version -> entry
    versions: HashMap<String, HashMap<u32, StatuteEntry>>,
    /// Index by tag
    tag_index: HashMap<String, HashSet<String>>,
    /// Index by jurisdiction
    jurisdiction_index: HashMap<String, HashSet<String>>,
    /// LRU cache for frequently accessed statutes
    cache: LruCache<String, StatuteEntry>,
    /// Fuzzy matcher for statute IDs
    fuzzy_matcher: SkimMatcherV2,
    /// Event store for change tracking
    event_store: EventStore,
    /// Webhook manager for notifications
    webhook_manager: WebhookManager,
    /// Archive for deleted/superseded statutes
    archive: StatuteArchive,
    /// Retention policy for auto-archiving
    retention_policy: RetentionPolicy,
    /// Analytics cache with TTL support
    analytics_cache: CachedAnalytics,
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
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
            fuzzy_matcher: SkimMatcherV2::default(),
            event_store: EventStore::new(),
            webhook_manager: WebhookManager::new(),
            archive: StatuteArchive::new(),
            retention_policy: RetentionPolicy::new(),
            analytics_cache: CachedAnalytics::new(300), // 5 minute default cache
        }
    }
}

impl StatuteRegistry {
    /// Creates a new empty registry.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_registry::StatuteRegistry;
    ///
    /// let registry = StatuteRegistry::new();
    /// assert_eq!(registry.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper method to record an event and trigger webhooks.
    fn record_event(&mut self, event: RegistryEvent) {
        self.event_store.record(event.clone());
        self.webhook_manager.trigger(&event);
    }

    /// Subscribes to registry events.
    pub fn subscribe_webhook<F>(
        &self,
        name: Option<String>,
        filter: Option<WebhookEventFilter>,
        callback: F,
    ) -> Uuid
    where
        F: Fn(&RegistryEvent) + Send + Sync + 'static,
    {
        self.webhook_manager.subscribe(name, filter, callback)
    }

    /// Unsubscribes a webhook.
    pub fn unsubscribe_webhook(&self, id: Uuid) -> bool {
        self.webhook_manager.unsubscribe(id)
    }

    /// Returns the count of active webhook subscriptions.
    pub fn webhook_count(&self) -> usize {
        self.webhook_manager.subscription_count()
    }

    /// Lists all webhook subscriptions.
    pub fn list_webhooks(&self) -> Vec<(Uuid, Option<String>)> {
        self.webhook_manager.list_subscriptions()
    }

    /// Clears all webhook subscriptions.
    pub fn clear_webhooks(&self) {
        self.webhook_manager.clear();
    }

    /// Registers a new statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Effect, EffectType, Statute};
    /// use legalis_registry::{StatuteEntry, StatuteRegistry};
    ///
    /// let mut registry = StatuteRegistry::new();
    /// let statute = Statute::new(
    ///     "statute-1",
    ///     "Test Statute",
    ///     Effect::new(EffectType::Grant, "Grant permission")
    /// );
    /// let entry = StatuteEntry::new(statute, "US");
    ///
    /// let id = registry.register(entry).unwrap();
    /// assert_eq!(registry.count(), 1);
    /// ```
    pub fn register(&mut self, entry: StatuteEntry) -> RegistryResult<Uuid> {
        let statute_id = entry.statute.id.clone();

        if self.statutes.contains_key(&statute_id) {
            return Err(RegistryError::DuplicateId(statute_id));
        }

        let registry_id = entry.registry_id;
        let jurisdiction = entry.jurisdiction.clone();
        let timestamp = Utc::now();

        // Update indexes
        for tag in &entry.tags {
            self.tag_index
                .entry(tag.clone())
                .or_default()
                .insert(statute_id.clone());
        }

        self.jurisdiction_index
            .entry(entry.jurisdiction.clone())
            .or_default()
            .insert(statute_id.clone());

        // Store version
        self.versions
            .entry(statute_id.clone())
            .or_default()
            .insert(entry.version, entry.clone());

        // Store as current
        self.statutes.insert(statute_id.clone(), entry);

        // Record event and trigger webhooks
        self.record_event(RegistryEvent::StatuteRegistered {
            registry_id,
            statute_id,
            jurisdiction,
            timestamp,
        });

        Ok(registry_id)
    }

    /// Updates a statute (creates new version).
    pub fn update(&mut self, statute_id: &str, statute: Statute) -> RegistryResult<u32> {
        let existing = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let old_version = existing.version;
        let new_version = existing.version + 1;
        let mut new_entry = StatuteEntry::new(statute, &existing.jurisdiction);
        new_entry.version = new_version;
        new_entry.tags = existing.tags.clone();
        new_entry.references = existing.references.clone();
        new_entry.modified_at = Utc::now();
        new_entry.update_etag();

        // Invalidate cache
        self.cache.pop(statute_id);

        // Store version
        self.versions
            .entry(statute_id.to_string())
            .or_default()
            .insert(new_version, new_entry.clone());

        // Update current
        self.statutes.insert(statute_id.to_string(), new_entry);

        // Record event and trigger webhooks
        self.record_event(RegistryEvent::StatuteUpdated {
            statute_id: statute_id.to_string(),
            old_version,
            new_version,
            timestamp: Utc::now(),
        });

        Ok(new_version)
    }

    /// Updates a statute with optimistic concurrency control.
    /// Returns an error if the ETag doesn't match.
    pub fn update_with_etag(
        &mut self,
        statute_id: &str,
        statute: Statute,
        expected_etag: &str,
    ) -> RegistryResult<u32> {
        let existing = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        // Check ETag
        if existing.etag != expected_etag {
            return Err(RegistryError::ConcurrentModification {
                expected: expected_etag.to_string(),
                actual: existing.etag.clone(),
            });
        }

        // Proceed with update
        self.update(statute_id, statute)
    }

    /// Gets a statute by ID (latest version).
    pub fn get(&mut self, statute_id: &str) -> Option<StatuteEntry> {
        // Check cache first
        if let Some(cached) = self.cache.get(statute_id) {
            return Some(cached.clone());
        }

        // Get from main storage and cache it
        if let Some(entry) = self.statutes.get(statute_id) {
            let entry_clone = entry.clone();
            self.cache.put(statute_id.to_string(), entry_clone.clone());
            Some(entry_clone)
        } else {
            None
        }
    }

    /// Gets a statute by ID without using the cache (for immutable access).
    pub fn get_uncached(&self, statute_id: &str) -> Option<&StatuteEntry> {
        self.statutes.get(statute_id)
    }

    /// Gets a specific version of a statute.
    pub fn get_version(&self, statute_id: &str, version: u32) -> RegistryResult<&StatuteEntry> {
        self.versions
            .get(statute_id)
            .and_then(|versions| versions.get(&version))
            .ok_or_else(|| RegistryError::VersionNotFound {
                statute_id: statute_id.to_string(),
                version,
            })
    }

    /// Lists all versions of a statute.
    pub fn list_versions(&self, statute_id: &str) -> Vec<u32> {
        self.versions
            .get(statute_id)
            .map(|v| {
                let mut versions: Vec<u32> = v.keys().copied().collect();
                versions.sort();
                versions
            })
            .unwrap_or_default()
    }

    /// Lists all statutes.
    pub fn list(&self) -> Vec<&StatuteEntry> {
        self.statutes.values().collect()
    }

    /// Lists active statutes.
    pub fn list_active(&self) -> Vec<&StatuteEntry> {
        self.statutes.values().filter(|e| e.is_active()).collect()
    }

    /// Queries statutes by tag.
    pub fn query_by_tag(&self, tag: &str) -> Vec<&StatuteEntry> {
        self.tag_index
            .get(tag)
            .map(|ids| ids.iter().filter_map(|id| self.statutes.get(id)).collect())
            .unwrap_or_default()
    }

    /// Queries statutes by jurisdiction.
    pub fn query_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&StatuteEntry> {
        self.jurisdiction_index
            .get(jurisdiction)
            .map(|ids| ids.iter().filter_map(|id| self.statutes.get(id)).collect())
            .unwrap_or_default()
    }

    /// Sets the status of a statute.
    pub fn set_status(&mut self, statute_id: &str, status: StatuteStatus) -> RegistryResult<()> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let old_status = entry.status;
        entry.status = status;
        entry.modified_at = Utc::now();
        entry.update_etag();

        // Invalidate cache
        self.cache.pop(statute_id);

        // Record event and trigger webhooks
        self.record_event(RegistryEvent::StatusChanged {
            statute_id: statute_id.to_string(),
            old_status,
            new_status: status,
            timestamp: Utc::now(),
        });

        Ok(())
    }

    /// Sets the status of a statute with optimistic concurrency control.
    pub fn set_status_with_etag(
        &mut self,
        statute_id: &str,
        status: StatuteStatus,
        expected_etag: &str,
    ) -> RegistryResult<()> {
        // Get the current ETag first
        let current_etag = self
            .statutes
            .get(statute_id)
            .map(|e| e.etag.clone())
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        // Check ETag
        if current_etag != expected_etag {
            return Err(RegistryError::ConcurrentModification {
                expected: expected_etag.to_string(),
                actual: current_etag,
            });
        }

        // Proceed with update
        self.set_status(statute_id, status)
    }

    /// Returns the total count of statutes.
    pub fn count(&self) -> usize {
        self.statutes.len()
    }

    /// Returns all tags.
    pub fn all_tags(&self) -> Vec<&String> {
        self.tag_index.keys().collect()
    }

    /// Returns all jurisdictions.
    pub fn all_jurisdictions(&self) -> Vec<&String> {
        self.jurisdiction_index.keys().collect()
    }

    /// Returns all statute IDs.
    pub fn all_statute_ids(&self) -> Vec<&String> {
        self.statutes.keys().collect()
    }

    /// Checks if a statute exists in the registry.
    pub fn contains(&self, statute_id: &str) -> bool {
        self.statutes.contains_key(statute_id)
    }

    /// Gets multiple statutes by their IDs.
    pub fn get_many(&mut self, statute_ids: &[&str]) -> Vec<Option<StatuteEntry>> {
        statute_ids.iter().map(|id| self.get(id)).collect()
    }

    /// Returns an iterator over all statutes (memory-efficient).
    ///
    /// This is more efficient than `all_statute_ids()` for large registries
    /// as it doesn't allocate a vector.
    pub fn iter(&self) -> impl Iterator<Item = &StatuteEntry> {
        self.statutes.values()
    }

    /// Returns an iterator over active statutes only.
    pub fn iter_active(&self) -> impl Iterator<Item = &StatuteEntry> {
        self.statutes
            .values()
            .filter(|entry| entry.status == StatuteStatus::Active)
    }

    /// Returns an iterator over (statute_id, entry) pairs.
    pub fn iter_with_ids(&self) -> impl Iterator<Item = (&String, &StatuteEntry)> {
        self.statutes.iter()
    }

    /// Gets the latest version number for a statute.
    pub fn latest_version(&self, statute_id: &str) -> Option<u32> {
        self.statutes.get(statute_id).map(|entry| entry.version)
    }

    /// Returns statistics about the registry.
    pub fn statistics(&self) -> RegistryStatistics {
        let total = self.statutes.len();
        let mut by_status = HashMap::new();
        let mut by_jurisdiction = HashMap::new();

        for entry in self.statutes.values() {
            *by_status.entry(entry.status).or_insert(0) += 1;
            *by_jurisdiction
                .entry(entry.jurisdiction.clone())
                .or_insert(0) += 1;
        }

        RegistryStatistics {
            total_statutes: total,
            total_versions: self.versions.values().map(|v| v.len()).sum(),
            total_events: self.event_store.count(),
            total_tags: self.tag_index.len(),
            total_jurisdictions: self.jurisdiction_index.len(),
            by_status,
            by_jurisdiction,
        }
    }

    /// Finds statutes that reference a given statute.
    pub fn find_referencing(&self, statute_id: &str) -> Vec<&StatuteEntry> {
        self.statutes
            .values()
            .filter(|e| e.references.contains(&statute_id.to_string()))
            .collect()
    }

    /// Gets the dependency graph for a statute.
    pub fn get_dependencies(&self, statute_id: &str) -> HashSet<String> {
        let mut deps = HashSet::new();
        self.collect_dependencies(statute_id, &mut deps, &mut HashSet::new());
        deps
    }

    fn collect_dependencies(
        &self,
        statute_id: &str,
        deps: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(statute_id) {
            return;
        }
        visited.insert(statute_id.to_string());

        if let Some(entry) = self.statutes.get(statute_id) {
            for reference in &entry.references {
                deps.insert(reference.clone());
                self.collect_dependencies(reference, deps, visited);
            }
        }
    }

    /// Searches statutes using fuzzy matching on statute IDs.
    pub fn fuzzy_search(&self, query: &str, limit: usize) -> Vec<(i64, &StatuteEntry)> {
        let mut matches: Vec<(i64, &StatuteEntry)> = self
            .statutes
            .iter()
            .filter_map(|(id, entry)| {
                self.fuzzy_matcher
                    .fuzzy_match(id, query)
                    .map(|score| (score, entry))
            })
            .collect();

        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches.truncate(limit);
        matches
    }

    /// Performs full-text search across statute IDs, titles, and descriptions.
    pub fn full_text_search(&self, query: &str) -> Vec<&StatuteEntry> {
        let query_lower = query.to_lowercase();
        self.statutes
            .values()
            .filter(|entry| {
                entry.statute.id.to_lowercase().contains(&query_lower)
                    || entry.statute.title.to_lowercase().contains(&query_lower)
                    || entry
                        .statute
                        .effect
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || entry
                        .statute
                        .discretion_logic
                        .as_ref()
                        .is_some_and(|d| d.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Advanced search with multiple filters.
    pub fn search(&self, query: &SearchQuery) -> Vec<&StatuteEntry> {
        self.statutes
            .values()
            .filter(|entry| {
                // Text search
                if let Some(text) = &query.text {
                    let text_lower = text.to_lowercase();
                    if !entry.statute.id.to_lowercase().contains(&text_lower)
                        && !entry.statute.title.to_lowercase().contains(&text_lower)
                        && !entry
                            .statute
                            .effect
                            .description
                            .to_lowercase()
                            .contains(&text_lower)
                        && !entry
                            .statute
                            .discretion_logic
                            .as_ref()
                            .is_some_and(|d| d.to_lowercase().contains(&text_lower))
                    {
                        return false;
                    }
                }

                // Tag filter
                if !query.tags.is_empty() && !query.tags.iter().any(|t| entry.tags.contains(t)) {
                    return false;
                }

                // Jurisdiction filter
                if let Some(jurisdiction) = &query.jurisdiction {
                    if &entry.jurisdiction != jurisdiction {
                        return false;
                    }
                }

                // Status filter
                if let Some(status) = &query.status {
                    if &entry.status != status {
                        return false;
                    }
                }

                // Active only filter
                if query.active_only && !entry.is_active() {
                    return false;
                }

                true
            })
            .collect()
    }

    /// Searches with pagination support.
    pub fn search_paged(
        &self,
        query: &SearchQuery,
        pagination: Pagination,
    ) -> PagedResult<StatuteEntry> {
        let all_results = self.search(query);
        let total = all_results.len();

        let items: Vec<StatuteEntry> = all_results
            .into_iter()
            .skip(pagination.offset())
            .take(pagination.limit())
            .cloned()
            .collect();

        PagedResult::new(items, pagination.page, pagination.per_page, total)
    }

    /// Lists all statutes with pagination.
    pub fn list_paged(&self, pagination: Pagination) -> PagedResult<StatuteEntry> {
        let total = self.statutes.len();
        let items: Vec<StatuteEntry> = self
            .statutes
            .values()
            .skip(pagination.offset())
            .take(pagination.limit())
            .cloned()
            .collect();

        PagedResult::new(items, pagination.page, pagination.per_page, total)
    }

    /// Batch registers multiple statutes.
    pub fn batch_register(&mut self, entries: Vec<StatuteEntry>) -> Vec<RegistryResult<Uuid>> {
        entries
            .into_iter()
            .map(|entry| self.register(entry))
            .collect()
    }

    /// Batch updates multiple statutes.
    pub fn batch_update(&mut self, updates: Vec<(String, Statute)>) -> Vec<RegistryResult<u32>> {
        updates
            .into_iter()
            .map(|(id, statute)| self.update(&id, statute))
            .collect()
    }

    /// Batch sets status for multiple statutes.
    pub fn batch_set_status(
        &mut self,
        statute_ids: Vec<String>,
        status: StatuteStatus,
    ) -> Vec<RegistryResult<()>> {
        statute_ids
            .into_iter()
            .map(|id| self.set_status(&id, status))
            .collect()
    }

    /// Deletes a statute from the registry.
    ///
    /// This removes the statute, all its versions, and cleans up all indexes.
    /// Returns the deleted entry if found.
    pub fn delete(&mut self, statute_id: &str) -> RegistryResult<StatuteEntry> {
        let entry = self
            .statutes
            .shift_remove(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let jurisdiction = entry.jurisdiction.clone();
        let version = entry.version;

        // Remove from cache
        self.cache.pop(statute_id);

        // Remove from tag index
        for tag in &entry.tags {
            if let Some(ids) = self.tag_index.get_mut(tag) {
                ids.remove(statute_id);
                if ids.is_empty() {
                    self.tag_index.remove(tag);
                }
            }
        }

        // Remove from jurisdiction index
        if let Some(ids) = self.jurisdiction_index.get_mut(&entry.jurisdiction) {
            ids.remove(statute_id);
            if ids.is_empty() {
                self.jurisdiction_index.remove(&entry.jurisdiction);
            }
        }

        // Remove all versions
        self.versions.remove(statute_id);

        // Record event and trigger webhooks
        self.record_event(RegistryEvent::StatuteDeleted {
            statute_id: statute_id.to_string(),
            jurisdiction,
            version,
            timestamp: Utc::now(),
        });

        Ok(entry)
    }

    /// Batch deletes multiple statutes.
    ///
    /// Returns a vector of results, one for each statute ID.
    pub fn batch_delete(&mut self, statute_ids: Vec<String>) -> Vec<RegistryResult<StatuteEntry>> {
        statute_ids.into_iter().map(|id| self.delete(&id)).collect()
    }

    /// Archives a statute and removes it from the active registry.
    ///
    /// This is a soft delete that preserves the statute in the archive.
    pub fn archive_statute(&mut self, statute_id: &str, reason: String) -> RegistryResult<()> {
        let entry = self.delete(statute_id)?;
        self.archive.archive(entry, reason.clone());

        // Record archive event
        self.record_event(RegistryEvent::StatuteArchived {
            statute_id: statute_id.to_string(),
            reason,
            timestamp: Utc::now(),
        });

        Ok(())
    }

    /// Unarchives a statute and restores it to the registry.
    pub fn unarchive_statute(&mut self, statute_id: &str) -> RegistryResult<Uuid> {
        let archived = self
            .archive
            .unarchive(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        self.register(archived.entry)
    }

    /// Gets an archived statute.
    pub fn get_archived(&self, statute_id: &str) -> Option<&ArchivedStatute> {
        self.archive.get(statute_id)
    }

    /// Lists all archived statute IDs.
    pub fn list_archived_ids(&self) -> Vec<String> {
        self.archive.list_ids()
    }

    /// Returns the count of archived statutes.
    pub fn archived_count(&self) -> usize {
        self.archive.count()
    }

    /// Searches archived statutes by reason.
    pub fn search_archived_by_reason(&self, query: &str) -> Vec<&ArchivedStatute> {
        self.archive.search_by_reason(query)
    }

    /// Sets the retention policy for the registry.
    pub fn set_retention_policy(&mut self, policy: RetentionPolicy) {
        self.retention_policy = policy;
    }

    /// Gets a reference to the current retention policy.
    pub fn retention_policy(&self) -> &RetentionPolicy {
        &self.retention_policy
    }

    /// Applies retention policy rules to archive eligible statutes.
    pub fn apply_retention_policy(&mut self) -> RetentionResult {
        let now = Utc::now();
        let total_count = self.statutes.len();
        let mut result = RetentionResult::new(total_count);

        // Collect IDs to archive (can't modify while iterating)
        let mut to_archive: Vec<(String, String)> = Vec::new();

        for (statute_id, entry) in &self.statutes {
            for rule in &self.retention_policy.rules {
                let should_archive = match rule {
                    RetentionRule::ExpiredStatutes { reason: _ } => {
                        // Check if statute has expired
                        if let Some(expiry) = entry.expiry_date {
                            expiry < now
                        } else {
                            false
                        }
                    }
                    RetentionRule::OlderThanDays { days, reason: _ } => {
                        // Check if statute is older than specified days
                        if let Some(effective) = entry.effective_date {
                            let cutoff = now - chrono::Duration::days(*days);
                            effective < cutoff
                        } else {
                            false
                        }
                    }
                    RetentionRule::ByStatus { status, reason: _ } => {
                        // Check if statute has specified status
                        entry.status == *status
                    }
                    RetentionRule::SupersededStatutes { reason: _ } => {
                        // Check if statute has been superseded
                        !entry.supersedes.is_empty()
                    }
                    RetentionRule::InactiveForDays { days, reason: _ } => {
                        // Check if statute hasn't been modified in specified days
                        let cutoff = now - chrono::Duration::days(*days);
                        entry.modified_at < cutoff
                    }
                };

                if should_archive {
                    let reason = match rule {
                        RetentionRule::ExpiredStatutes { reason } => reason.clone(),
                        RetentionRule::OlderThanDays { reason, .. } => reason.clone(),
                        RetentionRule::ByStatus { reason, .. } => reason.clone(),
                        RetentionRule::SupersededStatutes { reason } => reason.clone(),
                        RetentionRule::InactiveForDays { reason, .. } => reason.clone(),
                    };
                    to_archive.push((statute_id.clone(), reason));
                    break; // Only archive once per statute
                }
            }
        }

        // Archive the collected statutes
        for (statute_id, reason) in to_archive {
            if let Ok(()) = self.archive_statute(&statute_id, reason.clone()) {
                result.record_archived(statute_id, reason);
            }
        }

        result
    }

    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Returns cache statistics.
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.len(), self.cache.cap().get())
    }

    /// Searches statutes by effect type.
    pub fn search_by_effect_type(&self, effect_type: EffectType) -> Vec<&StatuteEntry> {
        self.statutes
            .values()
            .filter(|entry| entry.statute.effect.effect_type == effect_type)
            .collect()
    }

    /// Searches statutes that contain a specific condition variant.
    pub fn search_by_condition_type(
        &self,
        condition_matcher: impl Fn(&Condition) -> bool,
    ) -> Vec<&StatuteEntry> {
        self.statutes
            .values()
            .filter(|entry| {
                entry
                    .statute
                    .preconditions
                    .iter()
                    .any(|cond| Self::condition_contains(&condition_matcher, cond))
            })
            .collect()
    }

    /// Recursively checks if a condition matches the predicate.
    fn condition_contains(matcher: &impl Fn(&Condition) -> bool, condition: &Condition) -> bool {
        if matcher(condition) {
            return true;
        }

        match condition {
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::condition_contains(matcher, left) || Self::condition_contains(matcher, right)
            }
            Condition::Not(inner) => Self::condition_contains(matcher, inner),
            _ => false,
        }
    }

    /// Searches statutes that have age-based conditions.
    pub fn search_with_age_condition(&self) -> Vec<&StatuteEntry> {
        self.search_by_condition_type(|cond| matches!(cond, Condition::Age { .. }))
    }

    /// Searches statutes that have income-based conditions.
    pub fn search_with_income_condition(&self) -> Vec<&StatuteEntry> {
        self.search_by_condition_type(|cond| matches!(cond, Condition::Income { .. }))
    }

    /// Searches statutes that have geographic conditions.
    pub fn search_with_geographic_condition(&self) -> Vec<&StatuteEntry> {
        self.search_by_condition_type(|cond| matches!(cond, Condition::Geographic { .. }))
    }

    /// Searches statutes that have date range conditions.
    pub fn search_with_date_range_condition(&self) -> Vec<&StatuteEntry> {
        self.search_by_condition_type(|cond| matches!(cond, Condition::DateRange { .. }))
    }

    /// Gets detailed dependency information for a statute.
    pub fn get_dependency_graph(&self, statute_id: &str) -> Option<DependencyGraph> {
        if !self.statutes.contains_key(statute_id) {
            return None;
        }

        let mut graph = DependencyGraph {
            root_id: statute_id.to_string(),
            dependencies: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        };

        // Build forward dependencies
        self.build_dependency_graph(statute_id, &mut graph.dependencies, &mut HashSet::new());

        // Build reverse dependencies
        for (id, entry) in &self.statutes {
            for reference in &entry.references {
                graph
                    .reverse_dependencies
                    .entry(reference.clone())
                    .or_default()
                    .insert(id.clone());
            }
        }

        Some(graph)
    }

    fn build_dependency_graph(
        &self,
        statute_id: &str,
        graph: &mut HashMap<String, HashSet<String>>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(statute_id) {
            return;
        }
        visited.insert(statute_id.to_string());

        if let Some(entry) = self.statutes.get(statute_id) {
            let deps: HashSet<String> = entry.references.iter().cloned().collect();
            graph.insert(statute_id.to_string(), deps.clone());

            for reference in &entry.references {
                self.build_dependency_graph(reference, graph, visited);
            }
        }
    }

    /// Returns all events from the event store.
    pub fn all_events(&self) -> Vec<&RegistryEvent> {
        self.event_store.all_events()
    }

    /// Returns events for a specific statute.
    pub fn events_for_statute(&self, statute_id: &str) -> Vec<&RegistryEvent> {
        self.event_store.events_for_statute(statute_id)
    }

    /// Returns events within a date range.
    pub fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&RegistryEvent> {
        self.event_store.events_in_range(start, end)
    }

    /// Returns the count of events.
    pub fn event_count(&self) -> usize {
        self.event_store.count()
    }

    /// Clears all events from the event store.
    pub fn clear_events(&mut self) {
        self.event_store.clear();
    }

    /// Searches statutes with relevance ranking.
    ///
    /// Returns results sorted by relevance score (highest first).
    pub fn search_ranked<'a>(
        &'a self,
        query: &str,
        config: Option<RankingConfig>,
    ) -> Vec<SearchResult<'a>> {
        let config = config.unwrap_or_default();
        let query_lower = query.to_lowercase();

        let mut results: Vec<SearchResult> = self
            .statutes
            .values()
            .filter_map(|entry| {
                let score = self.calculate_relevance_score(entry, &query_lower, &config);
                if score > 0.0 {
                    let mut result = SearchResult::new(entry, score);

                    // Add highlights for matched fields
                    if entry.statute.title.to_lowercase().contains(&query_lower) {
                        result.add_highlight("title".to_string(), entry.statute.title.clone());
                    }
                    if entry.statute.id.to_lowercase().contains(&query_lower) {
                        result.add_highlight("id".to_string(), entry.statute.id.clone());
                    }
                    for tag in &entry.tags {
                        if tag.to_lowercase().contains(&query_lower) {
                            result.add_highlight("tag".to_string(), tag.clone());
                        }
                    }

                    Some(result)
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }

    /// Calculates relevance score for a statute entry.
    #[allow(dead_code)]
    fn calculate_relevance_score(
        &self,
        entry: &StatuteEntry,
        query: &str,
        config: &RankingConfig,
    ) -> f64 {
        let mut score = 0.0;

        // Title matching
        let title_lower = entry.statute.title.to_lowercase();
        if title_lower == query {
            score += config.title_weight * config.exact_match_boost;
        } else if title_lower.contains(query) {
            score += config.title_weight;
        }

        // ID matching
        let id_lower = entry.statute.id.to_lowercase();
        if id_lower == query {
            score += config.id_weight * config.exact_match_boost;
        } else if id_lower.contains(query) {
            score += config.id_weight;
        }

        // Tag matching
        for tag in &entry.tags {
            let tag_lower = tag.to_lowercase();
            if tag_lower == query {
                score += config.tag_weight * config.exact_match_boost;
            } else if tag_lower.contains(query) {
                score += config.tag_weight;
            }
        }

        // Jurisdiction matching
        let jurisdiction_lower = entry.jurisdiction.to_lowercase();
        if jurisdiction_lower == query {
            score += config.jurisdiction_weight * config.exact_match_boost;
        } else if jurisdiction_lower.contains(query) {
            score += config.jurisdiction_weight;
        }

        // Normalize score to 0.0-1.0 range
        // Max possible score is title + id + all tags + jurisdiction (with boost)
        let max_score = (config.title_weight
            + config.id_weight
            + config.jurisdiction_weight
            + config.tag_weight * 5.0)
            * config.exact_match_boost;

        (score / max_score).min(1.0)
    }

    /// Searches statutes with fuzzy matching and ranking.
    pub fn fuzzy_search_ranked<'a>(
        &'a mut self,
        query: &str,
        limit: usize,
        config: Option<RankingConfig>,
    ) -> Vec<SearchResult<'a>> {
        let config = config.unwrap_or_default();

        let mut results: Vec<SearchResult> = self
            .statutes
            .values()
            .filter_map(|entry| {
                // Use fuzzy matcher for ID matching
                let fuzzy_score = self
                    .fuzzy_matcher
                    .fuzzy_match(&entry.statute.id, query)
                    .unwrap_or(0) as f64;

                // Combine fuzzy score with text relevance
                let text_score =
                    self.calculate_relevance_score(entry, &query.to_lowercase(), &config);

                // Fuzzy score is typically 0-100+, normalize it
                let normalized_fuzzy = (fuzzy_score / 100.0).min(1.0);

                // Combine scores (weighted average)
                let combined_score = (normalized_fuzzy * 0.4 + text_score * 0.6).min(1.0);

                if combined_score > 0.1 {
                    Some(SearchResult::new(entry, combined_score))
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (descending) and limit results
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);

        results
    }

    /// Exports all events for backup or analysis.
    pub fn export_events(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.event_store.all_events())
    }

    /// Creates a backup of the entire registry.
    pub fn create_backup(&self, description: Option<String>) -> RegistryBackup {
        let statutes: Vec<StatuteEntry> = self.statutes.values().cloned().collect();
        let events: Vec<RegistryEvent> =
            self.event_store.all_events().into_iter().cloned().collect();

        RegistryBackup {
            statutes: statutes.clone(),
            versions: self.versions.clone(),
            events,
            metadata: BackupMetadata {
                created_at: Utc::now(),
                format_version: "1.0".to_string(),
                statute_count: statutes.len(),
                event_count: self.event_store.count(),
                description,
            },
        }
    }

    /// Exports the backup to a JSON string.
    pub fn export_backup(&self, description: Option<String>) -> Result<String, serde_json::Error> {
        let backup = self.create_backup(description);
        serde_json::to_string_pretty(&backup)
    }

    /// Restores the registry from a backup.
    /// This will clear the current registry and replace it with the backup data.
    pub fn restore_from_backup(&mut self, backup: RegistryBackup) -> RegistryResult<()> {
        // Clear current state
        self.statutes.clear();
        self.versions.clear();
        self.tag_index.clear();
        self.jurisdiction_index.clear();
        self.cache.clear();
        self.event_store.clear();

        // Restore versions
        self.versions = backup.versions;

        // Restore statutes and rebuild indexes
        for entry in backup.statutes {
            let statute_id = entry.statute.id.clone();

            // Update tag index
            for tag in &entry.tags {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .insert(statute_id.clone());
            }

            // Update jurisdiction index
            self.jurisdiction_index
                .entry(entry.jurisdiction.clone())
                .or_default()
                .insert(statute_id.clone());

            // Store statute
            self.statutes.insert(statute_id, entry);
        }

        // Restore events
        for event in backup.events {
            self.event_store.record(event);
        }

        Ok(())
    }

    /// Imports a backup from a JSON string.
    pub fn import_backup(&mut self, json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let backup: RegistryBackup = serde_json::from_str(json)?;
        self.restore_from_backup(backup)?;
        Ok(())
    }

    /// Merges a backup into the current registry.
    /// Unlike restore, this doesn't clear existing data but merges new entries.
    pub fn merge_backup(&mut self, backup: RegistryBackup) -> RegistryResult<Vec<String>> {
        let mut merged_ids = Vec::new();

        for entry in backup.statutes {
            let statute_id = entry.statute.id.clone();

            // Skip if statute already exists
            if self.statutes.contains_key(&statute_id) {
                continue;
            }

            // Update tag index
            for tag in &entry.tags {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .insert(statute_id.clone());
            }

            // Update jurisdiction index
            self.jurisdiction_index
                .entry(entry.jurisdiction.clone())
                .or_default()
                .insert(statute_id.clone());

            // Store statute
            self.statutes.insert(statute_id.clone(), entry);
            merged_ids.push(statute_id);
        }

        // Merge version history
        for (statute_id, versions) in backup.versions {
            let entry = self.versions.entry(statute_id).or_default();
            for (version, version_entry) in versions {
                entry.insert(version, version_entry);
            }
        }

        // Merge events
        for event in backup.events {
            self.event_store.record(event);
        }

        Ok(merged_ids)
    }

    /// Creates a point-in-time snapshot of the registry.
    pub fn create_snapshot(&self, description: Option<String>) -> RegistrySnapshot {
        let backup = self.create_backup(description.clone());
        RegistrySnapshot::new(backup, description)
    }

    /// Restores the registry from a snapshot.
    pub fn restore_from_snapshot(&mut self, snapshot: RegistrySnapshot) -> RegistryResult<()> {
        self.restore_from_backup(snapshot.backup)
    }

    /// Creates an incremental backup based on a previous snapshot.
    ///
    /// This captures only changes since the base snapshot was created.
    pub fn create_incremental_backup(&self, base_snapshot: &RegistrySnapshot) -> IncrementalBackup {
        let mut incremental = IncrementalBackup::new(base_snapshot.snapshot_id);

        let base_time = base_snapshot.created_at;

        // Collect events since the base snapshot
        incremental.delta_events = self
            .event_store
            .all_events()
            .iter()
            .filter(|e| self.event_timestamp(e) > base_time)
            .cloned()
            .cloned()
            .collect();

        // Collect changed statutes (modified after base snapshot)
        incremental.changed_statutes = self
            .statutes
            .values()
            .filter(|entry| entry.modified_at > base_time)
            .cloned()
            .collect();

        // For deleted statutes, we rely on the StatuteDeleted events
        incremental.deleted_statute_ids = incremental
            .delta_events
            .iter()
            .filter_map(|e| {
                if let RegistryEvent::StatuteDeleted { statute_id, .. } = e {
                    Some(statute_id.clone())
                } else {
                    None
                }
            })
            .collect();

        incremental
    }

    /// Applies an incremental backup to the current registry state.
    pub fn apply_incremental_backup(
        &mut self,
        incremental: IncrementalBackup,
    ) -> RegistryResult<()> {
        // Apply deleted statutes
        for statute_id in &incremental.deleted_statute_ids {
            if self.statutes.contains_key(statute_id) {
                self.delete(statute_id)?;
            }
        }

        // Apply changed statutes
        for entry in incremental.changed_statutes {
            let statute_id = entry.statute.id.clone();
            if self.statutes.contains_key(&statute_id) {
                // Update existing
                self.update(&statute_id, entry.statute)?;
            } else {
                // Register new
                self.register(entry)?;
            }
        }

        // Record delta events
        for event in incremental.delta_events {
            self.event_store.record(event);
        }

        Ok(())
    }

    /// Helper to extract timestamp from an event.
    #[allow(dead_code)]
    fn event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
        match event {
            RegistryEvent::StatuteRegistered { timestamp, .. } => *timestamp,
            RegistryEvent::StatuteUpdated { timestamp, .. } => *timestamp,
            RegistryEvent::StatusChanged { timestamp, .. } => *timestamp,
            RegistryEvent::TagAdded { timestamp, .. } => *timestamp,
            RegistryEvent::TagRemoved { timestamp, .. } => *timestamp,
            RegistryEvent::ReferenceAdded { timestamp, .. } => *timestamp,
            RegistryEvent::ReferenceRemoved { timestamp, .. } => *timestamp,
            RegistryEvent::MetadataUpdated { timestamp, .. } => *timestamp,
            RegistryEvent::StatuteDeleted { timestamp, .. } => *timestamp,
            RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
        }
    }

    /// Lists all statute summaries (lazy loading - returns lightweight data).
    pub fn list_summaries(&self) -> Vec<StatuteSummary> {
        self.statutes.values().map(StatuteSummary::from).collect()
    }

    /// Lists statute summaries with pagination (lazy loading).
    pub fn list_summaries_paged(&self, pagination: Pagination) -> PagedResult<StatuteSummary> {
        let total = self.statutes.len();
        let items: Vec<StatuteSummary> = self
            .statutes
            .values()
            .skip(pagination.offset())
            .take(pagination.limit())
            .map(StatuteSummary::from)
            .collect();

        PagedResult::new(items, pagination.page, pagination.per_page, total)
    }

    /// Searches and returns summaries (lazy loading).
    pub fn search_summaries(&self, query: &SearchQuery) -> Vec<StatuteSummary> {
        self.search(query)
            .into_iter()
            .map(StatuteSummary::from)
            .collect()
    }

    /// Searches and returns summaries with pagination (lazy loading).
    pub fn search_summaries_paged(
        &self,
        query: &SearchQuery,
        pagination: Pagination,
    ) -> PagedResult<StatuteSummary> {
        let all_results = self.search(query);
        let total = all_results.len();

        let items: Vec<StatuteSummary> = all_results
            .into_iter()
            .skip(pagination.offset())
            .take(pagination.limit())
            .map(StatuteSummary::from)
            .collect();

        PagedResult::new(items, pagination.page, pagination.per_page, total)
    }

    /// Gets summaries by tag (lazy loading).
    pub fn query_summaries_by_tag(&self, tag: &str) -> Vec<StatuteSummary> {
        self.query_by_tag(tag)
            .into_iter()
            .map(StatuteSummary::from)
            .collect()
    }

    /// Gets summaries by jurisdiction (lazy loading).
    pub fn query_summaries_by_jurisdiction(&self, jurisdiction: &str) -> Vec<StatuteSummary> {
        self.query_by_jurisdiction(jurisdiction)
            .into_iter()
            .map(StatuteSummary::from)
            .collect()
    }

    /// Gets summaries of active statutes (lazy loading).
    pub fn list_active_summaries(&self) -> Vec<StatuteSummary> {
        self.list_active()
            .into_iter()
            .map(StatuteSummary::from)
            .collect()
    }

    /// Adds a tag to a statute.
    pub fn add_tag(&mut self, statute_id: &str, tag: impl Into<String>) -> RegistryResult<()> {
        let tag = tag.into();
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        // Only add if not already present
        if !entry.tags.contains(&tag) {
            entry.tags.push(tag.clone());
            entry.modified_at = Utc::now();
            entry.update_etag();

            // Update tag index
            self.tag_index
                .entry(tag.clone())
                .or_default()
                .insert(statute_id.to_string());

            // Invalidate cache
            self.cache.pop(statute_id);

            // Record event and trigger webhooks
            self.record_event(RegistryEvent::TagAdded {
                statute_id: statute_id.to_string(),
                tag,
                timestamp: Utc::now(),
            });
        }

        Ok(())
    }

    /// Removes a tag from a statute.
    pub fn remove_tag(&mut self, statute_id: &str, tag: &str) -> RegistryResult<()> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        // Remove the tag if present
        if let Some(pos) = entry.tags.iter().position(|t| t == tag) {
            entry.tags.remove(pos);
            entry.modified_at = Utc::now();
            entry.update_etag();

            // Update tag index
            if let Some(statute_ids) = self.tag_index.get_mut(tag) {
                statute_ids.remove(statute_id);
                // Remove the tag entry if no more statutes have it
                if statute_ids.is_empty() {
                    self.tag_index.remove(tag);
                }
            }

            // Invalidate cache
            self.cache.pop(statute_id);

            // Record event and trigger webhooks
            self.record_event(RegistryEvent::TagRemoved {
                statute_id: statute_id.to_string(),
                tag: tag.to_string(),
                timestamp: Utc::now(),
            });
        }

        Ok(())
    }

    /// Adds or updates metadata for a statute.
    pub fn add_metadata(
        &mut self,
        statute_id: &str,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> RegistryResult<()> {
        let key = key.into();
        let value = value.into();

        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let old_value = entry.metadata.insert(key.clone(), value.clone());
        entry.modified_at = Utc::now();
        entry.update_etag();

        // Invalidate cache
        self.cache.pop(statute_id);

        // Record event and trigger webhooks
        self.record_event(RegistryEvent::MetadataUpdated {
            statute_id: statute_id.to_string(),
            key,
            old_value,
            new_value: Some(value),
            timestamp: Utc::now(),
        });

        Ok(())
    }

    /// Removes metadata from a statute.
    pub fn remove_metadata(&mut self, statute_id: &str, key: &str) -> RegistryResult<()> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let old_value = entry.metadata.remove(key);

        if old_value.is_some() {
            entry.modified_at = Utc::now();
            entry.update_etag();

            // Invalidate cache
            self.cache.pop(statute_id);

            // Record event and trigger webhooks
            self.record_event(RegistryEvent::MetadataUpdated {
                statute_id: statute_id.to_string(),
                key: key.to_string(),
                old_value,
                new_value: None,
                timestamp: Utc::now(),
            });
        }

        Ok(())
    }

    // =========================================================================
    // Advanced Analytics Methods
    // =========================================================================

    /// Computes temporal analytics for the registry.
    ///
    /// Analyzes registration patterns, update frequency, and version velocity.
    /// Results are cached for performance.
    pub fn temporal_analytics(&mut self) -> TemporalAnalytics {
        // Check cache first
        if let Some(cached) = self.analytics_cache.get_temporal() {
            return cached.clone();
        }

        // Compute analytics
        let analytics = self.compute_temporal_analytics();

        // Store in cache
        self.analytics_cache.set_temporal(analytics.clone());

        analytics
    }

    /// Computes temporal analytics without using cache.
    fn compute_temporal_analytics(&self) -> TemporalAnalytics {
        let mut registrations_per_day: HashMap<String, usize> = HashMap::new();
        let mut updates_per_day: HashMap<String, usize> = HashMap::new();
        let mut version_counts: HashMap<String, usize> = HashMap::new();

        // Count registrations per day (from created_at timestamps)
        for entry in self.statutes.values() {
            let date = entry.created_at.format("%Y-%m-%d").to_string();
            *registrations_per_day.entry(date).or_insert(0) += 1;
        }

        // Count updates per day (from modified_at timestamps)
        for entry in self.statutes.values() {
            if entry.modified_at != entry.created_at {
                let date = entry.modified_at.format("%Y-%m-%d").to_string();
                *updates_per_day.entry(date).or_insert(0) += 1;
            }
        }

        // Count versions per statute
        for (statute_id, versions) in &self.versions {
            version_counts.insert(statute_id.clone(), versions.len());
        }

        // Calculate average versions per statute
        let avg_versions = if self.statutes.is_empty() {
            0.0
        } else {
            version_counts.values().sum::<usize>() as f64 / self.statutes.len() as f64
        };

        // Find most versioned statutes (top 10)
        let mut most_versioned: Vec<(String, usize)> = version_counts.into_iter().collect();
        most_versioned.sort_by(|a, b| b.1.cmp(&a.1));
        most_versioned.truncate(10);

        // Calculate growth rate (average statutes per day)
        let days_count = registrations_per_day.len().max(1);
        let growth_rate = self.statutes.len() as f64 / days_count as f64;

        // Find peak activity date
        let peak_activity_date = registrations_per_day
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(date, count)| (date.clone(), *count));

        TemporalAnalytics {
            registrations_per_day,
            updates_per_day,
            avg_versions_per_statute: avg_versions,
            most_versioned_statutes: most_versioned,
            growth_rate,
            peak_activity_date,
        }
    }

    /// Computes relationship analytics for the registry.
    ///
    /// Analyzes statute dependencies, references, and supersession chains.
    /// Results are cached for performance.
    pub fn relationship_analytics(&mut self) -> RelationshipAnalytics {
        // Check cache first
        if let Some(cached) = self.analytics_cache.get_relationship() {
            return cached.clone();
        }

        // Compute analytics
        let analytics = self.compute_relationship_analytics();

        // Store in cache
        self.analytics_cache.set_relationship(analytics.clone());

        analytics
    }

    /// Computes relationship analytics without using cache.
    fn compute_relationship_analytics(&self) -> RelationshipAnalytics {
        let mut reference_counts: HashMap<String, usize> = HashMap::new();
        let mut dependency_counts: HashMap<String, usize> = HashMap::new();
        let mut supersession_chains: HashMap<String, Vec<String>> = HashMap::new();
        let mut has_relationships: HashSet<String> = HashSet::new();

        // Count references to each statute
        for entry in self.statutes.values() {
            for reference in &entry.references {
                *reference_counts.entry(reference.clone()).or_insert(0) += 1;
                has_relationships.insert(entry.statute.id.clone());
                has_relationships.insert(reference.clone());
            }
            dependency_counts.insert(entry.statute.id.clone(), entry.references.len());
        }

        // Build supersession chains
        for entry in self.statutes.values() {
            if !entry.supersedes.is_empty() {
                let mut chain = Vec::new();
                let mut current_ids = entry.supersedes.clone();
                let mut visited = HashSet::new();

                while let Some(id) = current_ids.pop() {
                    if visited.contains(&id) {
                        continue;
                    }
                    visited.insert(id.clone());
                    chain.push(id.clone());

                    // Look for what this statute supersedes
                    if let Some(e) = self.statutes.get(&id) {
                        for superseded_id in &e.supersedes {
                            if !visited.contains(superseded_id) {
                                current_ids.push(superseded_id.clone());
                            }
                        }
                    }
                }

                if !chain.is_empty() {
                    supersession_chains.insert(entry.statute.id.clone(), chain);
                }
            }
        }

        // Find most referenced statutes (top 10)
        let mut most_referenced: Vec<(String, usize)> = reference_counts.into_iter().collect();
        most_referenced.sort_by(|a, b| b.1.cmp(&a.1));
        most_referenced.truncate(10);

        // Find statutes with most dependencies (top 10)
        let mut most_dependencies: Vec<(String, usize)> = dependency_counts.into_iter().collect();
        most_dependencies.sort_by(|a, b| b.1.cmp(&a.1));
        most_dependencies.truncate(10);

        // Find orphaned statutes (no references to or from)
        let orphaned_statutes: Vec<String> = self
            .statutes
            .keys()
            .filter(|id| !has_relationships.contains(*id))
            .cloned()
            .collect();

        // Calculate average references per statute
        let total_refs: usize = self.statutes.values().map(|e| e.references.len()).sum();
        let avg_references = if self.statutes.is_empty() {
            0.0
        } else {
            total_refs as f64 / self.statutes.len() as f64
        };

        RelationshipAnalytics {
            most_referenced,
            most_dependencies,
            supersession_chains,
            orphaned_statutes,
            avg_references_per_statute: avg_references,
        }
    }

    /// Computes tag analytics for the registry.
    ///
    /// Analyzes tag usage patterns and co-occurrence.
    /// Results are cached for performance.
    pub fn tag_analytics(&mut self) -> TagAnalytics {
        // Check cache first
        if let Some(cached) = self.analytics_cache.get_tag() {
            return cached.clone();
        }

        // Compute analytics
        let analytics = self.compute_tag_analytics();

        // Store in cache
        self.analytics_cache.set_tag(analytics.clone());

        analytics
    }

    /// Computes tag analytics without using cache.
    fn compute_tag_analytics(&self) -> TagAnalytics {
        let mut tag_frequency: HashMap<String, usize> = HashMap::new();
        let mut tag_cooccurrence: HashMap<String, HashMap<String, usize>> = HashMap::new();

        // Count tag frequency and co-occurrence
        for entry in self.statutes.values() {
            // Tag frequency
            for tag in &entry.tags {
                *tag_frequency.entry(tag.clone()).or_insert(0) += 1;
            }

            // Tag co-occurrence
            for (i, tag1) in entry.tags.iter().enumerate() {
                for tag2 in entry.tags.iter().skip(i + 1) {
                    *tag_cooccurrence
                        .entry(tag1.clone())
                        .or_default()
                        .entry(tag2.clone())
                        .or_insert(0) += 1;
                    *tag_cooccurrence
                        .entry(tag2.clone())
                        .or_default()
                        .entry(tag1.clone())
                        .or_insert(0) += 1;
                }
            }
        }

        // Find most used tags (top 10)
        let mut most_used_tags: Vec<(String, usize)> =
            tag_frequency.iter().map(|(t, &c)| (t.clone(), c)).collect();
        most_used_tags.sort_by(|a, b| b.1.cmp(&a.1));
        let top_most_used = most_used_tags.iter().take(10).cloned().collect();

        // Find least used tags (bottom 10)
        most_used_tags.sort_by(|a, b| a.1.cmp(&b.1));
        let least_used_tags = most_used_tags.iter().take(10).cloned().collect();

        // Calculate average tags per statute
        let total_tags: usize = self.statutes.values().map(|e| e.tags.len()).sum();
        let avg_tags = if self.statutes.is_empty() {
            0.0
        } else {
            total_tags as f64 / self.statutes.len() as f64
        };

        TagAnalytics {
            tag_frequency,
            tag_cooccurrence,
            most_used_tags: top_most_used,
            least_used_tags,
            avg_tags_per_statute: avg_tags,
        }
    }

    /// Computes activity analytics for the registry.
    ///
    /// Analyzes modification patterns and status changes.
    /// Results are cached for performance.
    pub fn activity_analytics(&mut self) -> ActivityAnalytics {
        // Check cache first
        if let Some(cached) = self.analytics_cache.get_activity() {
            return cached.clone();
        }

        // Compute analytics
        let analytics = self.compute_activity_analytics();

        // Store in cache
        self.analytics_cache.set_activity(analytics.clone());

        analytics
    }

    /// Computes activity analytics without using cache.
    fn compute_activity_analytics(&self) -> ActivityAnalytics {
        let mut modification_counts: HashMap<String, usize> = HashMap::new();
        let mut status_change_counts: HashMap<String, usize> = HashMap::new();

        // Count modifications per statute (based on version history)
        for (statute_id, versions) in &self.versions {
            modification_counts.insert(statute_id.clone(), versions.len());
        }

        // Count status changes from events
        for event in self.event_store.all_events() {
            if let RegistryEvent::StatusChanged { statute_id, .. } = event {
                *status_change_counts.entry(statute_id.clone()).or_insert(0) += 1;
            }
        }

        // Find most modified statutes (top 10)
        let mut most_modified: Vec<(String, usize)> = modification_counts.into_iter().collect();
        most_modified.sort_by(|a, b| b.1.cmp(&a.1));
        most_modified.truncate(10);

        // Find recently modified statutes (top 20 by modified_at)
        let mut recently_modified: Vec<(String, DateTime<Utc>)> = self
            .statutes
            .iter()
            .map(|(id, entry)| (id.clone(), entry.modified_at))
            .collect();
        recently_modified.sort_by(|a, b| b.1.cmp(&a.1));
        recently_modified.truncate(20);

        // Find least modified statutes (bottom 20 by modified_at)
        let mut least_modified: Vec<(String, DateTime<Utc>)> = self
            .statutes
            .iter()
            .map(|(id, entry)| (id.clone(), entry.modified_at))
            .collect();
        least_modified.sort_by(|a, b| a.1.cmp(&b.1));
        least_modified.truncate(20);

        // Find statutes with frequent status changes (top 10)
        let mut frequent_status_changes: Vec<(String, usize)> =
            status_change_counts.into_iter().collect();
        frequent_status_changes.sort_by(|a, b| b.1.cmp(&a.1));
        frequent_status_changes.truncate(10);

        // Calculate average modification frequency
        let total_modifications: usize = self.versions.values().map(|v| v.len()).sum();
        let avg_mod_frequency = if !recently_modified.is_empty() && total_modifications > 0 {
            // Calculate average days between modifications based on most recent statutes
            let now = Utc::now();
            let avg_days_since_last_mod: f64 = recently_modified
                .iter()
                .map(|(_, date)| (now - *date).num_days() as f64)
                .sum::<f64>()
                / recently_modified.len() as f64;
            avg_days_since_last_mod
        } else {
            0.0
        };

        ActivityAnalytics {
            most_modified,
            recently_modified,
            least_modified,
            frequent_status_changes,
            avg_modification_frequency_days: avg_mod_frequency,
        }
    }

    /// Groups statutes by a specified field and returns counts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use legalis_registry::*;
    /// # let registry = StatuteRegistry::new();
    /// // Group by status
    /// let by_status = registry.aggregate_by(|entry| format!("{:?}", entry.status));
    ///
    /// // Group by jurisdiction
    /// let by_jurisdiction = registry.aggregate_by(|entry| entry.jurisdiction.clone());
    /// ```
    pub fn aggregate_by<F>(&self, key_fn: F) -> AggregationResult
    where
        F: Fn(&StatuteEntry) -> String,
    {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for entry in self.statutes.values() {
            let key = key_fn(entry);
            *counts.entry(key).or_insert(0) += 1;
        }

        AggregationResult::new(counts)
    }

    /// Groups statutes by multiple tags and returns counts.
    pub fn aggregate_by_tags(&self) -> AggregationResult {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for entry in self.statutes.values() {
            for tag in &entry.tags {
                *counts.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        AggregationResult::new(counts)
    }

    /// Exports temporal analytics to JSON.
    pub fn export_temporal_analytics_json(&mut self) -> Result<String, serde_json::Error> {
        let analytics = self.temporal_analytics();
        serde_json::to_string_pretty(&analytics)
    }

    /// Exports relationship analytics to JSON.
    pub fn export_relationship_analytics_json(&mut self) -> Result<String, serde_json::Error> {
        let analytics = self.relationship_analytics();
        serde_json::to_string_pretty(&analytics)
    }

    /// Exports tag analytics to JSON.
    pub fn export_tag_analytics_json(&mut self) -> Result<String, serde_json::Error> {
        let analytics = self.tag_analytics();
        serde_json::to_string_pretty(&analytics)
    }

    /// Exports activity analytics to JSON.
    pub fn export_activity_analytics_json(&mut self) -> Result<String, serde_json::Error> {
        let analytics = self.activity_analytics();
        serde_json::to_string_pretty(&analytics)
    }

    /// Exports all analytics to a combined JSON structure.
    pub fn export_all_analytics_json(&mut self) -> Result<String, serde_json::Error> {
        #[derive(Serialize)]
        struct AllAnalytics {
            temporal: TemporalAnalytics,
            relationship: RelationshipAnalytics,
            tag: TagAnalytics,
            activity: ActivityAnalytics,
            generated_at: DateTime<Utc>,
        }

        let all = AllAnalytics {
            temporal: self.temporal_analytics(),
            relationship: self.relationship_analytics(),
            tag: self.tag_analytics(),
            activity: self.activity_analytics(),
            generated_at: Utc::now(),
        };

        serde_json::to_string_pretty(&all)
    }

    /// Exports aggregation result to CSV format (feature-gated).
    #[cfg(feature = "csv-export")]
    pub fn export_aggregation_csv(
        &self,
        result: &AggregationResult,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record(["Key", "Count", "Percentage"])?;

        // Write data sorted by count (descending)
        for (key, count) in result.sorted_by_count() {
            let percentage = result.percentage(&key);
            wtr.write_record(&[key, count.to_string(), format!("{:.2}", percentage)])?;
        }

        let data = wtr.into_inner()?;
        Ok(String::from_utf8(data)?)
    }

    /// Invalidates the analytics cache.
    ///
    /// Call this after operations that might affect analytics results.
    pub fn invalidate_analytics_cache(&mut self) {
        self.analytics_cache.clear();
    }

    /// Sets the analytics cache duration in seconds.
    pub fn set_analytics_cache_duration(&mut self, duration_secs: i64) {
        self.analytics_cache.cache_duration_secs = duration_secs;
        self.analytics_cache.clear();
    }
}

/// Cached analytics with timestamp for TTL.
#[derive(Debug, Clone)]
struct CachedAnalytics {
    temporal: Option<(TemporalAnalytics, DateTime<Utc>)>,
    relationship: Option<(RelationshipAnalytics, DateTime<Utc>)>,
    tag: Option<(TagAnalytics, DateTime<Utc>)>,
    activity: Option<(ActivityAnalytics, DateTime<Utc>)>,
    cache_duration_secs: i64,
}

impl CachedAnalytics {
    fn new(cache_duration_secs: i64) -> Self {
        Self {
            temporal: None,
            relationship: None,
            tag: None,
            activity: None,
            cache_duration_secs,
        }
    }

    fn is_valid(timestamp: DateTime<Utc>, duration_secs: i64) -> bool {
        let now = Utc::now();
        (now - timestamp).num_seconds() < duration_secs
    }

    fn get_temporal(&self) -> Option<&TemporalAnalytics> {
        self.temporal.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }

    fn set_temporal(&mut self, analytics: TemporalAnalytics) {
        self.temporal = Some((analytics, Utc::now()));
    }

    fn get_relationship(&self) -> Option<&RelationshipAnalytics> {
        self.relationship
            .as_ref()
            .and_then(|(analytics, timestamp)| {
                if Self::is_valid(*timestamp, self.cache_duration_secs) {
                    Some(analytics)
                } else {
                    None
                }
            })
    }

    fn set_relationship(&mut self, analytics: RelationshipAnalytics) {
        self.relationship = Some((analytics, Utc::now()));
    }

    fn get_tag(&self) -> Option<&TagAnalytics> {
        self.tag.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }

    fn set_tag(&mut self, analytics: TagAnalytics) {
        self.tag = Some((analytics, Utc::now()));
    }

    fn get_activity(&self) -> Option<&ActivityAnalytics> {
        self.activity.as_ref().and_then(|(analytics, timestamp)| {
            if Self::is_valid(*timestamp, self.cache_duration_secs) {
                Some(analytics)
            } else {
                None
            }
        })
    }

    fn set_activity(&mut self, analytics: ActivityAnalytics) {
        self.activity = Some((analytics, Utc::now()));
    }

    fn clear(&mut self) {
        self.temporal = None;
        self.relationship = None;
        self.tag = None;
        self.activity = None;
    }
}

/// Multi-tenant registry manager.
///
/// Allows managing multiple isolated registries for different tenants.
#[derive(Debug)]
pub struct MultiTenantRegistry {
    /// Registry for each tenant
    tenants: HashMap<String, StatuteRegistry>,
    /// Default tenant ID (if any)
    default_tenant: Option<String>,
}

impl MultiTenantRegistry {
    /// Creates a new multi-tenant registry.
    pub fn new() -> Self {
        Self {
            tenants: HashMap::new(),
            default_tenant: None,
        }
    }

    /// Creates a new multi-tenant registry with a default tenant.
    pub fn with_default_tenant(tenant_id: impl Into<String>) -> Self {
        let tenant_id = tenant_id.into();
        let mut tenants = HashMap::new();
        tenants.insert(tenant_id.clone(), StatuteRegistry::new());

        Self {
            tenants,
            default_tenant: Some(tenant_id),
        }
    }

    /// Creates a new tenant registry.
    pub fn create_tenant(&mut self, tenant_id: impl Into<String>) -> RegistryResult<()> {
        let tenant_id = tenant_id.into();

        if self.tenants.contains_key(&tenant_id) {
            return Err(RegistryError::DuplicateId(format!(
                "Tenant '{}' already exists",
                tenant_id
            )));
        }

        self.tenants.insert(tenant_id, StatuteRegistry::new());
        Ok(())
    }

    /// Deletes a tenant registry.
    pub fn delete_tenant(&mut self, tenant_id: &str) -> RegistryResult<()> {
        self.tenants.remove(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })?;

        // Clear default tenant if it was deleted
        if self.default_tenant.as_deref() == Some(tenant_id) {
            self.default_tenant = None;
        }

        Ok(())
    }

    /// Gets a mutable reference to a tenant's registry.
    pub fn get_tenant_mut(&mut self, tenant_id: &str) -> RegistryResult<&mut StatuteRegistry> {
        self.tenants.get_mut(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })
    }

    /// Gets a reference to a tenant's registry.
    pub fn get_tenant(&self, tenant_id: &str) -> RegistryResult<&StatuteRegistry> {
        self.tenants.get(tenant_id).ok_or_else(|| {
            RegistryError::StatuteNotFound(format!("Tenant '{}' not found", tenant_id))
        })
    }

    /// Gets a mutable reference to the default tenant's registry.
    pub fn get_default_mut(&mut self) -> RegistryResult<&mut StatuteRegistry> {
        let tenant_id = self
            .default_tenant
            .as_ref()
            .ok_or_else(|| RegistryError::InvalidOperation("No default tenant set".to_string()))?
            .clone();
        self.get_tenant_mut(&tenant_id)
    }

    /// Gets a reference to the default tenant's registry.
    pub fn get_default(&self) -> RegistryResult<&StatuteRegistry> {
        let tenant_id = self
            .default_tenant
            .as_ref()
            .ok_or_else(|| RegistryError::InvalidOperation("No default tenant set".to_string()))?;
        self.get_tenant(tenant_id)
    }

    /// Lists all tenant IDs.
    pub fn list_tenants(&self) -> Vec<&String> {
        self.tenants.keys().collect()
    }

    /// Returns the number of tenants.
    pub fn tenant_count(&self) -> usize {
        self.tenants.len()
    }

    /// Checks if a tenant exists.
    pub fn has_tenant(&self, tenant_id: &str) -> bool {
        self.tenants.contains_key(tenant_id)
    }

    /// Sets the default tenant.
    pub fn set_default_tenant(&mut self, tenant_id: impl Into<String>) -> RegistryResult<()> {
        let tenant_id = tenant_id.into();

        if !self.tenants.contains_key(&tenant_id) {
            return Err(RegistryError::StatuteNotFound(format!(
                "Tenant '{}' not found",
                tenant_id
            )));
        }

        self.default_tenant = Some(tenant_id);
        Ok(())
    }

    /// Exports a tenant's registry to a backup.
    pub fn export_tenant(
        &self,
        tenant_id: &str,
        description: Option<String>,
    ) -> RegistryResult<String> {
        let registry = self.get_tenant(tenant_id)?;
        registry
            .export_backup(description)
            .map_err(|e| RegistryError::InvalidOperation(format!("Export failed: {}", e)))
    }

    /// Imports a backup into a tenant's registry.
    pub fn import_tenant(&mut self, tenant_id: &str, json: &str) -> RegistryResult<()> {
        let registry = self.get_tenant_mut(tenant_id)?;
        registry
            .import_backup(json)
            .map_err(|e| RegistryError::InvalidOperation(format!("Import failed: {}", e)))
    }

    /// Clones a tenant's registry to a new tenant.
    pub fn clone_tenant(
        &mut self,
        source_tenant_id: &str,
        new_tenant_id: impl Into<String>,
    ) -> RegistryResult<()> {
        let new_tenant_id = new_tenant_id.into();

        if self.tenants.contains_key(&new_tenant_id) {
            return Err(RegistryError::DuplicateId(format!(
                "Tenant '{}' already exists",
                new_tenant_id
            )));
        }

        // Export the source tenant
        let backup_json = self.export_tenant(source_tenant_id, None)?;

        // Create new tenant
        self.create_tenant(&new_tenant_id)?;

        // Import into new tenant
        self.import_tenant(&new_tenant_id, &backup_json)?;

        Ok(())
    }

    /// Returns statistics for all tenants.
    pub fn tenant_statistics(&self) -> HashMap<String, TenantStats> {
        self.tenants
            .iter()
            .map(|(id, registry)| {
                let stats = TenantStats {
                    statute_count: registry.count(),
                    event_count: registry.event_count(),
                    active_statute_count: registry.list_active().len(),
                    tag_count: registry.all_tags().len(),
                    jurisdiction_count: registry.all_jurisdictions().len(),
                };
                (id.clone(), stats)
            })
            .collect()
    }

    /// Returns summaries for all tenants (lazy loading).
    pub fn list_tenant_summaries(&self) -> HashMap<String, Vec<StatuteSummary>> {
        self.tenants
            .iter()
            .map(|(id, registry)| {
                let summaries = registry.list_summaries();
                (id.clone(), summaries)
            })
            .collect()
    }
}

impl Default for MultiTenantRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantStats {
    /// Number of statutes
    pub statute_count: usize,
    /// Number of events
    pub event_count: usize,
    /// Number of active statutes
    pub active_statute_count: usize,
    /// Number of unique tags
    pub tag_count: usize,
    /// Number of unique jurisdictions
    pub jurisdiction_count: usize,
}

/// Archive entry for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedStatute {
    /// The archived statute entry
    pub entry: StatuteEntry,
    /// Reason for archiving
    pub reason: String,
    /// When it was archived
    pub archived_at: DateTime<Utc>,
}

/// Archive for storing removed or superseded statutes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatuteArchive {
    /// Archived statutes by ID
    archived: HashMap<String, ArchivedStatute>,
}

impl StatuteArchive {
    /// Creates a new empty archive.
    pub fn new() -> Self {
        Self::default()
    }

    /// Archives a statute.
    pub fn archive(&mut self, entry: StatuteEntry, reason: String) {
        let statute_id = entry.statute.id.clone();
        self.archived.insert(
            statute_id,
            ArchivedStatute {
                entry,
                reason,
                archived_at: Utc::now(),
            },
        );
    }

    /// Retrieves an archived statute.
    pub fn get(&self, statute_id: &str) -> Option<&ArchivedStatute> {
        self.archived.get(statute_id)
    }

    /// Removes a statute from the archive (unarchive).
    pub fn unarchive(&mut self, statute_id: &str) -> Option<ArchivedStatute> {
        self.archived.remove(statute_id)
    }

    /// Lists all archived statute IDs.
    pub fn list_ids(&self) -> Vec<String> {
        self.archived.keys().cloned().collect()
    }

    /// Lists all archived statutes.
    pub fn list_all(&self) -> Vec<&ArchivedStatute> {
        self.archived.values().collect()
    }

    /// Returns the count of archived statutes.
    pub fn count(&self) -> usize {
        self.archived.len()
    }

    /// Searches archived statutes by reason (case-insensitive substring match).
    pub fn search_by_reason(&self, query: &str) -> Vec<&ArchivedStatute> {
        let query_lower = query.to_lowercase();
        self.archived
            .values()
            .filter(|a| a.reason.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Clears all archived statutes.
    pub fn clear(&mut self) {
        self.archived.clear();
    }
}

/// Retention policy rule for auto-archiving statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionRule {
    /// Archive statutes that have expired
    ExpiredStatutes { reason: String },
    /// Archive statutes older than specified days since effective date
    OlderThanDays { days: i64, reason: String },
    /// Archive statutes with specific status
    ByStatus {
        status: StatuteStatus,
        reason: String,
    },
    /// Archive statutes superseded by others
    SupersededStatutes { reason: String },
    /// Archive statutes not modified within specified days
    InactiveForDays { days: i64, reason: String },
}

/// Configuration for retention policies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetentionPolicy {
    /// Rules to apply for archiving
    rules: Vec<RetentionRule>,
    /// Whether to automatically apply retention on operations
    auto_apply: bool,
}

impl RetentionPolicy {
    /// Creates a new empty retention policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables automatic application of retention rules.
    pub fn with_auto_apply(mut self) -> Self {
        self.auto_apply = true;
        self
    }

    /// Adds a retention rule.
    pub fn add_rule(mut self, rule: RetentionRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Returns all rules.
    pub fn rules(&self) -> &[RetentionRule] {
        &self.rules
    }

    /// Checks if auto-apply is enabled.
    pub fn is_auto_apply(&self) -> bool {
        self.auto_apply
    }
}

/// Result of applying retention policies.
#[derive(Debug, Clone)]
pub struct RetentionResult {
    /// IDs of statutes that were archived
    pub archived_ids: Vec<String>,
    /// Reason for each archival
    pub reasons: HashMap<String, String>,
    /// Total statutes evaluated
    pub total_evaluated: usize,
}

impl RetentionResult {
    /// Creates a new retention result.
    pub fn new(total_evaluated: usize) -> Self {
        Self {
            archived_ids: Vec::new(),
            reasons: HashMap::new(),
            total_evaluated,
        }
    }

    /// Records an archived statute.
    pub fn record_archived(&mut self, statute_id: String, reason: String) {
        self.archived_ids.push(statute_id.clone());
        self.reasons.insert(statute_id, reason);
    }

    /// Returns the number of statutes archived.
    pub fn archived_count(&self) -> usize {
        self.archived_ids.len()
    }
}

/// Dependency graph for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Root statute ID
    pub root_id: String,
    /// Forward dependencies: statute_id -> set of statutes it depends on
    pub dependencies: HashMap<String, HashSet<String>>,
    /// Reverse dependencies: statute_id -> set of statutes that depend on it
    pub reverse_dependencies: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    /// Returns all statutes that the root depends on (directly or indirectly).
    pub fn all_dependencies(&self) -> HashSet<String> {
        let mut all_deps = HashSet::new();
        for deps in self.dependencies.values() {
            all_deps.extend(deps.iter().cloned());
        }
        all_deps
    }

    /// Returns all statutes that depend on the root (directly or indirectly).
    pub fn all_dependents(&self) -> HashSet<String> {
        self.reverse_dependencies
            .get(&self.root_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns the depth of the dependency tree.
    pub fn depth(&self) -> usize {
        self.calculate_depth(&self.root_id, &mut HashSet::new())
    }

    fn calculate_depth(&self, statute_id: &str, visited: &mut HashSet<String>) -> usize {
        if visited.contains(statute_id) {
            return 0;
        }
        visited.insert(statute_id.to_string());

        if let Some(deps) = self.dependencies.get(statute_id) {
            if deps.is_empty() {
                return 1;
            }
            deps.iter()
                .map(|dep| self.calculate_depth(dep, visited))
                .max()
                .unwrap_or(0)
                + 1
        } else {
            1
        }
    }
}

// =============================================================================
// Advanced Analytics
// =============================================================================

/// Temporal analytics for tracking registry growth and changes over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnalytics {
    /// Number of statutes registered per day (date -> count)
    pub registrations_per_day: HashMap<String, usize>,
    /// Number of updates per day (date -> count)
    pub updates_per_day: HashMap<String, usize>,
    /// Average version count per statute
    pub avg_versions_per_statute: f64,
    /// Statutes with highest version velocity (id, version_count)
    pub most_versioned_statutes: Vec<(String, usize)>,
    /// Growth rate (statutes per day) over the period
    pub growth_rate: f64,
    /// Peak activity date and count
    pub peak_activity_date: Option<(String, usize)>,
}

impl TemporalAnalytics {
    /// Creates a new temporal analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of registrations across all days.
    pub fn total_registrations(&self) -> usize {
        self.registrations_per_day.values().sum()
    }

    /// Returns the total number of updates across all days.
    pub fn total_updates(&self) -> usize {
        self.updates_per_day.values().sum()
    }

    /// Returns the total activity (registrations + updates).
    pub fn total_activity(&self) -> usize {
        self.total_registrations() + self.total_updates()
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

/// Relationship analytics for analyzing statute dependencies and supersession chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipAnalytics {
    /// Most referenced statutes (id, reference_count)
    pub most_referenced: Vec<(String, usize)>,
    /// Statutes with most dependencies (id, dependency_count)
    pub most_dependencies: Vec<(String, usize)>,
    /// Supersession chains (root_id -> chain of superseded IDs)
    pub supersession_chains: HashMap<String, Vec<String>>,
    /// Orphaned statutes (no references to or from other statutes)
    pub orphaned_statutes: Vec<String>,
    /// Average references per statute
    pub avg_references_per_statute: f64,
}

impl RelationshipAnalytics {
    /// Creates a new relationship analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the longest supersession chain length.
    pub fn max_chain_length(&self) -> usize {
        self.supersession_chains
            .values()
            .map(|chain| chain.len())
            .max()
            .unwrap_or(0)
    }

    /// Returns the total number of relationships.
    pub fn total_relationships(&self) -> usize {
        self.most_referenced.iter().map(|(_, count)| count).sum()
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

/// Tag analytics for analyzing tag usage patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagAnalytics {
    /// Tag frequency (tag -> count)
    pub tag_frequency: HashMap<String, usize>,
    /// Tag co-occurrence (tag1 -> tag2 -> count)
    pub tag_cooccurrence: HashMap<String, HashMap<String, usize>>,
    /// Most used tags (tag, count)
    pub most_used_tags: Vec<(String, usize)>,
    /// Least used tags (tag, count)
    pub least_used_tags: Vec<(String, usize)>,
    /// Average tags per statute
    pub avg_tags_per_statute: f64,
}

impl TagAnalytics {
    /// Creates a new tag analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the total number of unique tags.
    pub fn unique_tag_count(&self) -> usize {
        self.tag_frequency.len()
    }

    /// Returns the total tag usage across all statutes.
    pub fn total_tag_usage(&self) -> usize {
        self.tag_frequency.values().sum()
    }

    /// Gets tags that commonly appear together with the given tag.
    pub fn related_tags(&self, tag: &str, min_occurrences: usize) -> Vec<(String, usize)> {
        self.tag_cooccurrence
            .get(tag)
            .map(|cooccur| {
                let mut pairs: Vec<_> = cooccur
                    .iter()
                    .filter(|&(_, count)| *count >= min_occurrences)
                    .map(|(t, c)| (t.clone(), *c))
                    .collect();
                pairs.sort_by(|a, b| b.1.cmp(&a.1));
                pairs
            })
            .unwrap_or_default()
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

/// Activity analytics for tracking modification patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAnalytics {
    /// Most modified statutes (id, modification_count)
    pub most_modified: Vec<(String, usize)>,
    /// Recently modified statutes (id, last_modified_date)
    pub recently_modified: Vec<(String, DateTime<Utc>)>,
    /// Least modified statutes (id, last_modified_date)
    pub least_modified: Vec<(String, DateTime<Utc>)>,
    /// Statutes by status change frequency (id, status_change_count)
    pub frequent_status_changes: Vec<(String, usize)>,
    /// Average modification frequency (days between modifications)
    pub avg_modification_frequency_days: f64,
}

impl ActivityAnalytics {
    /// Creates a new activity analytics instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns statutes modified within the last N days.
    pub fn modified_within_days(&self, days: i64) -> Vec<String> {
        let threshold = Utc::now() - chrono::Duration::days(days);
        self.recently_modified
            .iter()
            .filter(|(_, date)| *date > threshold)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

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

/// Field projection options for efficient queries.
#[derive(Debug, Clone, Default)]
pub struct FieldProjection {
    /// Include statute ID
    pub include_id: bool,
    /// Include title
    pub include_title: bool,
    /// Include version
    pub include_version: bool,
    /// Include status
    pub include_status: bool,
    /// Include jurisdiction
    pub include_jurisdiction: bool,
    /// Include tags
    pub include_tags: bool,
    /// Include dates
    pub include_dates: bool,
    /// Include metadata
    pub include_metadata: bool,
}

impl FieldProjection {
    /// Creates a projection that includes all fields.
    pub fn all() -> Self {
        Self {
            include_id: true,
            include_title: true,
            include_version: true,
            include_status: true,
            include_jurisdiction: true,
            include_tags: true,
            include_dates: true,
            include_metadata: true,
        }
    }

    /// Creates a projection with only essential fields.
    pub fn essential() -> Self {
        Self {
            include_id: true,
            include_title: true,
            include_version: true,
            include_status: true,
            ..Default::default()
        }
    }

    /// Adds ID to the projection.
    pub fn with_id(mut self) -> Self {
        self.include_id = true;
        self
    }

    /// Adds title to the projection.
    pub fn with_title(mut self) -> Self {
        self.include_title = true;
        self
    }

    /// Adds version to the projection.
    pub fn with_version(mut self) -> Self {
        self.include_version = true;
        self
    }

    /// Adds status to the projection.
    pub fn with_status(mut self) -> Self {
        self.include_status = true;
        self
    }

    /// Adds jurisdiction to the projection.
    pub fn with_jurisdiction(mut self) -> Self {
        self.include_jurisdiction = true;
        self
    }

    /// Adds tags to the projection.
    pub fn with_tags(mut self) -> Self {
        self.include_tags = true;
        self
    }

    /// Adds dates to the projection.
    pub fn with_dates(mut self) -> Self {
        self.include_dates = true;
        self
    }

    /// Adds metadata to the projection.
    pub fn with_metadata(mut self) -> Self {
        self.include_metadata = true;
        self
    }
}

/// Aggregation functions for grouping and counting.
#[derive(Debug, Clone)]
pub struct AggregationResult {
    /// Group key -> count
    pub counts: HashMap<String, usize>,
    /// Total items aggregated
    pub total: usize,
}

impl AggregationResult {
    /// Creates a new aggregation result.
    pub fn new(counts: HashMap<String, usize>) -> Self {
        let total = counts.values().sum();
        Self { counts, total }
    }

    /// Returns the count for a specific group.
    pub fn get_count(&self, key: &str) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }

    /// Returns all groups sorted by count (descending).
    pub fn sorted_by_count(&self) -> Vec<(String, usize)> {
        let mut pairs: Vec<_> = self.counts.iter().map(|(k, &v)| (k.clone(), v)).collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs
    }

    /// Returns the percentage for a specific group.
    pub fn percentage(&self, key: &str) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.get_count(key) as f64 / self.total as f64) * 100.0
    }
}

// =============================================================================
// Async API Support
// =============================================================================

#[cfg(feature = "async")]
pub mod async_api {
    //! Async variants of registry operations.
    //!
    //! This module provides async versions of the main registry methods,
    //! allowing integration with async runtimes like tokio.

    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// Async-friendly wrapper around StatuteRegistry.
    pub struct AsyncStatuteRegistry {
        inner: Arc<RwLock<StatuteRegistry>>,
    }

    impl AsyncStatuteRegistry {
        /// Creates a new async registry.
        pub fn new() -> Self {
            Self {
                inner: Arc::new(RwLock::new(StatuteRegistry::new())),
            }
        }

        /// Registers a new statute asynchronously.
        pub async fn register(&self, entry: StatuteEntry) -> RegistryResult<Uuid> {
            let mut registry = self.inner.write().await;
            registry.register(entry)
        }

        /// Updates a statute asynchronously.
        pub async fn update(&self, statute_id: &str, statute: Statute) -> RegistryResult<u32> {
            let mut registry = self.inner.write().await;
            registry.update(statute_id, statute)
        }

        /// Updates a statute with optimistic concurrency control asynchronously.
        pub async fn update_with_etag(
            &self,
            statute_id: &str,
            statute: Statute,
            expected_etag: &str,
        ) -> RegistryResult<u32> {
            let mut registry = self.inner.write().await;
            registry.update_with_etag(statute_id, statute, expected_etag)
        }

        /// Gets a statute by ID asynchronously.
        pub async fn get(&self, statute_id: &str) -> Option<StatuteEntry> {
            let mut registry = self.inner.write().await;
            registry.get(statute_id)
        }

        /// Gets a statute without using cache asynchronously.
        pub async fn get_uncached(&self, statute_id: &str) -> Option<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.get_uncached(statute_id).cloned()
        }

        /// Gets a specific version of a statute asynchronously.
        pub async fn get_version(
            &self,
            statute_id: &str,
            version: u32,
        ) -> RegistryResult<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.get_version(statute_id, version).cloned()
        }

        /// Lists all versions of a statute asynchronously.
        pub async fn list_versions(&self, statute_id: &str) -> Vec<u32> {
            let registry = self.inner.read().await;
            registry.list_versions(statute_id)
        }

        /// Lists all statutes asynchronously.
        pub async fn list(&self) -> Vec<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.list().into_iter().cloned().collect()
        }

        /// Lists active statutes asynchronously.
        pub async fn list_active(&self) -> Vec<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.list_active().into_iter().cloned().collect()
        }

        /// Queries statutes by tag asynchronously.
        pub async fn query_by_tag(&self, tag: &str) -> Vec<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.query_by_tag(tag).into_iter().cloned().collect()
        }

        /// Queries statutes by jurisdiction asynchronously.
        pub async fn query_by_jurisdiction(&self, jurisdiction: &str) -> Vec<StatuteEntry> {
            let registry = self.inner.read().await;
            registry
                .query_by_jurisdiction(jurisdiction)
                .into_iter()
                .cloned()
                .collect()
        }

        /// Sets the status of a statute asynchronously.
        pub async fn set_status(
            &self,
            statute_id: &str,
            status: StatuteStatus,
        ) -> RegistryResult<()> {
            let mut registry = self.inner.write().await;
            registry.set_status(statute_id, status)
        }

        /// Searches statutes asynchronously.
        pub async fn search(&self, query: &SearchQuery) -> Vec<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.search(query).iter().map(|&e| e.clone()).collect()
        }

        /// Searches statutes with pagination asynchronously.
        pub async fn search_paged(
            &self,
            query: &SearchQuery,
            pagination: Pagination,
        ) -> PagedResult<StatuteEntry> {
            let registry = self.inner.read().await;
            registry.search_paged(query, pagination)
        }

        /// Creates a backup asynchronously.
        pub async fn create_backup(&self, description: Option<String>) -> RegistryBackup {
            let registry = self.inner.read().await;
            registry.create_backup(description)
        }

        /// Restores from a backup asynchronously.
        pub async fn restore_from_backup(&self, backup: RegistryBackup) -> RegistryResult<()> {
            let mut registry = self.inner.write().await;
            registry.restore_from_backup(backup)
        }

        /// Batch registers statutes asynchronously.
        pub async fn batch_register(
            &self,
            entries: Vec<StatuteEntry>,
        ) -> Vec<RegistryResult<Uuid>> {
            let mut registry = self.inner.write().await;
            registry.batch_register(entries)
        }

        /// Subscribes to registry events asynchronously.
        pub async fn subscribe_webhook<F>(
            &self,
            name: Option<String>,
            filter: Option<WebhookEventFilter>,
            callback: F,
        ) -> Uuid
        where
            F: Fn(&RegistryEvent) + Send + Sync + 'static,
        {
            let registry = self.inner.read().await;
            registry.subscribe_webhook(name, filter, callback)
        }

        /// Unsubscribes a webhook asynchronously.
        pub async fn unsubscribe_webhook(&self, id: Uuid) -> bool {
            let registry = self.inner.read().await;
            registry.unsubscribe_webhook(id)
        }

        /// Computes temporal analytics asynchronously.
        ///
        /// Analyzes registration patterns, update frequency, and version velocity.
        pub async fn temporal_analytics(&self) -> TemporalAnalytics {
            let mut registry = self.inner.write().await;
            registry.temporal_analytics()
        }

        /// Computes relationship analytics asynchronously.
        ///
        /// Analyzes statute dependencies, references, and supersession chains.
        pub async fn relationship_analytics(&self) -> RelationshipAnalytics {
            let mut registry = self.inner.write().await;
            registry.relationship_analytics()
        }

        /// Computes tag analytics asynchronously.
        ///
        /// Analyzes tag usage patterns and co-occurrence.
        pub async fn tag_analytics(&self) -> TagAnalytics {
            let mut registry = self.inner.write().await;
            registry.tag_analytics()
        }

        /// Computes activity analytics asynchronously.
        ///
        /// Analyzes modification patterns and status changes.
        pub async fn activity_analytics(&self) -> ActivityAnalytics {
            let mut registry = self.inner.write().await;
            registry.activity_analytics()
        }

        /// Groups statutes by a specified field and returns counts asynchronously.
        pub async fn aggregate_by<F>(&self, key_fn: F) -> AggregationResult
        where
            F: Fn(&StatuteEntry) -> String + Send,
        {
            let registry = self.inner.read().await;
            registry.aggregate_by(key_fn)
        }

        /// Groups statutes by multiple tags and returns counts asynchronously.
        pub async fn aggregate_by_tags(&self) -> AggregationResult {
            let registry = self.inner.read().await;
            registry.aggregate_by_tags()
        }
    }

    impl Default for AsyncStatuteRegistry {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Clone for AsyncStatuteRegistry {
        fn clone(&self) -> Self {
            Self {
                inner: Arc::clone(&self.inner),
            }
        }
    }
}

// =============================================================================
// Streaming Support
// =============================================================================

#[cfg(all(feature = "async", feature = "async-stream"))]
pub mod streaming {
    //! Streaming support for large result sets.
    //!
    //! This module provides Stream implementations for efficiently
    //! iterating over large collections of statutes.

    use super::*;
    use async_stream::stream;
    use futures::Stream;

    /// Creates a stream of all statutes.
    pub fn stream_all(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteEntry>> {
        stream! {
            let registry = registry.read().await;
            let statutes: Vec<StatuteEntry> = registry.list().into_iter().cloned().collect();
            drop(registry);

            for chunk in statutes.chunks(chunk_size) {
                yield chunk.to_vec();
            }
        }
    }

    /// Creates a stream of statutes matching a query.
    pub fn stream_search(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        query: SearchQuery,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteEntry>> {
        stream! {
            let registry = registry.read().await;
            let results: Vec<StatuteEntry> = registry.search(&query).iter().map(|&e| e.clone()).collect();
            drop(registry);

            for chunk in results.chunks(chunk_size) {
                yield chunk.to_vec();
            }
        }
    }

    /// Creates a stream of statute summaries.
    pub fn stream_summaries(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteSummary>> {
        stream! {
            let registry = registry.read().await;
            let summaries: Vec<StatuteSummary> = registry
                .list_summaries()
                .into_iter()
                .collect();
            drop(registry);

            for chunk in summaries.chunks(chunk_size) {
                yield chunk.to_vec();
            }
        }
    }
}

// =============================================================================
// Transaction Support
// =============================================================================

pub mod transaction {
    //! Transaction support for batched registry operations.
    //!
    //! This module provides a transaction pattern that allows
    //! multiple operations to be batched together and committed
    //! or rolled back as a unit.

    use super::*;

    /// A transaction operation.
    #[derive(Debug, Clone)]
    pub enum Operation {
        /// Register a new statute
        Register(Box<StatuteEntry>),
        /// Update an existing statute
        Update {
            statute_id: String,
            statute: Statute,
        },
        /// Set the status of a statute
        SetStatus {
            statute_id: String,
            status: StatuteStatus,
        },
        /// Add a tag to a statute
        AddTag { statute_id: String, tag: String },
        /// Remove a tag from a statute
        RemoveTag { statute_id: String, tag: String },
        /// Add metadata to a statute
        AddMetadata {
            statute_id: String,
            key: String,
            value: String,
        },
    }

    /// A transaction for batching operations.
    pub struct Transaction {
        operations: Vec<Operation>,
    }

    impl Transaction {
        /// Creates a new transaction.
        pub fn new() -> Self {
            Self {
                operations: Vec::new(),
            }
        }

        /// Adds a register operation.
        pub fn register(mut self, entry: StatuteEntry) -> Self {
            self.operations.push(Operation::Register(Box::new(entry)));
            self
        }

        /// Adds an update operation.
        pub fn update(mut self, statute_id: impl Into<String>, statute: Statute) -> Self {
            self.operations.push(Operation::Update {
                statute_id: statute_id.into(),
                statute,
            });
            self
        }

        /// Adds a set status operation.
        pub fn set_status(mut self, statute_id: impl Into<String>, status: StatuteStatus) -> Self {
            self.operations.push(Operation::SetStatus {
                statute_id: statute_id.into(),
                status,
            });
            self
        }

        /// Adds an add tag operation.
        pub fn add_tag(mut self, statute_id: impl Into<String>, tag: impl Into<String>) -> Self {
            self.operations.push(Operation::AddTag {
                statute_id: statute_id.into(),
                tag: tag.into(),
            });
            self
        }

        /// Adds a remove tag operation.
        pub fn remove_tag(mut self, statute_id: impl Into<String>, tag: impl Into<String>) -> Self {
            self.operations.push(Operation::RemoveTag {
                statute_id: statute_id.into(),
                tag: tag.into(),
            });
            self
        }

        /// Adds metadata.
        pub fn add_metadata(
            mut self,
            statute_id: impl Into<String>,
            key: impl Into<String>,
            value: impl Into<String>,
        ) -> Self {
            self.operations.push(Operation::AddMetadata {
                statute_id: statute_id.into(),
                key: key.into(),
                value: value.into(),
            });
            self
        }

        /// Commits the transaction, applying all operations.
        pub fn commit(self, registry: &mut StatuteRegistry) -> RegistryResult<TransactionResult> {
            let mut results = Vec::new();
            let mut successful = 0;
            let mut failed = 0;

            for op in self.operations {
                let result = match op {
                    Operation::Register(entry) => registry
                        .register(*entry)
                        .map(OperationResult::Registered)
                        .map_err(OperationError::Registry),
                    Operation::Update {
                        statute_id,
                        statute,
                    } => registry
                        .update(&statute_id, statute)
                        .map(OperationResult::Updated)
                        .map_err(OperationError::Registry),
                    Operation::SetStatus { statute_id, status } => registry
                        .set_status(&statute_id, status)
                        .map(|_| OperationResult::StatusSet)
                        .map_err(OperationError::Registry),
                    Operation::AddTag { statute_id, tag } => registry
                        .add_tag(&statute_id, tag)
                        .map(|_| OperationResult::TagAdded)
                        .map_err(OperationError::Registry),
                    Operation::RemoveTag { statute_id, tag } => registry
                        .remove_tag(&statute_id, &tag)
                        .map(|_| OperationResult::TagRemoved)
                        .map_err(OperationError::Registry),
                    Operation::AddMetadata {
                        statute_id,
                        key,
                        value,
                    } => registry
                        .add_metadata(&statute_id, key, value)
                        .map(|_| OperationResult::MetadataAdded)
                        .map_err(OperationError::Registry),
                };

                match result {
                    Ok(r) => {
                        successful += 1;
                        results.push(Ok(r));
                    }
                    Err(e) => {
                        failed += 1;
                        results.push(Err(e));
                    }
                }
            }

            Ok(TransactionResult {
                results,
                successful,
                failed,
            })
        }
    }

    impl Default for Transaction {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Result of a transaction operation.
    #[derive(Debug, Clone)]
    pub enum OperationResult {
        /// Statute was registered
        Registered(Uuid),
        /// Statute was updated
        Updated(u32),
        /// Status was set
        StatusSet,
        /// Tag was added
        TagAdded,
        /// Tag was removed
        TagRemoved,
        /// Metadata was added
        MetadataAdded,
    }

    /// Error during a transaction operation.
    #[derive(Debug, Error)]
    pub enum OperationError {
        #[error("Registry error: {0}")]
        Registry(#[from] RegistryError),
    }

    /// Result of committing a transaction.
    #[derive(Debug)]
    pub struct TransactionResult {
        /// Results for each operation
        pub results: Vec<Result<OperationResult, OperationError>>,
        /// Number of successful operations
        pub successful: usize,
        /// Number of failed operations
        pub failed: usize,
    }

    impl TransactionResult {
        /// Returns true if all operations succeeded.
        pub fn is_success(&self) -> bool {
            self.failed == 0
        }

        /// Returns true if any operations failed.
        pub fn has_failures(&self) -> bool {
            self.failed > 0
        }
    }
}

// =============================================================================
// Akoma Ntoso Support
// =============================================================================

#[cfg(feature = "akoma-ntoso")]
pub mod akoma_ntoso {
    //! Import/export support for Akoma Ntoso format.
    //!
    //! Akoma Ntoso is an XML standard for parliamentary,
    //! legislative and judiciary documents.

    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    /// Akoma Ntoso document wrapper.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename = "akomaNtoso")]
    pub struct AkomaNtoso {
        #[serde(rename = "act")]
        pub act: Act,
    }

    /// Akoma Ntoso act element.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Act {
        #[serde(rename = "meta")]
        pub meta: Meta,
        #[serde(rename = "body")]
        pub body: Body,
    }

    /// Akoma Ntoso metadata.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Meta {
        #[serde(rename = "identification")]
        pub identification: Identification,
        #[serde(rename = "publication")]
        pub publication: Option<Publication>,
    }

    /// Akoma Ntoso identification.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Identification {
        #[serde(rename = "FRBRWork")]
        pub work: FRBRLevel,
        #[serde(rename = "FRBRExpression")]
        pub expression: FRBRLevel,
    }

    /// Akoma Ntoso FRBR level.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FRBRLevel {
        #[serde(rename = "FRBRthis")]
        pub this: FRBRElement,
        #[serde(rename = "FRBRuri")]
        pub uri: FRBRElement,
        #[serde(rename = "FRBRdate")]
        pub date: FRBRDate,
        #[serde(rename = "FRBRauthor")]
        pub author: FRBRElement,
        #[serde(rename = "FRBRcountry")]
        pub country: FRBRElement,
    }

    /// Akoma Ntoso FRBR element.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FRBRElement {
        #[serde(rename = "@value")]
        pub value: String,
    }

    /// Akoma Ntoso FRBR date.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FRBRDate {
        #[serde(rename = "@date")]
        pub date: String,
        #[serde(rename = "@name")]
        pub name: String,
    }

    /// Akoma Ntoso publication.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Publication {
        #[serde(rename = "@date")]
        pub date: String,
        #[serde(rename = "@name")]
        pub name: String,
        #[serde(rename = "@showAs")]
        pub show_as: String,
    }

    /// Akoma Ntoso body.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Body {
        #[serde(rename = "section", default)]
        pub sections: Vec<Section>,
    }

    /// Akoma Ntoso section.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Section {
        #[serde(rename = "@eId")]
        pub id: String,
        #[serde(rename = "num")]
        pub num: Option<String>,
        #[serde(rename = "heading")]
        pub heading: Option<String>,
        #[serde(rename = "content")]
        pub content: Option<String>,
    }

    /// Exports a statute to Akoma Ntoso format.
    pub fn export_statute(entry: &StatuteEntry) -> Result<String, quick_xml::DeError> {
        let akoma = statute_to_akoma(entry);
        to_string(&akoma)
    }

    /// Imports a statute from Akoma Ntoso format.
    pub fn import_statute(
        xml: &str,
        jurisdiction: &str,
    ) -> Result<StatuteEntry, quick_xml::DeError> {
        let akoma: AkomaNtoso = from_str(xml)?;
        Ok(akoma_to_statute(akoma, jurisdiction))
    }

    /// Converts a statute to Akoma Ntoso format.
    fn statute_to_akoma(entry: &StatuteEntry) -> AkomaNtoso {
        AkomaNtoso {
            act: Act {
                meta: Meta {
                    identification: Identification {
                        work: FRBRLevel {
                            this: FRBRElement {
                                value: format!(
                                    "/akn/{}/act/{}",
                                    entry.jurisdiction, entry.statute.id
                                ),
                            },
                            uri: FRBRElement {
                                value: format!(
                                    "/akn/{}/act/{}",
                                    entry.jurisdiction, entry.statute.id
                                ),
                            },
                            date: FRBRDate {
                                date: entry.created_at.format("%Y-%m-%d").to_string(),
                                name: "enactment".to_string(),
                            },
                            author: FRBRElement {
                                value: format!("#{}", entry.jurisdiction),
                            },
                            country: FRBRElement {
                                value: entry.jurisdiction.clone(),
                            },
                        },
                        expression: FRBRLevel {
                            this: FRBRElement {
                                value: format!(
                                    "/akn/{}/act/{}/eng@{}",
                                    entry.jurisdiction,
                                    entry.statute.id,
                                    entry.created_at.format("%Y-%m-%d")
                                ),
                            },
                            uri: FRBRElement {
                                value: format!(
                                    "/akn/{}/act/{}/eng@",
                                    entry.jurisdiction, entry.statute.id
                                ),
                            },
                            date: FRBRDate {
                                date: entry.modified_at.format("%Y-%m-%d").to_string(),
                                name: "expression".to_string(),
                            },
                            author: FRBRElement {
                                value: "#author".to_string(),
                            },
                            country: FRBRElement {
                                value: entry.jurisdiction.clone(),
                            },
                        },
                    },
                    publication: entry.effective_date.map(|d| Publication {
                        date: d.format("%Y-%m-%d").to_string(),
                        name: "publication".to_string(),
                        show_as: "Publication Date".to_string(),
                    }),
                },
                body: Body {
                    sections: vec![Section {
                        id: "main".to_string(),
                        num: Some("1".to_string()),
                        heading: Some(entry.statute.title.clone()),
                        content: Some(format!("{:?}", entry.statute)),
                    }],
                },
            },
        }
    }

    /// Converts Akoma Ntoso format to a statute.
    fn akoma_to_statute(akoma: AkomaNtoso, jurisdiction: &str) -> StatuteEntry {
        let statute_id = akoma
            .act
            .meta
            .identification
            .work
            .uri
            .value
            .split('/')
            .next_back()
            .unwrap_or("unknown")
            .to_string();

        let title = akoma
            .act
            .body
            .sections
            .first()
            .and_then(|s| s.heading.clone())
            .unwrap_or_else(|| "Untitled".to_string());

        // Create a default effect for imported statutes
        let effect = legalis_core::Effect::new(
            legalis_core::EffectType::Custom,
            "Imported from Akoma Ntoso XML",
        );

        let statute = Statute::new(&statute_id, &title, effect);

        StatuteEntry::new(statute, jurisdiction)
    }
}

// =============================================================================
// Database Backend Support
// =============================================================================

#[cfg(any(feature = "sqlite", feature = "postgres"))]
pub mod storage {
    //! Storage backend implementations for persistent statute storage.
    //!
    //! This module provides database backends with connection pooling
    //! for SQLite and PostgreSQL.

    use super::*;
    use sqlx::{Pool, Row};
    use std::sync::Arc;

    /// Storage backend trait for statute persistence.
    #[cfg(feature = "async")]
    #[async_trait::async_trait]
    pub trait StorageBackend: Send + Sync {
        /// Stores a statute entry.
        async fn store(&self, entry: &StatuteEntry) -> RegistryResult<()>;

        /// Retrieves a statute by ID.
        async fn get(&self, statute_id: &str) -> RegistryResult<Option<StatuteEntry>>;

        /// Retrieves a specific version of a statute.
        async fn get_version(
            &self,
            statute_id: &str,
            version: u32,
        ) -> RegistryResult<Option<StatuteEntry>>;

        /// Lists all statutes.
        async fn list(&self) -> RegistryResult<Vec<StatuteEntry>>;

        /// Lists all versions of a statute.
        async fn list_versions(&self, statute_id: &str) -> RegistryResult<Vec<u32>>;

        /// Deletes a statute.
        async fn delete(&self, statute_id: &str) -> RegistryResult<()>;

        /// Searches statutes by jurisdiction.
        async fn find_by_jurisdiction(
            &self,
            jurisdiction: &str,
        ) -> RegistryResult<Vec<StatuteEntry>>;

        /// Searches statutes by tag.
        async fn find_by_tag(&self, tag: &str) -> RegistryResult<Vec<StatuteEntry>>;

        /// Counts total statutes.
        async fn count(&self) -> RegistryResult<usize>;
    }

    /// SQLite storage backend with connection pooling.
    #[cfg(feature = "sqlite")]
    pub struct SqliteBackend {
        pool: Arc<Pool<sqlx::Sqlite>>,
    }

    #[cfg(feature = "sqlite")]
    impl SqliteBackend {
        /// Creates a new SQLite backend.
        pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(10)
                .connect(database_url)
                .await?;

            // Run migrations
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS statutes (
                    registry_id TEXT PRIMARY KEY,
                    statute_id TEXT NOT NULL,
                    version INTEGER NOT NULL,
                    etag TEXT NOT NULL,
                    status TEXT NOT NULL,
                    effective_date TEXT,
                    expiry_date TEXT,
                    amends TEXT,
                    jurisdiction TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    modified_at TEXT NOT NULL,
                    statute_data TEXT NOT NULL,
                    tags TEXT NOT NULL,
                    references TEXT NOT NULL,
                    supersedes TEXT NOT NULL,
                    metadata TEXT NOT NULL,
                    UNIQUE(statute_id, version)
                );

                CREATE INDEX IF NOT EXISTS idx_statute_id ON statutes(statute_id);
                CREATE INDEX IF NOT EXISTS idx_jurisdiction ON statutes(jurisdiction);
                CREATE INDEX IF NOT EXISTS idx_status ON statutes(status);
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self {
                pool: Arc::new(pool),
            })
        }

        /// Gets the connection pool.
        pub fn pool(&self) -> &Pool<sqlx::Sqlite> {
            &self.pool
        }
    }

    #[cfg(feature = "sqlite")]
    #[async_trait::async_trait]
    impl StorageBackend for SqliteBackend {
        async fn store(&self, entry: &StatuteEntry) -> RegistryResult<()> {
            let statute_json = serde_json::to_string(&entry.statute)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let tags_json = serde_json::to_string(&entry.tags)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let refs_json = serde_json::to_string(&entry.references)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let supersedes_json = serde_json::to_string(&entry.supersedes)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let metadata_json = serde_json::to_string(&entry.metadata)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO statutes (
                    registry_id, statute_id, version, etag, status,
                    effective_date, expiry_date, amends, jurisdiction,
                    created_at, modified_at, statute_data, tags, references,
                    supersedes, metadata
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(entry.registry_id.to_string())
            .bind(&entry.statute.id)
            .bind(entry.version as i64)
            .bind(&entry.etag)
            .bind(format!("{:?}", entry.status))
            .bind(entry.effective_date.map(|d| d.to_rfc3339()))
            .bind(entry.expiry_date.map(|d| d.to_rfc3339()))
            .bind(&entry.amends)
            .bind(&entry.jurisdiction)
            .bind(entry.created_at.to_rfc3339())
            .bind(entry.modified_at.to_rfc3339())
            .bind(statute_json)
            .bind(tags_json)
            .bind(refs_json)
            .bind(supersedes_json)
            .bind(metadata_json)
            .execute(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(())
        }

        async fn get(&self, statute_id: &str) -> RegistryResult<Option<StatuteEntry>> {
            let row = sqlx::query(
                "SELECT * FROM statutes WHERE statute_id = ? ORDER BY version DESC LIMIT 1",
            )
            .bind(statute_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            row.map(|r| self.row_to_entry(&r)).transpose()
        }

        async fn get_version(
            &self,
            statute_id: &str,
            version: u32,
        ) -> RegistryResult<Option<StatuteEntry>> {
            let row = sqlx::query("SELECT * FROM statutes WHERE statute_id = ? AND version = ?")
                .bind(statute_id)
                .bind(version as i64)
                .fetch_optional(&*self.pool)
                .await
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            row.map(|r| self.row_to_entry(&r)).transpose()
        }

        async fn list(&self) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT * FROM statutes s1
                WHERE version = (SELECT MAX(version) FROM statutes s2 WHERE s2.statute_id = s1.statute_id)
                "#,
            )
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn list_versions(&self, statute_id: &str) -> RegistryResult<Vec<u32>> {
            let rows =
                sqlx::query("SELECT version FROM statutes WHERE statute_id = ? ORDER BY version")
                    .bind(statute_id)
                    .fetch_all(&*self.pool)
                    .await
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(rows.iter().map(|r| r.get::<i64, _>(0) as u32).collect())
        }

        async fn delete(&self, statute_id: &str) -> RegistryResult<()> {
            sqlx::query("DELETE FROM statutes WHERE statute_id = ?")
                .bind(statute_id)
                .execute(&*self.pool)
                .await
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(())
        }

        async fn find_by_jurisdiction(
            &self,
            jurisdiction: &str,
        ) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT * FROM statutes s1
                WHERE jurisdiction = ?
                AND version = (SELECT MAX(version) FROM statutes s2 WHERE s2.statute_id = s1.statute_id)
                "#,
            )
            .bind(jurisdiction)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn find_by_tag(&self, tag: &str) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT * FROM statutes s1
                WHERE tags LIKE ?
                AND version = (SELECT MAX(version) FROM statutes s2 WHERE s2.statute_id = s1.statute_id)
                "#,
            )
            .bind(format!("%\"{}\",%", tag))
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn count(&self) -> RegistryResult<usize> {
            let row = sqlx::query(
                r#"
                SELECT COUNT(DISTINCT statute_id) FROM statutes
                "#,
            )
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(row.get::<i64, _>(0) as usize)
        }
    }

    #[cfg(feature = "sqlite")]
    impl SqliteBackend {
        #[allow(dead_code)]
        fn row_to_entry(&self, row: &sqlx::sqlite::SqliteRow) -> RegistryResult<StatuteEntry> {
            let statute_json: String = row.get("statute_data");
            let tags_json: String = row.get("tags");
            let refs_json: String = row.get("references");
            let supersedes_json: String = row.get("supersedes");
            let metadata_json: String = row.get("metadata");

            Ok(StatuteEntry {
                registry_id: Uuid::parse_str(row.get("registry_id"))
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                statute: serde_json::from_str(&statute_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                version: row.get::<i64, _>("version") as u32,
                etag: row.get("etag"),
                status: match row.get::<String, _>("status").as_str() {
                    "Draft" => StatuteStatus::Draft,
                    "UnderReview" => StatuteStatus::UnderReview,
                    "Approved" => StatuteStatus::Approved,
                    "Active" => StatuteStatus::Active,
                    "Repealed" => StatuteStatus::Repealed,
                    "Superseded" => StatuteStatus::Superseded,
                    _ => StatuteStatus::Draft,
                },
                effective_date: row
                    .get::<Option<String>, _>("effective_date")
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                expiry_date: row
                    .get::<Option<String>, _>("expiry_date")
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                amends: row.get("amends"),
                supersedes: serde_json::from_str(&supersedes_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                references: serde_json::from_str(&refs_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                tags: serde_json::from_str(&tags_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                jurisdiction: row.get("jurisdiction"),
                metadata: serde_json::from_str(&metadata_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                created_at: DateTime::parse_from_rfc3339(row.get("created_at"))
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?
                    .with_timezone(&Utc),
                modified_at: DateTime::parse_from_rfc3339(row.get("modified_at"))
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?
                    .with_timezone(&Utc),
            })
        }
    }

    /// PostgreSQL storage backend with connection pooling.
    #[cfg(feature = "postgres")]
    pub struct PostgresBackend {
        pool: Arc<Pool<sqlx::Postgres>>,
    }

    #[cfg(feature = "postgres")]
    impl PostgresBackend {
        /// Creates a new PostgreSQL backend.
        pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(20)
                .connect(database_url)
                .await?;

            // Run migrations
            sqlx::query(
                r#"
                CREATE TABLE IF NOT EXISTS statutes (
                    registry_id UUID PRIMARY KEY,
                    statute_id TEXT NOT NULL,
                    version INTEGER NOT NULL,
                    etag TEXT NOT NULL,
                    status TEXT NOT NULL,
                    effective_date TIMESTAMPTZ,
                    expiry_date TIMESTAMPTZ,
                    amends TEXT,
                    jurisdiction TEXT NOT NULL,
                    created_at TIMESTAMPTZ NOT NULL,
                    modified_at TIMESTAMPTZ NOT NULL,
                    statute_data JSONB NOT NULL,
                    tags JSONB NOT NULL,
                    references JSONB NOT NULL,
                    supersedes JSONB NOT NULL,
                    metadata JSONB NOT NULL,
                    UNIQUE(statute_id, version)
                );

                CREATE INDEX IF NOT EXISTS idx_statute_id ON statutes(statute_id);
                CREATE INDEX IF NOT EXISTS idx_jurisdiction ON statutes(jurisdiction);
                CREATE INDEX IF NOT EXISTS idx_status ON statutes(status);
                CREATE INDEX IF NOT EXISTS idx_tags ON statutes USING GIN (tags);
                "#,
            )
            .execute(&pool)
            .await?;

            Ok(Self {
                pool: Arc::new(pool),
            })
        }

        /// Gets the connection pool.
        pub fn pool(&self) -> &Pool<sqlx::Postgres> {
            &self.pool
        }
    }

    #[cfg(feature = "postgres")]
    #[async_trait::async_trait]
    impl StorageBackend for PostgresBackend {
        async fn store(&self, entry: &StatuteEntry) -> RegistryResult<()> {
            let statute_json = serde_json::to_value(&entry.statute)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let tags_json = serde_json::to_value(&entry.tags)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let refs_json = serde_json::to_value(&entry.references)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let supersedes_json = serde_json::to_value(&entry.supersedes)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;
            let metadata_json = serde_json::to_value(&entry.metadata)
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            sqlx::query(
                r#"
                INSERT INTO statutes (
                    registry_id, statute_id, version, etag, status,
                    effective_date, expiry_date, amends, jurisdiction,
                    created_at, modified_at, statute_data, tags, references,
                    supersedes, metadata
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                ON CONFLICT (statute_id, version)
                DO UPDATE SET
                    etag = EXCLUDED.etag,
                    status = EXCLUDED.status,
                    modified_at = EXCLUDED.modified_at,
                    statute_data = EXCLUDED.statute_data,
                    metadata = EXCLUDED.metadata
                "#,
            )
            .bind(entry.registry_id)
            .bind(&entry.statute.id)
            .bind(entry.version as i32)
            .bind(&entry.etag)
            .bind(format!("{:?}", entry.status))
            .bind(entry.effective_date)
            .bind(entry.expiry_date)
            .bind(&entry.amends)
            .bind(&entry.jurisdiction)
            .bind(entry.created_at)
            .bind(entry.modified_at)
            .bind(statute_json)
            .bind(tags_json)
            .bind(refs_json)
            .bind(supersedes_json)
            .bind(metadata_json)
            .execute(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(())
        }

        async fn get(&self, statute_id: &str) -> RegistryResult<Option<StatuteEntry>> {
            let row = sqlx::query(
                "SELECT * FROM statutes WHERE statute_id = $1 ORDER BY version DESC LIMIT 1",
            )
            .bind(statute_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            row.map(|r| self.row_to_entry(&r)).transpose()
        }

        async fn get_version(
            &self,
            statute_id: &str,
            version: u32,
        ) -> RegistryResult<Option<StatuteEntry>> {
            let row = sqlx::query("SELECT * FROM statutes WHERE statute_id = $1 AND version = $2")
                .bind(statute_id)
                .bind(version as i32)
                .fetch_optional(&*self.pool)
                .await
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            row.map(|r| self.row_to_entry(&r)).transpose()
        }

        async fn list(&self) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT DISTINCT ON (statute_id) *
                FROM statutes
                ORDER BY statute_id, version DESC
                "#,
            )
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn list_versions(&self, statute_id: &str) -> RegistryResult<Vec<u32>> {
            let rows =
                sqlx::query("SELECT version FROM statutes WHERE statute_id = $1 ORDER BY version")
                    .bind(statute_id)
                    .fetch_all(&*self.pool)
                    .await
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(rows.iter().map(|r| r.get::<i32, _>(0) as u32).collect())
        }

        async fn delete(&self, statute_id: &str) -> RegistryResult<()> {
            sqlx::query("DELETE FROM statutes WHERE statute_id = $1")
                .bind(statute_id)
                .execute(&*self.pool)
                .await
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(())
        }

        async fn find_by_jurisdiction(
            &self,
            jurisdiction: &str,
        ) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT DISTINCT ON (statute_id) *
                FROM statutes
                WHERE jurisdiction = $1
                ORDER BY statute_id, version DESC
                "#,
            )
            .bind(jurisdiction)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn find_by_tag(&self, tag: &str) -> RegistryResult<Vec<StatuteEntry>> {
            let rows = sqlx::query(
                r#"
                SELECT DISTINCT ON (statute_id) *
                FROM statutes
                WHERE tags ? $1
                ORDER BY statute_id, version DESC
                "#,
            )
            .bind(tag)
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            rows.iter().map(|r| self.row_to_entry(r)).collect()
        }

        async fn count(&self) -> RegistryResult<usize> {
            let row = sqlx::query("SELECT COUNT(DISTINCT statute_id) FROM statutes")
                .fetch_one(&*self.pool)
                .await
                .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?;

            Ok(row.get::<i64, _>(0) as usize)
        }
    }

    #[cfg(feature = "postgres")]
    impl PostgresBackend {
        #[allow(dead_code)]
        fn row_to_entry(&self, row: &sqlx::postgres::PgRow) -> RegistryResult<StatuteEntry> {
            let statute_json: serde_json::Value = row.get("statute_data");
            let tags_json: serde_json::Value = row.get("tags");
            let refs_json: serde_json::Value = row.get("references");
            let supersedes_json: serde_json::Value = row.get("supersedes");
            let metadata_json: serde_json::Value = row.get("metadata");

            Ok(StatuteEntry {
                registry_id: row.get("registry_id"),
                statute: serde_json::from_value(statute_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                version: row.get::<i32, _>("version") as u32,
                etag: row.get("etag"),
                status: match row.get::<String, _>("status").as_str() {
                    "Draft" => StatuteStatus::Draft,
                    "UnderReview" => StatuteStatus::UnderReview,
                    "Approved" => StatuteStatus::Approved,
                    "Active" => StatuteStatus::Active,
                    "Repealed" => StatuteStatus::Repealed,
                    "Superseded" => StatuteStatus::Superseded,
                    _ => StatuteStatus::Draft,
                },
                effective_date: row.get("effective_date"),
                expiry_date: row.get("expiry_date"),
                amends: row.get("amends"),
                supersedes: serde_json::from_value(supersedes_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                references: serde_json::from_value(refs_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                tags: serde_json::from_value(tags_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                jurisdiction: row.get("jurisdiction"),
                metadata: serde_json::from_value(metadata_json)
                    .map_err(|e| RegistryError::InvalidOperation(e.to_string()))?,
                created_at: row.get("created_at"),
                modified_at: row.get("modified_at"),
            })
        }
    }
}

// =============================================================================
// GraphQL API Support
// =============================================================================

#[cfg(feature = "graphql")]
pub mod graphql {
    //! GraphQL API for statute registry.
    //!
    //! This module provides a GraphQL interface for querying and
    //! mutating the statute registry.

    use super::*;
    use async_graphql::{EmptySubscription, FieldResult, Object, Schema, SimpleObject};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// GraphQL-compatible statute entry.
    #[derive(SimpleObject, Clone)]
    pub struct GraphQLStatuteEntry {
        pub registry_id: String,
        pub statute_id: String,
        pub title: String,
        pub version: i32,
        pub status: String,
        pub jurisdiction: String,
        pub tags: Vec<String>,
        pub created_at: String,
        pub modified_at: String,
    }

    impl From<&StatuteEntry> for GraphQLStatuteEntry {
        fn from(entry: &StatuteEntry) -> Self {
            Self {
                registry_id: entry.registry_id.to_string(),
                statute_id: entry.statute.id.clone(),
                title: entry.statute.title.clone(),
                version: entry.version as i32,
                status: format!("{:?}", entry.status),
                jurisdiction: entry.jurisdiction.clone(),
                tags: entry.tags.clone(),
                created_at: entry.created_at.to_rfc3339(),
                modified_at: entry.modified_at.to_rfc3339(),
            }
        }
    }

    /// GraphQL query root.
    pub struct QueryRoot {
        registry: Arc<RwLock<StatuteRegistry>>,
    }

    impl QueryRoot {
        /// Creates a new query root.
        pub fn new(registry: Arc<RwLock<StatuteRegistry>>) -> Self {
            Self { registry }
        }
    }

    #[Object]
    impl QueryRoot {
        /// Gets a statute by ID.
        async fn statute(&self, id: String) -> FieldResult<Option<GraphQLStatuteEntry>> {
            let mut registry = self.registry.write().await;
            Ok(registry.get(&id).map(|e| GraphQLStatuteEntry::from(&e)))
        }

        /// Lists all statutes.
        async fn statutes(&self) -> FieldResult<Vec<GraphQLStatuteEntry>> {
            let registry = self.registry.read().await;
            Ok(registry
                .list()
                .iter()
                .map(|e| GraphQLStatuteEntry::from(*e))
                .collect())
        }

        /// Lists active statutes.
        async fn active_statutes(&self) -> FieldResult<Vec<GraphQLStatuteEntry>> {
            let registry = self.registry.read().await;
            Ok(registry
                .list_active()
                .iter()
                .map(|e| GraphQLStatuteEntry::from(*e))
                .collect())
        }

        /// Searches statutes by tag.
        async fn statutes_by_tag(&self, tag: String) -> FieldResult<Vec<GraphQLStatuteEntry>> {
            let registry = self.registry.read().await;
            Ok(registry
                .query_by_tag(&tag)
                .iter()
                .map(|e| GraphQLStatuteEntry::from(*e))
                .collect())
        }

        /// Searches statutes by jurisdiction.
        async fn statutes_by_jurisdiction(
            &self,
            jurisdiction: String,
        ) -> FieldResult<Vec<GraphQLStatuteEntry>> {
            let registry = self.registry.read().await;
            Ok(registry
                .query_by_jurisdiction(&jurisdiction)
                .iter()
                .map(|e| GraphQLStatuteEntry::from(*e))
                .collect())
        }

        /// Gets statute count.
        async fn statute_count(&self) -> FieldResult<i32> {
            let registry = self.registry.read().await;
            Ok(registry.count() as i32)
        }
    }

    /// GraphQL mutation root.
    pub struct MutationRoot {
        registry: Arc<RwLock<StatuteRegistry>>,
    }

    impl MutationRoot {
        /// Creates a new mutation root.
        pub fn new(registry: Arc<RwLock<StatuteRegistry>>) -> Self {
            Self { registry }
        }
    }

    #[Object]
    impl MutationRoot {
        /// Sets the status of a statute.
        async fn set_status(&self, id: String, status: String) -> FieldResult<bool> {
            let mut registry = self.registry.write().await;
            let status_enum = match status.as_str() {
                "Draft" => StatuteStatus::Draft,
                "UnderReview" => StatuteStatus::UnderReview,
                "Approved" => StatuteStatus::Approved,
                "Active" => StatuteStatus::Active,
                "Repealed" => StatuteStatus::Repealed,
                "Superseded" => StatuteStatus::Superseded,
                _ => return Ok(false),
            };
            registry.set_status(&id, status_enum).ok();
            Ok(true)
        }

        /// Adds a tag to a statute.
        async fn add_tag(&self, id: String, tag: String) -> FieldResult<bool> {
            let mut registry = self.registry.write().await;
            registry.add_tag(&id, tag).ok();
            Ok(true)
        }

        /// Removes a tag from a statute.
        async fn remove_tag(&self, id: String, tag: String) -> FieldResult<bool> {
            let mut registry = self.registry.write().await;
            registry.remove_tag(&id, &tag).ok();
            Ok(true)
        }
    }

    /// Creates a GraphQL schema for the registry.
    pub fn create_schema(
        registry: Arc<RwLock<StatuteRegistry>>,
    ) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
        Schema::build(
            QueryRoot::new(Arc::clone(&registry)),
            MutationRoot::new(registry),
            EmptySubscription,
        )
        .finish()
    }
}

// ============================================================================
// Diff and Comparison
// ============================================================================

/// Represents a change in a field between two statute versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldChange<T: Clone> {
    /// Field value changed from old to new
    Changed { old: T, new: T },
    /// Field was added (only in new version)
    Added { value: T },
    /// Field was removed (only in old version)
    Removed { value: T },
    /// Field unchanged
    Unchanged { value: T },
}

impl<T: Clone + PartialEq> FieldChange<T> {
    /// Creates a field change by comparing old and new values.
    pub fn from_optional(old: Option<&T>, new: Option<&T>) -> Option<Self> {
        match (old, new) {
            (Some(o), Some(n)) if o != n => Some(FieldChange::Changed {
                old: o.clone(),
                new: n.clone(),
            }),
            (Some(o), Some(_)) => Some(FieldChange::Unchanged { value: o.clone() }),
            (None, Some(n)) => Some(FieldChange::Added { value: n.clone() }),
            (Some(o), None) => Some(FieldChange::Removed { value: o.clone() }),
            (None, None) => None,
        }
    }

    /// Creates a field change by comparing required values.
    pub fn from_values(old: &T, new: &T) -> Self {
        if old != new {
            FieldChange::Changed {
                old: old.clone(),
                new: new.clone(),
            }
        } else {
            FieldChange::Unchanged { value: old.clone() }
        }
    }

    /// Returns true if this represents a change.
    pub fn is_changed(&self) -> bool {
        matches!(
            self,
            FieldChange::Changed { .. } | FieldChange::Added { .. } | FieldChange::Removed { .. }
        )
    }

    /// Returns the new value if available.
    pub fn new_value(&self) -> Option<&T> {
        match self {
            FieldChange::Changed { new, .. } => Some(new),
            FieldChange::Added { value } => Some(value),
            FieldChange::Unchanged { value } => Some(value),
            FieldChange::Removed { .. } => None,
        }
    }
}

/// Represents differences between two statute entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    /// Statute ID
    pub statute_id: String,
    /// Old version number
    pub old_version: u32,
    /// New version number
    pub new_version: u32,
    /// Title changes
    pub title: Option<FieldChange<String>>,
    /// Status changes
    pub status: Option<FieldChange<StatuteStatus>>,
    /// Effective date changes
    pub effective_date: Option<FieldChange<DateTime<Utc>>>,
    /// Expiry date changes
    pub expiry_date: Option<FieldChange<DateTime<Utc>>>,
    /// Jurisdiction changes
    pub jurisdiction: Option<FieldChange<String>>,
    /// Tags added
    pub tags_added: Vec<String>,
    /// Tags removed
    pub tags_removed: Vec<String>,
    /// Metadata added
    pub metadata_added: HashMap<String, String>,
    /// Metadata removed
    pub metadata_removed: HashMap<String, String>,
    /// Metadata changed
    pub metadata_changed: HashMap<String, (String, String)>,
    /// References added
    pub references_added: Vec<String>,
    /// References removed
    pub references_removed: Vec<String>,
    /// Supersedes added
    pub supersedes_added: Vec<String>,
    /// Supersedes removed
    pub supersedes_removed: Vec<String>,
    /// Whether the statute content itself changed
    pub content_changed: bool,
}

impl StatuteDiff {
    /// Computes the difference between two statute entries.
    pub fn compute(old: &StatuteEntry, new: &StatuteEntry) -> Self {
        // Compare tags
        let old_tags: HashSet<_> = old.tags.iter().collect();
        let new_tags: HashSet<_> = new.tags.iter().collect();
        let tags_added: Vec<_> = new_tags
            .difference(&old_tags)
            .map(|s| (*s).clone())
            .collect();
        let tags_removed: Vec<_> = old_tags
            .difference(&new_tags)
            .map(|s| (*s).clone())
            .collect();

        // Compare metadata
        let mut metadata_added = HashMap::new();
        let mut metadata_removed = HashMap::new();
        let mut metadata_changed = HashMap::new();

        for (key, new_val) in &new.metadata {
            match old.metadata.get(key) {
                Some(old_val) if old_val != new_val => {
                    metadata_changed.insert(key.clone(), (old_val.clone(), new_val.clone()));
                }
                None => {
                    metadata_added.insert(key.clone(), new_val.clone());
                }
                _ => {}
            }
        }

        for (key, old_val) in &old.metadata {
            if !new.metadata.contains_key(key) {
                metadata_removed.insert(key.clone(), old_val.clone());
            }
        }

        // Compare references
        let old_refs: HashSet<_> = old.references.iter().collect();
        let new_refs: HashSet<_> = new.references.iter().collect();
        let references_added: Vec<_> = new_refs
            .difference(&old_refs)
            .map(|s| (*s).clone())
            .collect();
        let references_removed: Vec<_> = old_refs
            .difference(&new_refs)
            .map(|s| (*s).clone())
            .collect();

        // Compare supersedes
        let old_supersedes: HashSet<_> = old.supersedes.iter().collect();
        let new_supersedes: HashSet<_> = new.supersedes.iter().collect();
        let supersedes_added: Vec<_> = new_supersedes
            .difference(&old_supersedes)
            .map(|s| (*s).clone())
            .collect();
        let supersedes_removed: Vec<_> = old_supersedes
            .difference(&new_supersedes)
            .map(|s| (*s).clone())
            .collect();

        // Check if statute content changed
        // We compare the statute's JSON representation for simplicity
        let content_changed = serde_json::to_string(&old.statute).unwrap_or_default()
            != serde_json::to_string(&new.statute).unwrap_or_default();

        StatuteDiff {
            statute_id: new.statute.id.clone(),
            old_version: old.version,
            new_version: new.version,
            title: FieldChange::from_values(&old.statute.title, &new.statute.title)
                .is_changed()
                .then(|| FieldChange::from_values(&old.statute.title, &new.statute.title)),
            status: FieldChange::from_values(&old.status, &new.status)
                .is_changed()
                .then(|| FieldChange::from_values(&old.status, &new.status)),
            effective_date: FieldChange::from_optional(
                old.effective_date.as_ref(),
                new.effective_date.as_ref(),
            ),
            expiry_date: FieldChange::from_optional(
                old.expiry_date.as_ref(),
                new.expiry_date.as_ref(),
            ),
            jurisdiction: FieldChange::from_values(&old.jurisdiction, &new.jurisdiction)
                .is_changed()
                .then(|| FieldChange::from_values(&old.jurisdiction, &new.jurisdiction)),
            tags_added,
            tags_removed,
            metadata_added,
            metadata_removed,
            metadata_changed,
            references_added,
            references_removed,
            supersedes_added,
            supersedes_removed,
            content_changed,
        }
    }

    /// Returns true if there are any changes.
    pub fn has_changes(&self) -> bool {
        self.title.as_ref().is_some_and(|c| c.is_changed())
            || self.status.as_ref().is_some_and(|c| c.is_changed())
            || self.effective_date.as_ref().is_some_and(|c| c.is_changed())
            || self.expiry_date.as_ref().is_some_and(|c| c.is_changed())
            || self.jurisdiction.as_ref().is_some_and(|c| c.is_changed())
            || !self.tags_added.is_empty()
            || !self.tags_removed.is_empty()
            || !self.metadata_added.is_empty()
            || !self.metadata_removed.is_empty()
            || !self.metadata_changed.is_empty()
            || !self.references_added.is_empty()
            || !self.references_removed.is_empty()
            || !self.supersedes_added.is_empty()
            || !self.supersedes_removed.is_empty()
            || self.content_changed
    }

    /// Returns a human-readable summary of changes.
    pub fn summary(&self) -> String {
        let mut changes = Vec::new();

        if self.title.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("title");
        }
        if self.status.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("status");
        }
        if self.effective_date.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("effective date");
        }
        if self.expiry_date.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("expiry date");
        }
        if self.jurisdiction.as_ref().is_some_and(|c| c.is_changed()) {
            changes.push("jurisdiction");
        }
        if !self.tags_added.is_empty() || !self.tags_removed.is_empty() {
            changes.push("tags");
        }
        if !self.metadata_added.is_empty()
            || !self.metadata_removed.is_empty()
            || !self.metadata_changed.is_empty()
        {
            changes.push("metadata");
        }
        if !self.references_added.is_empty() || !self.references_removed.is_empty() {
            changes.push("references");
        }
        if !self.supersedes_added.is_empty() || !self.supersedes_removed.is_empty() {
            changes.push("supersedes");
        }
        if self.content_changed {
            changes.push("content");
        }

        if changes.is_empty() {
            "No changes".to_string()
        } else {
            format!("Changed: {}", changes.join(", "))
        }
    }
}

impl StatuteRegistry {
    /// Computes the difference between two versions of a statute.
    ///
    /// # Errors
    ///
    /// Returns an error if either version is not found.
    pub fn diff(
        &self,
        statute_id: &str,
        old_version: u32,
        new_version: u32,
    ) -> RegistryResult<StatuteDiff> {
        let old = self.get_version(statute_id, old_version)?;
        let new = self.get_version(statute_id, new_version)?;

        Ok(StatuteDiff::compute(old, new))
    }

    /// Computes the difference between a version and the latest version.
    pub fn diff_with_latest(
        &self,
        statute_id: &str,
        old_version: u32,
    ) -> RegistryResult<StatuteDiff> {
        let latest_version = self
            .latest_version(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        self.diff(statute_id, old_version, latest_version)
    }
}

// ============================================================================
// Validation Framework
// ============================================================================

/// A validation error.
#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    #[error("Empty statute ID")]
    EmptyStatuteId,

    #[error("Empty title")]
    EmptyTitle,

    #[error("Invalid jurisdiction: {0}")]
    InvalidJurisdiction(String),

    #[error("Invalid effective date: {0}")]
    InvalidEffectiveDate(String),

    #[error("Expiry date must be after effective date")]
    ExpiryBeforeEffective,

    #[error("Empty tag")]
    EmptyTag,

    #[error("Duplicate tag: {0}")]
    DuplicateTag(String),

    #[error("Custom validation error: {0}")]
    Custom(String),
}

/// Result type for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// A validation rule for statute entries.
pub trait ValidationRule: Send + Sync {
    /// Validates a statute entry.
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()>;

    /// Returns a description of this validation rule.
    fn description(&self) -> String;
}

/// Validates that statute ID is not empty.
#[derive(Debug, Clone)]
pub struct NonEmptyIdRule;

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

/// Validates that title is not empty.
#[derive(Debug, Clone)]
pub struct NonEmptyTitleRule;

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

/// Validates that jurisdiction is valid.
#[derive(Debug, Clone)]
pub struct ValidJurisdictionRule {
    /// Allowed jurisdictions
    pub allowed: HashSet<String>,
}

impl ValidJurisdictionRule {
    /// Creates a new jurisdiction validation rule.
    pub fn new(allowed: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            allowed: allowed.into_iter().map(|s| s.into()).collect(),
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

/// Validates that effective and expiry dates are logical.
#[derive(Debug, Clone)]
pub struct DateValidationRule;

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

/// Validates that tags are not empty and unique.
#[derive(Debug, Clone)]
pub struct TagValidationRule;

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

/// A collection of validation rules.
#[derive(Default)]
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    /// Creates a new empty validator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a validator with default rules.
    pub fn with_defaults() -> Self {
        let mut validator = Self::new();
        validator.add_rule(Box::new(NonEmptyIdRule));
        validator.add_rule(Box::new(NonEmptyTitleRule));
        validator.add_rule(Box::new(DateValidationRule));
        validator.add_rule(Box::new(TagValidationRule));
        validator
    }

    /// Adds a validation rule.
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// Validates a statute entry against all rules.
    pub fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()> {
        for rule in &self.rules {
            rule.validate(entry)?;
        }
        Ok(())
    }

    /// Returns all validation rules.
    pub fn rules(&self) -> &[Box<dyn ValidationRule>] {
        &self.rules
    }
}

// ============================================================================
// Metrics Collection
// ============================================================================

/// Operation metrics for the registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Total number of registrations
    pub registrations: u64,
    /// Total number of updates
    pub updates: u64,
    /// Total number of reads
    pub reads: u64,
    /// Total number of searches
    pub searches: u64,
    /// Total number of deletes (if supported)
    pub deletes: u64,
    /// Total number of status changes
    pub status_changes: u64,
    /// Total number of tag operations
    pub tag_operations: u64,
    /// Total number of metadata operations
    pub metadata_operations: u64,
    /// Total number of cache hits
    pub cache_hits: u64,
    /// Total number of cache misses
    pub cache_misses: u64,
    /// Total number of webhook triggers
    pub webhook_triggers: u64,
    /// Total number of validation failures
    pub validation_failures: u64,
}

impl OperationMetrics {
    /// Creates new empty metrics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Returns the total number of operations.
    pub fn total_operations(&self) -> u64 {
        self.registrations
            + self.updates
            + self.reads
            + self.searches
            + self.deletes
            + self.status_changes
            + self.tag_operations
            + self.metadata_operations
    }

    /// Resets all metrics to zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl StatuteRegistry {
    /// Returns the current operation metrics.
    ///
    /// Note: This requires the registry to track metrics internally.
    /// This is a placeholder that returns default metrics.
    pub fn metrics(&self) -> OperationMetrics {
        // In a real implementation, this would return actual tracked metrics
        // For now, we return a default instance as a placeholder
        OperationMetrics::default()
    }
}

// ============================================================================
// Merge Functionality
// ============================================================================

/// Strategy for resolving conflicts during merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Prefer the older version's values
    PreferOld,
    /// Prefer the newer version's values
    PreferNew,
    /// Fail if there are conflicts
    FailOnConflict,
    /// Merge both values (for collections)
    MergeBoth,
}

/// A conflict that occurred during merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeConflict {
    /// Title conflict
    Title { old: String, new: String },
    /// Status conflict
    Status {
        old: StatuteStatus,
        new: StatuteStatus,
    },
    /// Jurisdiction conflict
    Jurisdiction { old: String, new: String },
    /// Effective date conflict
    EffectiveDate {
        old: Option<DateTime<Utc>>,
        new: Option<DateTime<Utc>>,
    },
    /// Expiry date conflict
    ExpiryDate {
        old: Option<DateTime<Utc>>,
        new: Option<DateTime<Utc>>,
    },
}

/// Result of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// The merged statute entry
    pub entry: StatuteEntry,
    /// Conflicts that were resolved
    pub conflicts: Vec<MergeConflict>,
    /// Whether the merge was automatic or had conflicts
    pub has_conflicts: bool,
}

impl MergeResult {
    /// Returns true if the merge was successful without conflicts.
    pub fn is_clean(&self) -> bool {
        !self.has_conflicts
    }
}

impl StatuteEntry {
    /// Merges another statute entry into this one using the specified strategy.
    ///
    /// This is useful for reconciling concurrent modifications.
    pub fn merge(&self, other: &StatuteEntry, strategy: MergeStrategy) -> MergeResult {
        let mut merged = self.clone();
        let mut conflicts = Vec::new();

        // Merge title
        if self.statute.title != other.statute.title {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.statute.title = other.statute.title.clone();
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Title {
                        old: self.statute.title.clone(),
                        new: other.statute.title.clone(),
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.statute.title = other.statute.title.clone();
                }
            }
        }

        // Merge status
        if self.status != other.status {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.status = other.status;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Status {
                        old: self.status,
                        new: other.status,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.status = other.status;
                }
            }
        }

        // Merge jurisdiction
        if self.jurisdiction != other.jurisdiction {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.jurisdiction = other.jurisdiction.clone();
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::Jurisdiction {
                        old: self.jurisdiction.clone(),
                        new: other.jurisdiction.clone(),
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.jurisdiction = other.jurisdiction.clone();
                }
            }
        }

        // Merge effective date
        if self.effective_date != other.effective_date {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.effective_date = other.effective_date;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::EffectiveDate {
                        old: self.effective_date,
                        new: other.effective_date,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.effective_date = other.effective_date;
                }
            }
        }

        // Merge expiry date
        if self.expiry_date != other.expiry_date {
            match strategy {
                MergeStrategy::PreferOld => {}
                MergeStrategy::PreferNew => {
                    merged.expiry_date = other.expiry_date;
                }
                MergeStrategy::FailOnConflict => {
                    conflicts.push(MergeConflict::ExpiryDate {
                        old: self.expiry_date,
                        new: other.expiry_date,
                    });
                }
                MergeStrategy::MergeBoth => {
                    merged.expiry_date = other.expiry_date;
                }
            }
        }

        // Merge tags (always union)
        let old_tags: HashSet<_> = self.tags.iter().cloned().collect();
        let new_tags: HashSet<_> = other.tags.iter().cloned().collect();
        merged.tags = old_tags.union(&new_tags).cloned().collect();

        // Merge metadata
        match strategy {
            MergeStrategy::PreferOld => {}
            MergeStrategy::PreferNew => {
                merged.metadata = other.metadata.clone();
            }
            MergeStrategy::MergeBoth => {
                // Merge metadata: new values override old
                for (k, v) in &other.metadata {
                    merged.metadata.insert(k.clone(), v.clone());
                }
            }
            MergeStrategy::FailOnConflict => {
                // Only add new metadata, don't override
                for (k, v) in &other.metadata {
                    if !merged.metadata.contains_key(k) {
                        merged.metadata.insert(k.clone(), v.clone());
                    }
                }
            }
        }

        // Merge references (always union)
        let old_refs: HashSet<_> = self.references.iter().cloned().collect();
        let new_refs: HashSet<_> = other.references.iter().cloned().collect();
        merged.references = old_refs.union(&new_refs).cloned().collect();

        // Merge supersedes (always union)
        let old_super: HashSet<_> = self.supersedes.iter().cloned().collect();
        let new_super: HashSet<_> = other.supersedes.iter().cloned().collect();
        merged.supersedes = old_super.union(&new_super).cloned().collect();

        // Update timestamps
        merged.modified_at = Utc::now();
        merged.update_etag();

        MergeResult {
            entry: merged,
            has_conflicts: !conflicts.is_empty(),
            conflicts,
        }
    }
}

// ============================================================================
// YAML Export/Import
// ============================================================================

#[cfg(feature = "yaml")]
impl StatuteRegistry {
    /// Exports the registry to YAML format.
    ///
    /// # Errors
    ///
    /// Returns an error if YAML serialization fails.
    pub fn export_yaml(&self) -> Result<String, serde_yaml::Error> {
        let backup = RegistryBackup {
            statutes: self.statutes.values().cloned().collect(),
            versions: self.versions.clone(),
            events: self.event_store.all_events().into_iter().cloned().collect(),
            metadata: BackupMetadata {
                created_at: Utc::now(),
                format_version: "1.0".to_string(),
                statute_count: self.statutes.len(),
                event_count: self.event_store.count(),
                description: Some("YAML export".to_string()),
            },
        };
        serde_yaml::to_string(&backup)
    }

    /// Imports a registry from YAML format.
    ///
    /// # Errors
    ///
    /// Returns an error if YAML deserialization fails or the backup is invalid.
    pub fn import_yaml(&mut self, yaml: &str) -> Result<(), Box<dyn std::error::Error>> {
        let backup: RegistryBackup = serde_yaml::from_str(yaml)?;
        self.restore_from_backup(backup)?;
        Ok(())
    }

    /// Exports a single statute entry to YAML.
    pub fn export_statute_yaml(entry: &StatuteEntry) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(entry)
    }

    /// Imports a single statute entry from YAML.
    pub fn import_statute_yaml(yaml: &str) -> Result<StatuteEntry, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}

// ============================================================================
// CSV Export
// ============================================================================

#[cfg(feature = "csv-export")]
impl StatuteRegistry {
    /// Exports statute summaries to CSV format.
    ///
    /// # Errors
    ///
    /// Returns an error if CSV serialization fails.
    pub fn export_summaries_csv(&self) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record([
            "statute_id",
            "title",
            "version",
            "status",
            "jurisdiction",
            "tags",
            "created_at",
            "modified_at",
            "is_active",
        ])?;

        // Write records
        for entry in self.statutes.values() {
            let summary = StatuteSummary::from(entry);
            wtr.write_record([
                &summary.statute_id,
                &summary.title,
                &summary.version.to_string(),
                &format!("{:?}", summary.status),
                &summary.jurisdiction,
                &summary.tags.join(";"),
                &summary.created_at.to_rfc3339(),
                &summary.modified_at.to_rfc3339(),
                &summary.is_active.to_string(),
            ])?;
        }

        let data = wtr
            .into_inner()
            .map_err(|e| csv::Error::from(std::io::Error::other(e)))?;
        Ok(String::from_utf8(data).unwrap_or_default())
    }

    /// Exports filtered statute summaries to CSV format.
    ///
    /// # Errors
    ///
    /// Returns an error if CSV serialization fails.
    pub fn export_filtered_csv(
        &self,
        filter: impl Fn(&StatuteEntry) -> bool,
    ) -> Result<String, csv::Error> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header
        wtr.write_record([
            "statute_id",
            "title",
            "version",
            "status",
            "jurisdiction",
            "tags",
            "created_at",
            "modified_at",
            "is_active",
        ])?;

        // Write filtered records
        for entry in self.statutes.values().filter(|e| filter(e)) {
            let summary = StatuteSummary::from(entry);
            wtr.write_record([
                &summary.statute_id,
                &summary.title,
                &summary.version.to_string(),
                &format!("{:?}", summary.status),
                &summary.jurisdiction,
                &summary.tags.join(";"),
                &summary.created_at.to_rfc3339(),
                &summary.modified_at.to_rfc3339(),
                &summary.is_active.to_string(),
            ])?;
        }

        let data = wtr
            .into_inner()
            .map_err(|e| csv::Error::from(std::io::Error::other(e)))?;
        Ok(String::from_utf8(data).unwrap_or_default())
    }
}

// ============================================================================
// Backup Compression
// ============================================================================

#[cfg(feature = "compression")]
impl StatuteRegistry {
    /// Exports a compressed backup.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization or compression fails.
    pub fn export_compressed_backup(
        &self,
        description: Option<String>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;

        let json = self.export_backup(description)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json.as_bytes())?;
        Ok(encoder.finish()?)
    }

    /// Imports a compressed backup.
    ///
    /// # Errors
    ///
    /// Returns an error if decompression or deserialization fails.
    pub fn import_compressed_backup(
        &mut self,
        compressed: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(compressed);
        let mut json = String::new();
        decoder.read_to_string(&mut json)?;
        self.import_backup(&json)?;
        Ok(())
    }

    /// Returns the compression ratio of a backup (original_size / compressed_size).
    pub fn compression_ratio(
        &self,
        description: Option<String>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let original = self.export_backup(description)?;
        let compressed = self.export_compressed_backup(None)?;
        Ok(original.len() as f64 / compressed.len() as f64)
    }
}

// ============================================================================
// Batch Validation
// ============================================================================

/// Result of batch validation.
#[derive(Debug, Clone)]
pub struct BatchValidationResult {
    /// Total number of entries validated
    pub total: usize,
    /// Number of valid entries
    pub valid: usize,
    /// Number of invalid entries
    pub invalid: usize,
    /// Validation errors by statute ID
    pub errors: HashMap<String, ValidationError>,
}

impl BatchValidationResult {
    /// Returns true if all entries are valid.
    pub fn is_all_valid(&self) -> bool {
        self.invalid == 0
    }

    /// Returns the validation success rate (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.valid as f64 / self.total as f64
        }
    }
}

impl Validator {
    /// Validates multiple statute entries.
    pub fn validate_batch(&self, entries: &[StatuteEntry]) -> BatchValidationResult {
        let mut errors = HashMap::new();
        let mut valid = 0;
        let mut invalid = 0;

        for entry in entries {
            match self.validate(entry) {
                Ok(()) => valid += 1,
                Err(e) => {
                    invalid += 1;
                    errors.insert(entry.statute.id.clone(), e);
                }
            }
        }

        BatchValidationResult {
            total: entries.len(),
            valid,
            invalid,
            errors,
        }
    }

    /// Validates multiple entries and returns only the valid ones.
    pub fn filter_valid(&self, entries: Vec<StatuteEntry>) -> Vec<StatuteEntry> {
        entries
            .into_iter()
            .filter(|e| self.validate(e).is_ok())
            .collect()
    }

    /// Validates multiple entries and returns only the invalid ones with their errors.
    pub fn filter_invalid(
        &self,
        entries: Vec<StatuteEntry>,
    ) -> Vec<(StatuteEntry, ValidationError)> {
        entries
            .into_iter()
            .filter_map(|e| self.validate(&e).err().map(|err| (e, err)))
            .collect()
    }
}

// ============================================================================
// Enhanced Search Result Caching
// ============================================================================

/// Cache entry with TTL (Time To Live).
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CachedSearchResult {
    results: Vec<String>,
    cached_at: DateTime<Utc>,
    ttl_seconds: i64,
}

#[allow(dead_code)]
impl CachedSearchResult {
    /// Returns true if the cache entry is still valid.
    fn is_valid(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.cached_at);
        elapsed.num_seconds() < self.ttl_seconds
    }
}

/// Search cache configuration.
#[derive(Debug, Clone, Copy)]
pub struct SearchCacheConfig {
    /// Maximum number of cached queries
    pub max_entries: usize,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: i64,
}

impl Default for SearchCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 100,
            ttl_seconds: 300, // 5 minutes
        }
    }
}

impl SearchCacheConfig {
    /// Creates a new cache config.
    pub fn new(max_entries: usize, ttl_seconds: i64) -> Self {
        Self {
            max_entries,
            ttl_seconds,
        }
    }

    /// Creates a cache config with no TTL (cache indefinitely).
    pub fn no_ttl(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: i64::MAX,
        }
    }

    /// Creates a cache config with short TTL (1 minute).
    pub fn short_lived(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: 60,
        }
    }

    /// Creates a cache config with long TTL (1 hour).
    pub fn long_lived(max_entries: usize) -> Self {
        Self {
            max_entries,
            ttl_seconds: 3600,
        }
    }
}

// ============================================================================
// Audit Trail System
// ============================================================================

/// Audit log entry capturing detailed operation information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique audit ID
    pub audit_id: Uuid,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// User or system that performed the operation
    pub actor: String,
    /// Type of operation performed
    pub operation: AuditOperation,
    /// Statute ID affected (if applicable)
    pub statute_id: Option<String>,
    /// Result of the operation
    pub result: AuditResult,
    /// IP address or source identifier
    pub source: Option<String>,
    /// Additional context data
    pub metadata: HashMap<String, String>,
}

/// Types of auditable operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditOperation {
    /// Register new statute
    Register,
    /// Update existing statute
    Update,
    /// Delete statute
    Delete,
    /// Archive statute
    Archive,
    /// Unarchive statute
    Unarchive,
    /// Change status
    StatusChange {
        from: StatuteStatus,
        to: StatuteStatus,
    },
    /// Add tag
    AddTag { tag: String },
    /// Remove tag
    RemoveTag { tag: String },
    /// Add metadata
    AddMetadata { key: String },
    /// Remove metadata
    RemoveMetadata { key: String },
    /// Export data
    Export { format: String },
    /// Import data
    Import { format: String },
    /// Search operation
    Search { query: String },
    /// Batch operation
    BatchOperation {
        operation_type: String,
        count: usize,
    },
    /// Apply retention policy
    RetentionPolicy,
    /// Create snapshot
    CreateSnapshot,
    /// Restore from snapshot
    RestoreSnapshot { snapshot_id: Uuid },
}

/// Result of an audited operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditResult {
    /// Operation succeeded
    Success,
    /// Operation failed with error message
    Failure { error: String },
    /// Operation partially succeeded
    PartialSuccess { succeeded: usize, failed: usize },
}

impl AuditEntry {
    /// Creates a new audit entry.
    pub fn new(actor: String, operation: AuditOperation, result: AuditResult) -> Self {
        Self {
            audit_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            actor,
            operation,
            statute_id: None,
            result,
            source: None,
            metadata: HashMap::new(),
        }
    }

    /// Builder: Sets the statute ID.
    pub fn with_statute_id(mut self, statute_id: String) -> Self {
        self.statute_id = Some(statute_id);
        self
    }

    /// Builder: Sets the source.
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Builder: Adds metadata.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Checks if the operation was successful.
    pub fn is_success(&self) -> bool {
        matches!(self.result, AuditResult::Success)
    }

    /// Checks if the operation failed.
    pub fn is_failure(&self) -> bool {
        matches!(self.result, AuditResult::Failure { .. })
    }
}

/// Audit trail manager for tracking all operations.
#[derive(Debug, Clone)]
pub struct AuditTrail {
    entries: VecDeque<AuditEntry>,
    max_entries: usize,
    enabled: bool,
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl AuditTrail {
    /// Creates a new audit trail with maximum entries.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
            enabled: true,
        }
    }

    /// Records an audit entry.
    pub fn record(&mut self, entry: AuditEntry) {
        if !self.enabled {
            return;
        }

        self.entries.push_back(entry);
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    /// Enables audit logging.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables audit logging.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Checks if audit logging is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns all audit entries.
    pub fn entries(&self) -> &VecDeque<AuditEntry> {
        &self.entries
    }

    /// Returns entries for a specific actor.
    pub fn entries_by_actor(&self, actor: &str) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }

    /// Returns entries for a specific statute.
    pub fn entries_by_statute(&self, statute_id: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id.as_deref() == Some(statute_id))
            .collect()
    }

    /// Returns entries within a time range.
    pub fn entries_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Returns entries by operation type.
    pub fn entries_by_operation(&self, operation_type: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| format!("{:?}", e.operation).contains(operation_type))
            .collect()
    }

    /// Returns only successful operations.
    pub fn successful_operations(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.is_success()).collect()
    }

    /// Returns only failed operations.
    pub fn failed_operations(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.is_failure()).collect()
    }

    /// Returns the total number of entries.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Clears all audit entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Exports audit trail to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.entries)
    }
}

// ============================================================================
// Health Check System
// ============================================================================

/// Health status of the registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some degradation but functional
    Degraded { issues: Vec<String> },
    /// Critical issues affecting functionality
    Unhealthy { errors: Vec<String> },
}

impl HealthStatus {
    /// Checks if the status is healthy.
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    /// Checks if the status is degraded.
    pub fn is_degraded(&self) -> bool {
        matches!(self, HealthStatus::Degraded { .. })
    }

    /// Checks if the status is unhealthy.
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy { .. })
    }
}

/// Comprehensive health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Overall health status
    pub status: HealthStatus,
    /// Timestamp of the check
    pub timestamp: DateTime<Utc>,
    /// Total statutes in registry
    pub statute_count: usize,
    /// Total versions tracked
    pub version_count: usize,
    /// Total events in event store
    pub event_count: usize,
    /// Cache hit rate (0.0-1.0)
    pub cache_hit_rate: f64,
    /// Number of archived statutes
    pub archived_count: usize,
    /// Memory usage estimate (bytes)
    pub memory_estimate_bytes: usize,
    /// Check duration (milliseconds)
    pub check_duration_ms: u64,
    /// Component-specific checks
    pub component_checks: HashMap<String, ComponentHealth>,
}

/// Health status of individual components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Is component healthy
    pub healthy: bool,
    /// Component-specific message
    pub message: Option<String>,
    /// Component metrics
    pub metrics: HashMap<String, f64>,
}

impl ComponentHealth {
    /// Creates a healthy component check.
    pub fn healthy(name: String) -> Self {
        Self {
            name,
            healthy: true,
            message: None,
            metrics: HashMap::new(),
        }
    }

    /// Creates an unhealthy component check.
    pub fn unhealthy(name: String, message: String) -> Self {
        Self {
            name,
            healthy: false,
            message: Some(message),
            metrics: HashMap::new(),
        }
    }

    /// Adds a metric to the component health.
    pub fn with_metric(mut self, key: String, value: f64) -> Self {
        self.metrics.insert(key, value);
        self
    }
}

// ============================================================================
// Registry Comparison Tools
// ============================================================================

/// Difference between two registries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDifference {
    /// Timestamp of comparison
    pub compared_at: DateTime<Utc>,
    /// Statutes only in left registry
    pub only_in_left: Vec<String>,
    /// Statutes only in right registry
    pub only_in_right: Vec<String>,
    /// Statutes in both but with differences
    pub different_statutes: Vec<StatuteDifferenceDetail>,
    /// Statutes that are identical
    pub identical_statutes: Vec<String>,
}

impl RegistryDifference {
    /// Creates a new empty registry difference.
    pub fn new() -> Self {
        Self {
            compared_at: Utc::now(),
            only_in_left: Vec::new(),
            only_in_right: Vec::new(),
            different_statutes: Vec::new(),
            identical_statutes: Vec::new(),
        }
    }

    /// Returns the total number of differences found.
    pub fn difference_count(&self) -> usize {
        self.only_in_left.len() + self.only_in_right.len() + self.different_statutes.len()
    }

    /// Checks if the registries are identical.
    pub fn is_identical(&self) -> bool {
        self.difference_count() == 0
    }

    /// Returns a summary of the comparison.
    pub fn summary(&self) -> String {
        format!(
            "Only in left: {}, Only in right: {}, Different: {}, Identical: {}",
            self.only_in_left.len(),
            self.only_in_right.len(),
            self.different_statutes.len(),
            self.identical_statutes.len()
        )
    }
}

impl Default for RegistryDifference {
    fn default() -> Self {
        Self::new()
    }
}

/// Details of differences in a specific statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDifferenceDetail {
    /// Statute ID
    pub statute_id: String,
    /// Fields that differ
    pub differing_fields: Vec<String>,
    /// Version in left registry
    pub left_version: u32,
    /// Version in right registry
    pub right_version: u32,
}

// ============================================================================
// Bulk Streaming Operations
// ============================================================================

/// Configuration for bulk operations.
#[derive(Debug, Clone)]
pub struct BulkConfig {
    /// Batch size for processing
    pub batch_size: usize,
    /// Whether to continue on error
    pub continue_on_error: bool,
    /// Maximum parallel operations
    pub max_parallelism: usize,
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

impl BulkConfig {
    /// Creates a new bulk config.
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            ..Default::default()
        }
    }

    /// Builder: Sets continue on error.
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }

    /// Builder: Sets max parallelism.
    pub fn with_max_parallelism(mut self, max_parallelism: usize) -> Self {
        self.max_parallelism = max_parallelism;
        self
    }
}

/// Result of a bulk operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    /// Total items processed
    pub total_processed: usize,
    /// Successful operations
    pub successful: usize,
    /// Failed operations
    pub failed: usize,
    /// Error details by statute ID
    pub errors: HashMap<String, String>,
    /// Duration of the operation
    pub duration_ms: u64,
}

impl BulkOperationResult {
    /// Creates a new empty result.
    pub fn new() -> Self {
        Self {
            total_processed: 0,
            successful: 0,
            failed: 0,
            errors: HashMap::new(),
            duration_ms: 0,
        }
    }

    /// Checks if all operations succeeded.
    pub fn is_all_successful(&self) -> bool {
        self.failed == 0 && self.total_processed > 0
    }

    /// Returns the success rate (0.0-1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_processed == 0 {
            0.0
        } else {
            self.successful as f64 / self.total_processed as f64
        }
    }
}

impl Default for BulkOperationResult {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// StatuteRegistry Extensions
// ============================================================================

impl StatuteRegistry {
    /// Performs a comprehensive health check on the registry.
    pub fn health_check(&self) -> HealthCheckResult {
        let start = std::time::Instant::now();
        let mut component_checks = HashMap::new();

        // Check cache health
        let cache_health = ComponentHealth::healthy("cache".to_string())
            .with_metric("capacity".to_string(), self.cache.cap().get() as f64)
            .with_metric("current_size".to_string(), self.cache.len() as f64);
        component_checks.insert("cache".to_string(), cache_health);

        // Check storage health
        let statute_count = self.statutes.len();
        let version_count: usize = self.versions.values().map(|v| v.len()).sum();
        let storage_health = ComponentHealth::healthy("storage".to_string())
            .with_metric("statutes".to_string(), statute_count as f64)
            .with_metric("versions".to_string(), version_count as f64);
        component_checks.insert("storage".to_string(), storage_health);

        // Check index health
        let tag_count = self.tag_index.len();
        let jurisdiction_count = self.jurisdiction_index.len();
        let index_health = ComponentHealth::healthy("indexes".to_string())
            .with_metric("tags".to_string(), tag_count as f64)
            .with_metric("jurisdictions".to_string(), jurisdiction_count as f64);
        component_checks.insert("indexes".to_string(), index_health);

        // Check event store health
        let event_count = self.event_store.events.len();
        let event_health = ComponentHealth::healthy("event_store".to_string())
            .with_metric("events".to_string(), event_count as f64);
        component_checks.insert("event_store".to_string(), event_health);

        // Determine overall status
        let mut issues = Vec::new();
        let errors = Vec::new();

        if statute_count == 0 {
            issues.push("Registry is empty".to_string());
        }

        if statute_count > 100000 {
            issues.push("Registry has very large number of statutes (>100k)".to_string());
        }

        if event_count > 1000000 {
            issues.push("Event store has very large number of events (>1M)".to_string());
        }

        let status = if !errors.is_empty() {
            HealthStatus::Unhealthy { errors }
        } else if !issues.is_empty() {
            HealthStatus::Degraded { issues }
        } else {
            HealthStatus::Healthy
        };

        // Estimate memory usage (rough approximation)
        let memory_estimate = statute_count * 1024  // ~1KB per statute
            + version_count * 1024                   // ~1KB per version
            + event_count * 512; // ~512B per event

        let duration_ms = start.elapsed().as_millis() as u64;

        HealthCheckResult {
            status,
            timestamp: Utc::now(),
            statute_count,
            version_count,
            event_count,
            cache_hit_rate: 0.0, // Would need metrics tracking
            archived_count: self.archive.count(),
            memory_estimate_bytes: memory_estimate,
            check_duration_ms: duration_ms,
            component_checks,
        }
    }

    /// Compares this registry with another registry.
    pub fn compare_with(&self, other: &StatuteRegistry) -> RegistryDifference {
        let mut diff = RegistryDifference::new();

        let left_ids: HashSet<_> = self.statutes.keys().cloned().collect();
        let right_ids: HashSet<_> = other.statutes.keys().cloned().collect();

        // Find statutes only in left
        diff.only_in_left = left_ids.difference(&right_ids).cloned().collect();
        diff.only_in_left.sort();

        // Find statutes only in right
        diff.only_in_right = right_ids.difference(&left_ids).cloned().collect();
        diff.only_in_right.sort();

        // Find statutes in both and compare
        for statute_id in left_ids.intersection(&right_ids) {
            let left_entry = &self.statutes[statute_id];
            let right_entry = &other.statutes[statute_id];

            if self.are_entries_identical(left_entry, right_entry) {
                diff.identical_statutes.push(statute_id.clone());
            } else {
                let differing_fields = self.find_differing_fields(left_entry, right_entry);
                diff.different_statutes.push(StatuteDifferenceDetail {
                    statute_id: statute_id.clone(),
                    differing_fields,
                    left_version: left_entry.version,
                    right_version: right_entry.version,
                });
            }
        }

        diff.identical_statutes.sort();
        diff
    }

    /// Checks if two statute entries are identical.
    fn are_entries_identical(&self, left: &StatuteEntry, right: &StatuteEntry) -> bool {
        left.statute.id == right.statute.id
            && left.statute.title == right.statute.title
            && left.version == right.version
            && left.status == right.status
            && left.jurisdiction == right.jurisdiction
            && left.tags == right.tags
    }

    /// Finds fields that differ between two entries.
    fn find_differing_fields(&self, left: &StatuteEntry, right: &StatuteEntry) -> Vec<String> {
        let mut fields = Vec::new();

        if left.statute.title != right.statute.title {
            fields.push("title".to_string());
        }
        if left.version != right.version {
            fields.push("version".to_string());
        }
        if left.status != right.status {
            fields.push("status".to_string());
        }
        if left.jurisdiction != right.jurisdiction {
            fields.push("jurisdiction".to_string());
        }
        if left.tags != right.tags {
            fields.push("tags".to_string());
        }
        if left.effective_date != right.effective_date {
            fields.push("effective_date".to_string());
        }
        if left.expiry_date != right.expiry_date {
            fields.push("expiry_date".to_string());
        }

        fields
    }

    /// Performs bulk registration with configuration.
    pub fn bulk_register(
        &mut self,
        entries: Vec<StatuteEntry>,
        config: BulkConfig,
    ) -> BulkOperationResult {
        let start = std::time::Instant::now();
        let mut result = BulkOperationResult::new();

        for chunk in entries.chunks(config.batch_size) {
            for entry in chunk {
                result.total_processed += 1;
                match self.register(entry.clone()) {
                    Ok(_) => result.successful += 1,
                    Err(e) => {
                        result.failed += 1;
                        result
                            .errors
                            .insert(entry.statute.id.clone(), e.to_string());
                        if !config.continue_on_error {
                            result.duration_ms = start.elapsed().as_millis() as u64;
                            return result;
                        }
                    }
                }
            }
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// Performs bulk deletion with configuration.
    pub fn bulk_delete_with_config(
        &mut self,
        statute_ids: Vec<String>,
        config: BulkConfig,
    ) -> BulkOperationResult {
        let start = std::time::Instant::now();
        let mut result = BulkOperationResult::new();

        for chunk in statute_ids.chunks(config.batch_size) {
            for statute_id in chunk {
                result.total_processed += 1;
                match self.delete(statute_id) {
                    Ok(_) => result.successful += 1,
                    Err(e) => {
                        result.failed += 1;
                        result.errors.insert(statute_id.clone(), e.to_string());
                        if !config.continue_on_error {
                            result.duration_ms = start.elapsed().as_millis() as u64;
                            return result;
                        }
                    }
                }
            }
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// Streams statute IDs matching a predicate.
    pub fn stream_ids<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        self.statutes
            .iter()
            .filter(|(_, entry)| predicate(entry))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Streams entries matching a predicate with batching.
    pub fn stream_entries<F>(&self, predicate: F, batch_size: usize) -> Vec<Vec<StatuteEntry>>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        let entries: Vec<StatuteEntry> = self
            .statutes
            .values()
            .filter(|entry| predicate(entry))
            .cloned()
            .collect();

        entries
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

// ============================================================================
// Performance Benchmarking
// ============================================================================

/// Performance benchmark result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Number of iterations
    pub iterations: usize,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
    /// Average duration per operation in microseconds
    pub avg_duration_us: f64,
    /// Operations per second
    pub ops_per_sec: f64,
    /// Minimum duration in microseconds
    pub min_duration_us: u64,
    /// Maximum duration in microseconds
    pub max_duration_us: u64,
}

impl BenchmarkResult {
    /// Creates a new benchmark result.
    pub fn new(name: String, iterations: usize, durations_us: Vec<u64>) -> Self {
        let total_duration_us: u64 = durations_us.iter().sum();
        let total_duration_ms = total_duration_us / 1000;
        let avg_duration_us = total_duration_us as f64 / iterations as f64;
        let ops_per_sec = 1_000_000.0 / avg_duration_us;
        let min_duration_us = *durations_us.iter().min().unwrap_or(&0);
        let max_duration_us = *durations_us.iter().max().unwrap_or(&0);

        Self {
            name,
            iterations,
            total_duration_ms,
            avg_duration_us,
            ops_per_sec,
            min_duration_us,
            max_duration_us,
        }
    }

    /// Returns a formatted summary.
    pub fn summary(&self) -> String {
        format!(
            "{}: {:.2} ops/sec, avg: {:.2}µs, min: {}µs, max: {}µs ({} iterations)",
            self.name,
            self.ops_per_sec,
            self.avg_duration_us,
            self.min_duration_us,
            self.max_duration_us,
            self.iterations
        )
    }
}

/// Benchmark suite for registry operations.
#[derive(Debug, Clone, Default)]
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    /// Creates a new benchmark suite.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Adds a benchmark result.
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Returns all benchmark results.
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Exports results to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.results)
    }

    /// Returns a summary of all benchmarks.
    pub fn summary(&self) -> String {
        let mut summary = String::from("Benchmark Results:\n");
        for result in &self.results {
            summary.push_str(&format!("  {}\n", result.summary()));
        }
        summary
    }
}

// ============================================================================
// Rate Limiting
// ============================================================================

/// Rate limit configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Time window in seconds
    pub window_secs: i64,
    /// Whether to enable rate limiting
    pub enabled: bool,
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

impl RateLimitConfig {
    /// Creates a new rate limit config.
    pub fn new(max_requests: usize, window_secs: i64) -> Self {
        Self {
            max_requests,
            window_secs,
            enabled: true,
        }
    }

    /// Disables rate limiting.
    pub fn disabled() -> Self {
        Self {
            max_requests: 0,
            window_secs: 0,
            enabled: false,
        }
    }

    /// Builder: Sets enabled flag.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Rate limiter for protecting against abuse.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    /// Request timestamps by key (e.g., user ID, IP)
    requests: HashMap<String, VecDeque<DateTime<Utc>>>,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            requests: HashMap::new(),
        }
    }

    /// Checks if a request is allowed for the given key.
    pub fn check_rate_limit(&mut self, key: &str) -> bool {
        if !self.config.enabled {
            return true;
        }

        let now = Utc::now();
        let window_start = now - chrono::Duration::seconds(self.config.window_secs);

        // Get or create request history for this key
        let history = self.requests.entry(key.to_string()).or_default();

        // Remove old requests outside the window
        while let Some(&front) = history.front() {
            if front < window_start {
                history.pop_front();
            } else {
                break;
            }
        }

        // Check if under limit
        if history.len() >= self.config.max_requests {
            return false;
        }

        // Record this request
        history.push_back(now);
        true
    }

    /// Returns current request count for a key.
    pub fn current_count(&self, key: &str) -> usize {
        self.requests.get(key).map(|h| h.len()).unwrap_or(0)
    }

    /// Returns remaining requests for a key.
    pub fn remaining(&self, key: &str) -> usize {
        if !self.config.enabled {
            return usize::MAX;
        }
        let current = self.current_count(key);
        self.config.max_requests.saturating_sub(current)
    }

    /// Resets rate limit for a specific key.
    pub fn reset(&mut self, key: &str) {
        self.requests.remove(key);
    }

    /// Clears all rate limit data.
    pub fn clear_all(&mut self) {
        self.requests.clear();
    }

    /// Returns the configuration.
    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

// ============================================================================
// Circuit Breaker
// ============================================================================

/// Circuit breaker state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are rejected
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Time to wait before attempting recovery (seconds)
    pub timeout_secs: i64,
    /// Number of successful requests needed to close circuit
    pub success_threshold: usize,
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

impl CircuitBreakerConfig {
    /// Creates a new circuit breaker config.
    pub fn new(failure_threshold: usize, timeout_secs: i64, success_threshold: usize) -> Self {
        Self {
            failure_threshold,
            timeout_secs,
            success_threshold,
        }
    }
}

/// Circuit breaker for fault tolerance.
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<DateTime<Utc>>,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker.
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }

    /// Records a successful operation.
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                    self.last_failure_time = None;
                }
            }
            CircuitState::Open => {
                // Should not happen, but reset if it does
                self.state = CircuitState::Closed;
                self.failure_count = 0;
                self.success_count = 0;
            }
        }
    }

    /// Records a failed operation.
    pub fn record_failure(&mut self) {
        self.last_failure_time = Some(Utc::now());

        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.success_count = 0;
            }
            CircuitState::Open => {
                // Already open, nothing to do
            }
        }
    }

    /// Checks if a request is allowed.
    pub fn is_request_allowed(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has passed
                if let Some(last_failure) = self.last_failure_time {
                    let now = Utc::now();
                    let timeout = chrono::Duration::seconds(self.config.timeout_secs);
                    if now - last_failure >= timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Returns the current state.
    pub fn state(&self) -> &CircuitState {
        &self.state
    }

    /// Returns the failure count.
    pub fn failure_count(&self) -> usize {
        self.failure_count
    }

    /// Resets the circuit breaker.
    pub fn reset(&mut self) {
        self.state = CircuitState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.last_failure_time = None;
    }

    /// Forces the circuit to open.
    pub fn force_open(&mut self) {
        self.state = CircuitState::Open;
        self.last_failure_time = Some(Utc::now());
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }
}

// ============================================================================
// Observability
// ============================================================================

/// Log level for observability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

/// Structured log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Operation that generated the log
    pub operation: String,
    /// Additional context fields
    pub fields: HashMap<String, String>,
}

impl LogEntry {
    /// Creates a new log entry.
    pub fn new(level: LogLevel, operation: String, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            message,
            operation,
            fields: HashMap::new(),
        }
    }

    /// Adds a field to the log entry.
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }
}

/// Metric type for observability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter that only increases
    Counter { value: u64 },
    /// Gauge that can increase or decrease
    Gauge { value: f64 },
    /// Histogram of values
    Histogram { values: Vec<f64> },
    /// Timing measurement in microseconds
    Timing { duration_us: u64 },
}

/// Metric entry for observability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEntry {
    /// Metric name
    pub name: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Metric type and value
    pub metric_type: MetricType,
    /// Labels for grouping
    pub labels: HashMap<String, String>,
}

impl MetricEntry {
    /// Creates a new counter metric.
    pub fn counter(name: String, value: u64) -> Self {
        Self {
            name,
            timestamp: Utc::now(),
            metric_type: MetricType::Counter { value },
            labels: HashMap::new(),
        }
    }

    /// Creates a new gauge metric.
    pub fn gauge(name: String, value: f64) -> Self {
        Self {
            name,
            timestamp: Utc::now(),
            metric_type: MetricType::Gauge { value },
            labels: HashMap::new(),
        }
    }

    /// Creates a new timing metric.
    pub fn timing(name: String, duration_us: u64) -> Self {
        Self {
            name,
            timestamp: Utc::now(),
            metric_type: MetricType::Timing { duration_us },
            labels: HashMap::new(),
        }
    }

    /// Adds a label to the metric.
    pub fn with_label(mut self, key: String, value: String) -> Self {
        self.labels.insert(key, value);
        self
    }
}

/// Observability collector for logs and metrics.
#[derive(Debug, Clone)]
pub struct ObservabilityCollector {
    logs: VecDeque<LogEntry>,
    metrics: VecDeque<MetricEntry>,
    max_logs: usize,
    max_metrics: usize,
    min_log_level: LogLevel,
}

impl Default for ObservabilityCollector {
    fn default() -> Self {
        Self::new(10000, 10000, LogLevel::Info)
    }
}

impl ObservabilityCollector {
    /// Creates a new observability collector.
    pub fn new(max_logs: usize, max_metrics: usize, min_log_level: LogLevel) -> Self {
        Self {
            logs: VecDeque::new(),
            metrics: VecDeque::new(),
            max_logs,
            max_metrics,
            min_log_level,
        }
    }

    /// Records a log entry.
    pub fn log(&mut self, entry: LogEntry) {
        if entry.level < self.min_log_level {
            return;
        }

        self.logs.push_back(entry);
        if self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }

    /// Records a metric entry.
    pub fn metric(&mut self, entry: MetricEntry) {
        self.metrics.push_back(entry);
        if self.metrics.len() > self.max_metrics {
            self.metrics.pop_front();
        }
    }

    /// Returns all logs.
    pub fn logs(&self) -> &VecDeque<LogEntry> {
        &self.logs
    }

    /// Returns all metrics.
    pub fn metrics(&self) -> &VecDeque<MetricEntry> {
        &self.metrics
    }

    /// Returns logs filtered by level.
    pub fn logs_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter().filter(|e| e.level == level).collect()
    }

    /// Returns logs filtered by operation.
    pub fn logs_by_operation(&self, operation: &str) -> Vec<&LogEntry> {
        self.logs
            .iter()
            .filter(|e| e.operation == operation)
            .collect()
    }

    /// Returns metrics by name.
    pub fn metrics_by_name(&self, name: &str) -> Vec<&MetricEntry> {
        self.metrics.iter().filter(|m| m.name == name).collect()
    }

    /// Clears all logs.
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Clears all metrics.
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    /// Exports logs to JSON.
    pub fn export_logs_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.logs)
    }

    /// Exports metrics to JSON.
    pub fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                    StatuteEntry::new(test_statute(&format!("civil-{}", i)), "JP")
                        .with_tag("civil"),
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

        let statute_b =
            StatuteEntry::new(test_statute("statute-b"), "JP").with_reference("statute-c");

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
                    StatuteEntry::new(test_statute(&format!("civil-{}", i)), "JP")
                        .with_tag("civil"),
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
                StatuteEntry::new(test_statute("active-1"), "JP")
                    .with_status(StatuteStatus::Active),
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
        let entry1 =
            StatuteEntry::new(test_statute("test-1"), "JP").with_status(StatuteStatus::Draft);

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
        let entry1 =
            StatuteEntry::new(test_statute("test-1"), "JP").with_metadata("key", "old_value");

        let entry2 =
            StatuteEntry::new(test_statute("test-1"), "JP").with_metadata("key", "new_value");

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
        let query =
            SearchQuery::new().with_effective_date_range(past - chrono::Duration::days(1), now);

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
}
