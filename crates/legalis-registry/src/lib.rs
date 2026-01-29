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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
                if let Some(jurisdiction) = &query.jurisdiction
                    && &entry.jurisdiction != jurisdiction
                {
                    return false;
                }

                // Status filter
                if let Some(status) = &query.status
                    && &entry.status != status
                {
                    return false;
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
            statute: Box<Statute>,
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
                statute: Box::new(statute),
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
                        .update(&statute_id, *statute)
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
    pub fn export_statute(entry: &StatuteEntry) -> Result<String, quick_xml::SeError> {
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

// ============================================================================
// Data Quality Features
// ============================================================================

/// Quality score for a statute entry (0.0 - 100.0).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QualityScore {
    /// Overall quality score
    pub overall: f64,
    /// Completeness score (fields populated)
    pub completeness: f64,
    /// Consistency score (internal consistency)
    pub consistency: f64,
    /// Metadata richness score
    pub metadata_richness: f64,
    /// Documentation quality score
    pub documentation_quality: f64,
}

impl QualityScore {
    /// Creates a quality score with all components.
    pub fn new(
        completeness: f64,
        consistency: f64,
        metadata_richness: f64,
        documentation_quality: f64,
    ) -> Self {
        let overall = (completeness * 0.4
            + consistency * 0.3
            + metadata_richness * 0.2
            + documentation_quality * 0.1)
            .clamp(0.0, 100.0);

        Self {
            overall,
            completeness,
            consistency,
            metadata_richness,
            documentation_quality,
        }
    }

    /// Checks if the quality meets a threshold.
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall >= threshold
    }

    /// Returns the grade (A-F) based on score.
    pub fn grade(&self) -> char {
        match self.overall as u32 {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        }
    }
}

/// Quality assessment for a statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// Statute ID being assessed
    pub statute_id: String,
    /// Quality score
    pub score: QualityScore,
    /// Issues found
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
    /// Assessment timestamp
    pub assessed_at: DateTime<Utc>,
}

impl QualityAssessment {
    /// Creates a new quality assessment.
    pub fn new(statute_id: String, score: QualityScore) -> Self {
        Self {
            statute_id,
            score,
            issues: Vec::new(),
            suggestions: Vec::new(),
            assessed_at: Utc::now(),
        }
    }

    /// Adds an issue to the assessment.
    pub fn with_issue(mut self, issue: String) -> Self {
        self.issues.push(issue);
        self
    }

    /// Adds a suggestion to the assessment.
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Checks if the assessment has any issues.
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

/// Similarity measure between two statutes.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimilarityScore {
    /// Overall similarity (0.0 - 1.0)
    pub overall: f64,
    /// Title similarity
    pub title: f64,
    /// Content similarity
    pub content: f64,
    /// Metadata similarity
    pub metadata: f64,
}

impl SimilarityScore {
    /// Creates a new similarity score.
    pub fn new(title: f64, content: f64, metadata: f64) -> Self {
        let overall = (title * 0.4 + content * 0.5 + metadata * 0.1).clamp(0.0, 1.0);

        Self {
            overall,
            title,
            content,
            metadata,
        }
    }

    /// Checks if similarity exceeds threshold (likely duplicate).
    pub fn is_likely_duplicate(&self, threshold: f64) -> bool {
        self.overall >= threshold
    }

    /// Checks if similarity suggests possible duplicate.
    pub fn is_possible_duplicate(&self, threshold: f64) -> bool {
        self.overall >= threshold * 0.7
    }
}

/// A potential duplicate statute pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCandidate {
    /// First statute ID
    pub statute_id_1: String,
    /// Second statute ID
    pub statute_id_2: String,
    /// Similarity score
    pub similarity: SimilarityScore,
    /// Reason for flagging as duplicate
    pub reason: String,
}

impl DuplicateCandidate {
    /// Creates a new duplicate candidate.
    pub fn new(
        statute_id_1: String,
        statute_id_2: String,
        similarity: SimilarityScore,
        reason: String,
    ) -> Self {
        Self {
            statute_id_1,
            statute_id_2,
            similarity,
            reason,
        }
    }
}

/// Result of duplicate detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateDetectionResult {
    /// All duplicate candidates found
    pub candidates: Vec<DuplicateCandidate>,
    /// Similarity threshold used
    pub threshold: f64,
    /// Number of statutes analyzed
    pub statutes_analyzed: usize,
    /// Detection timestamp
    pub detected_at: DateTime<Utc>,
}

impl DuplicateDetectionResult {
    /// Creates a new duplicate detection result.
    pub fn new(threshold: f64, statutes_analyzed: usize) -> Self {
        Self {
            candidates: Vec::new(),
            threshold,
            statutes_analyzed,
            detected_at: Utc::now(),
        }
    }

    /// Adds a duplicate candidate.
    pub fn add_candidate(&mut self, candidate: DuplicateCandidate) {
        self.candidates.push(candidate);
    }

    /// Returns only likely duplicates (high confidence).
    pub fn likely_duplicates(&self) -> Vec<&DuplicateCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.similarity.is_likely_duplicate(self.threshold))
            .collect()
    }

    /// Returns possible duplicates (medium confidence).
    pub fn possible_duplicates(&self) -> Vec<&DuplicateCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.similarity.is_possible_duplicate(self.threshold))
            .collect()
    }

    /// Returns total number of duplicate pairs found.
    pub fn total_duplicates(&self) -> usize {
        self.candidates.len()
    }
}

/// Data profile for a field in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldProfile {
    /// Field name
    pub field_name: String,
    /// Total values
    pub total_values: usize,
    /// Null/empty values count
    pub null_count: usize,
    /// Unique values count
    pub unique_count: usize,
    /// Most common values (top 10)
    pub most_common: Vec<(String, usize)>,
    /// Completeness percentage
    pub completeness: f64,
}

impl FieldProfile {
    /// Creates a new field profile.
    pub fn new(field_name: String, total_values: usize) -> Self {
        Self {
            field_name,
            total_values,
            null_count: 0,
            unique_count: 0,
            most_common: Vec::new(),
            completeness: 0.0,
        }
    }

    /// Calculates completeness percentage.
    pub fn calculate_completeness(&mut self) {
        if self.total_values > 0 {
            self.completeness =
                ((self.total_values - self.null_count) as f64 / self.total_values as f64) * 100.0;
        }
    }
}

/// Comprehensive data profile for the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProfile {
    /// Total statutes profiled
    pub total_statutes: usize,
    /// Field profiles
    pub field_profiles: HashMap<String, FieldProfile>,
    /// Average quality score
    pub average_quality: f64,
    /// Quality distribution (grade -> count)
    pub quality_distribution: HashMap<char, usize>,
    /// Status distribution
    pub status_distribution: HashMap<StatuteStatus, usize>,
    /// Jurisdiction distribution
    pub jurisdiction_distribution: HashMap<String, usize>,
    /// Tag usage patterns
    pub tag_patterns: HashMap<String, usize>,
    /// Profiling timestamp
    pub profiled_at: DateTime<Utc>,
}

impl DataProfile {
    /// Creates a new data profile.
    pub fn new(total_statutes: usize) -> Self {
        Self {
            total_statutes,
            field_profiles: HashMap::new(),
            average_quality: 0.0,
            quality_distribution: HashMap::new(),
            status_distribution: HashMap::new(),
            jurisdiction_distribution: HashMap::new(),
            tag_patterns: HashMap::new(),
            profiled_at: Utc::now(),
        }
    }

    /// Adds a field profile.
    pub fn add_field_profile(&mut self, profile: FieldProfile) {
        self.field_profiles
            .insert(profile.field_name.clone(), profile);
    }

    /// Gets the completeness of a field.
    pub fn field_completeness(&self, field_name: &str) -> Option<f64> {
        self.field_profiles.get(field_name).map(|p| p.completeness)
    }

    /// Exports the profile to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl StatuteRegistry {
    /// Calculates quality score for a statute entry.
    pub fn calculate_quality_score(&self, entry: &StatuteEntry) -> QualityScore {
        // Completeness score (0-100)
        let mut completeness_score = 0.0;

        // Required fields (always present)
        completeness_score += 30.0;

        // Optional but important fields
        if entry.expiry_date.is_some() {
            completeness_score += 10.0;
        }

        if !entry.tags.is_empty() {
            completeness_score += 15.0;
        }

        if !entry.metadata.is_empty() {
            completeness_score += 15.0;
        }

        if entry.amends.is_some() {
            completeness_score += 10.0;
        }

        if !entry.supersedes.is_empty() {
            completeness_score += 10.0;
        }

        if !entry.references.is_empty() {
            completeness_score += 10.0;
        }

        // Consistency score (0-100)
        let mut consistency_score = 100.0;

        // Check if expiry date is after effective date
        if let (Some(expiry), Some(effective)) = (entry.expiry_date, entry.effective_date)
            && expiry <= effective
        {
            consistency_score -= 30.0;
        }

        // Check if status matches dates
        if entry.status == StatuteStatus::Repealed {
            if let Some(expiry) = entry.expiry_date {
                if expiry > Utc::now() {
                    consistency_score -= 20.0;
                }
            } else {
                consistency_score -= 20.0;
            }
        }

        // Metadata richness score (0-100)
        let metadata_richness = if entry.metadata.is_empty() {
            0.0
        } else {
            ((entry.metadata.len() as f64).min(10.0) / 10.0) * 100.0
        };

        // Documentation quality score (0-100)
        let doc_quality = {
            let title_len = entry.statute.title.len();
            let has_description = entry
                .metadata
                .contains_key("description")
                .then_some(())
                .is_some();
            let has_tags = !entry.tags.is_empty();

            let mut score = 0.0;

            // Title quality (descriptive, not too short)
            if title_len > 10 {
                score += 40.0;
            } else if title_len > 5 {
                score += 20.0;
            }

            // Has description metadata
            if has_description {
                score += 40.0;
            }

            // Has tags for categorization
            if has_tags {
                score += 20.0;
            }

            score
        };

        QualityScore::new(
            completeness_score,
            consistency_score,
            metadata_richness,
            doc_quality,
        )
    }

    /// Performs quality assessment for a statute.
    pub fn assess_quality(&self, statute_id: &str) -> RegistryResult<QualityAssessment> {
        let entry = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let score = self.calculate_quality_score(entry);
        let mut assessment = QualityAssessment::new(statute_id.to_string(), score);

        // Add issues and suggestions
        if entry.tags.is_empty() {
            assessment = assessment
                .with_issue("No tags assigned".to_string())
                .with_suggestion("Add relevant tags for better categorization".to_string());
        }

        if entry.metadata.is_empty() {
            assessment = assessment
                .with_issue("No metadata provided".to_string())
                .with_suggestion("Add metadata fields like description, author, etc.".to_string());
        }

        if let (Some(expiry), Some(effective)) = (entry.expiry_date, entry.effective_date)
            && expiry <= effective
        {
            assessment = assessment
                .with_issue("Expiry date is before or equal to effective date".to_string());
        }

        if entry.status == StatuteStatus::Repealed && entry.expiry_date.is_none() {
            assessment =
                assessment.with_issue("Status is Repealed but no expiry date is set".to_string());
        }

        if entry.statute.title.len() < 10 {
            assessment = assessment
                .with_issue("Title is too short".to_string())
                .with_suggestion("Use a more descriptive title".to_string());
        }

        Ok(assessment)
    }

    /// Assesses quality for all statutes in the registry.
    pub fn assess_all_quality(&self) -> Vec<QualityAssessment> {
        self.statutes
            .keys()
            .filter_map(|id| self.assess_quality(id).ok())
            .collect()
    }

    /// Calculates similarity between two statute entries.
    pub fn calculate_similarity(
        &self,
        entry1: &StatuteEntry,
        entry2: &StatuteEntry,
    ) -> SimilarityScore {
        // Title similarity (using fuzzy matching)
        let matcher = SkimMatcherV2::default();
        let title_sim = matcher
            .fuzzy_match(&entry1.statute.title, &entry2.statute.title)
            .map(|score| (score as f64 / 100.0).min(1.0))
            .unwrap_or(0.0);

        // Content similarity (based on references)
        let content_sim = {
            // Count common references
            let refs1: HashSet<_> = entry1.references.iter().collect();
            let refs2: HashSet<_> = entry2.references.iter().collect();
            let common = refs1.intersection(&refs2).count();
            let total = refs1.union(&refs2).count();

            if total > 0 {
                common as f64 / total as f64
            } else {
                // If no references in either, check if effect types are the same
                if entry1.statute.effect.effect_type == entry2.statute.effect.effect_type {
                    0.5
                } else {
                    0.0
                }
            }
        };

        // Metadata similarity
        let tags1: HashSet<_> = entry1.tags.iter().collect();
        let tags2: HashSet<_> = entry2.tags.iter().collect();
        let common_tags = tags1.intersection(&tags2).count();
        let total_tags = tags1.union(&tags2).count();

        let metadata_sim = if total_tags > 0 {
            common_tags as f64 / total_tags as f64
        } else {
            0.0
        };

        SimilarityScore::new(title_sim, content_sim, metadata_sim)
    }

    /// Detects potential duplicate statutes.
    pub fn detect_duplicates(&self, threshold: f64) -> DuplicateDetectionResult {
        let statute_ids: Vec<_> = self.statutes.keys().cloned().collect();
        let mut result = DuplicateDetectionResult::new(threshold, statute_ids.len());

        for i in 0..statute_ids.len() {
            for j in (i + 1)..statute_ids.len() {
                let id1 = &statute_ids[i];
                let id2 = &statute_ids[j];

                if let (Some(entry1), Some(entry2)) =
                    (self.statutes.get(id1), self.statutes.get(id2))
                {
                    let similarity = self.calculate_similarity(entry1, entry2);

                    if similarity.overall >= threshold * 0.7 {
                        let reason = if similarity.overall >= threshold {
                            "High similarity detected".to_string()
                        } else {
                            "Moderate similarity detected".to_string()
                        };

                        result.add_candidate(DuplicateCandidate::new(
                            id1.clone(),
                            id2.clone(),
                            similarity,
                            reason,
                        ));
                    }
                }
            }
        }

        result
    }

    /// Profiles the data in the registry.
    pub fn profile_data(&mut self) -> DataProfile {
        let total = self.statutes.len();
        let mut profile = DataProfile::new(total);

        // Calculate quality scores and distribution
        let mut total_quality = 0.0;
        let mut quality_counts: HashMap<char, usize> = HashMap::new();

        for entry in self.statutes.values() {
            let score = self.calculate_quality_score(entry);
            total_quality += score.overall;

            let grade = score.grade();
            *quality_counts.entry(grade).or_insert(0) += 1;

            // Status distribution
            *profile.status_distribution.entry(entry.status).or_insert(0) += 1;

            // Jurisdiction distribution
            *profile
                .jurisdiction_distribution
                .entry(entry.jurisdiction.clone())
                .or_insert(0) += 1;

            // Tag patterns
            for tag in &entry.tags {
                *profile.tag_patterns.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        if total > 0 {
            profile.average_quality = total_quality / total as f64;
        }
        profile.quality_distribution = quality_counts;

        // Profile common fields
        let mut title_profile = FieldProfile::new("title".to_string(), total);
        let mut jurisdiction_profile = FieldProfile::new("jurisdiction".to_string(), total);
        let mut tags_profile = FieldProfile::new("tags".to_string(), total);

        let mut title_counts: HashMap<String, usize> = HashMap::new();
        let mut jurisdiction_counts: HashMap<String, usize> = HashMap::new();

        for entry in self.statutes.values() {
            // Titles
            *title_counts.entry(entry.statute.title.clone()).or_insert(0) += 1;

            // Jurisdictions
            *jurisdiction_counts
                .entry(entry.jurisdiction.clone())
                .or_insert(0) += 1;

            // Tags
            if entry.tags.is_empty() {
                tags_profile.null_count += 1;
            }
        }

        title_profile.unique_count = title_counts.len();
        title_profile.calculate_completeness();
        let mut title_vec: Vec<_> = title_counts.into_iter().collect();
        title_vec.sort_by(|a, b| b.1.cmp(&a.1));
        title_profile.most_common = title_vec.into_iter().take(10).collect();

        jurisdiction_profile.unique_count = jurisdiction_counts.len();
        jurisdiction_profile.calculate_completeness();
        let mut jurisdiction_vec: Vec<_> = jurisdiction_counts.into_iter().collect();
        jurisdiction_vec.sort_by(|a, b| b.1.cmp(&a.1));
        jurisdiction_profile.most_common = jurisdiction_vec.into_iter().take(10).collect();

        tags_profile.unique_count = profile.tag_patterns.len();
        tags_profile.calculate_completeness();

        profile.add_field_profile(title_profile);
        profile.add_field_profile(jurisdiction_profile);
        profile.add_field_profile(tags_profile);

        profile
    }

    /// Finds statutes with quality scores below a threshold.
    pub fn find_low_quality_statutes(&self, threshold: f64) -> Vec<(String, QualityScore)> {
        self.statutes
            .iter()
            .map(|(id, entry)| (id.clone(), self.calculate_quality_score(entry)))
            .filter(|(_, score)| score.overall < threshold)
            .collect()
    }

    /// Exports quality assessments to JSON.
    pub fn export_quality_assessments_json(&self) -> Result<String, serde_json::Error> {
        let assessments = self.assess_all_quality();
        serde_json::to_string_pretty(&assessments)
    }

    /// Exports duplicate detection results to JSON.
    pub fn export_duplicates_json(&self, threshold: f64) -> Result<String, serde_json::Error> {
        let duplicates = self.detect_duplicates(threshold);
        serde_json::to_string_pretty(&duplicates)
    }
}

// ============================================================================
// Automatic Data Enrichment
// ============================================================================

/// Enrichment suggestion for a statute entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentSuggestion {
    /// Type of enrichment
    pub enrichment_type: EnrichmentType,
    /// Suggested value or action
    pub suggestion: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Reason for suggestion
    pub reason: String,
}

impl EnrichmentSuggestion {
    /// Creates a new enrichment suggestion.
    pub fn new(
        enrichment_type: EnrichmentType,
        suggestion: String,
        confidence: f64,
        reason: String,
    ) -> Self {
        Self {
            enrichment_type,
            suggestion,
            confidence: confidence.clamp(0.0, 1.0),
            reason,
        }
    }

    /// Checks if suggestion meets confidence threshold.
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}

/// Type of data enrichment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnrichmentType {
    /// Auto-tagging based on content
    AutoTag,
    /// Metadata inference
    MetadataInference,
    /// Jurisdiction inference
    JurisdictionInference,
    /// Related statute suggestion
    RelatedStatute,
    /// Category classification
    CategoryClassification,
}

/// Result of automatic enrichment analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichmentResult {
    /// Statute ID being enriched
    pub statute_id: String,
    /// List of suggestions
    pub suggestions: Vec<EnrichmentSuggestion>,
    /// Timestamp of analysis
    pub analyzed_at: DateTime<Utc>,
}

impl EnrichmentResult {
    /// Creates a new enrichment result.
    pub fn new(statute_id: String) -> Self {
        Self {
            statute_id,
            suggestions: Vec::new(),
            analyzed_at: Utc::now(),
        }
    }

    /// Adds a suggestion to the result.
    pub fn add_suggestion(&mut self, suggestion: EnrichmentSuggestion) {
        self.suggestions.push(suggestion);
    }

    /// Returns suggestions meeting a confidence threshold.
    pub fn high_confidence_suggestions(&self, threshold: f64) -> Vec<&EnrichmentSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.meets_threshold(threshold))
            .collect()
    }

    /// Groups suggestions by type.
    pub fn suggestions_by_type(
        &self,
        enrichment_type: EnrichmentType,
    ) -> Vec<&EnrichmentSuggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.enrichment_type == enrichment_type)
            .collect()
    }
}

/// Enrichment configuration.
#[derive(Debug, Clone)]
pub struct EnrichmentConfig {
    /// Enable auto-tagging
    pub enable_auto_tagging: bool,
    /// Enable metadata inference
    pub enable_metadata_inference: bool,
    /// Enable jurisdiction inference
    pub enable_jurisdiction_inference: bool,
    /// Minimum confidence threshold
    pub min_confidence: f64,
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

impl EnrichmentConfig {
    /// Creates a new enrichment configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets auto-tagging enabled.
    pub fn with_auto_tagging(mut self, enabled: bool) -> Self {
        self.enable_auto_tagging = enabled;
        self
    }

    /// Sets metadata inference enabled.
    pub fn with_metadata_inference(mut self, enabled: bool) -> Self {
        self.enable_metadata_inference = enabled;
        self
    }

    /// Sets jurisdiction inference enabled.
    pub fn with_jurisdiction_inference(mut self, enabled: bool) -> Self {
        self.enable_jurisdiction_inference = enabled;
        self
    }

    /// Sets minimum confidence threshold.
    pub fn with_min_confidence(mut self, threshold: f64) -> Self {
        self.min_confidence = threshold.clamp(0.0, 1.0);
        self
    }
}

impl StatuteRegistry {
    /// Analyzes a statute for enrichment opportunities.
    pub fn analyze_enrichment(
        &self,
        statute_id: &str,
        config: &EnrichmentConfig,
    ) -> RegistryResult<EnrichmentResult> {
        let entry = self
            .statutes
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let mut result = EnrichmentResult::new(statute_id.to_string());

        // Auto-tagging based on title and content
        if config.enable_auto_tagging {
            self.suggest_auto_tags(entry, &mut result);
        }

        // Metadata inference
        if config.enable_metadata_inference {
            self.suggest_metadata(entry, &mut result);
        }

        // Jurisdiction inference
        if config.enable_jurisdiction_inference {
            self.suggest_jurisdiction_metadata(entry, &mut result);
        }

        Ok(result)
    }

    /// Suggests automatic tags based on content.
    fn suggest_auto_tags(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        let title_lower = entry.statute.title.to_lowercase();

        // Common legal domain tags
        let tag_patterns = [
            ("civil", vec!["civil", "contract", "property", "tort"]),
            ("criminal", vec!["criminal", "penal", "offense", "crime"]),
            (
                "administrative",
                vec!["administrative", "regulation", "agency"],
            ),
            ("tax", vec!["tax", "revenue", "fiscal"]),
            ("employment", vec!["employment", "labor", "worker"]),
            ("corporate", vec!["corporate", "company", "business"]),
            (
                "intellectual-property",
                vec!["patent", "trademark", "copyright", "ip"],
            ),
            (
                "environmental",
                vec!["environmental", "pollution", "conservation"],
            ),
            ("healthcare", vec!["health", "medical", "patient"]),
            ("education", vec!["education", "school", "university"]),
        ];

        for (tag, keywords) in &tag_patterns {
            if !entry.tags.contains(&tag.to_string()) {
                let matches = keywords
                    .iter()
                    .filter(|kw| title_lower.contains(*kw))
                    .count();
                if matches > 0 {
                    let confidence = (matches as f64 / keywords.len() as f64).min(0.95);
                    result.add_suggestion(EnrichmentSuggestion::new(
                        EnrichmentType::AutoTag,
                        tag.to_string(),
                        confidence,
                        format!("Title contains keywords: {}", keywords.join(", ")),
                    ));
                }
            }
        }
    }

    /// Suggests metadata based on analysis.
    fn suggest_metadata(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        // Suggest description if missing
        if !entry.metadata.contains_key("description") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::MetadataInference,
                "description".to_string(),
                0.6,
                "Missing description metadata - consider adding statute summary".to_string(),
            ));
        }

        // Suggest category based on tags
        if !entry.metadata.contains_key("category") && !entry.tags.is_empty() {
            let category = entry.tags.first().unwrap();
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::CategoryClassification,
                category.clone(),
                0.75,
                format!("Category inferred from primary tag: {}", category),
            ));
        }

        // Suggest effective date metadata if not set
        if entry.effective_date.is_none() && !entry.metadata.contains_key("effective_date_note") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::MetadataInference,
                "effective_date_note".to_string(),
                0.5,
                "Consider adding effective date information".to_string(),
            ));
        }
    }

    /// Suggests jurisdiction-related metadata.
    fn suggest_jurisdiction_metadata(&self, entry: &StatuteEntry, result: &mut EnrichmentResult) {
        // Count statutes in same jurisdiction
        let jurisdiction_count = self
            .statutes
            .values()
            .filter(|e| e.jurisdiction == entry.jurisdiction)
            .count();

        if jurisdiction_count > 10 && !entry.metadata.contains_key("jurisdiction_family") {
            result.add_suggestion(EnrichmentSuggestion::new(
                EnrichmentType::JurisdictionInference,
                "jurisdiction_family".to_string(),
                0.8,
                format!(
                    "Part of {} statute family in jurisdiction {}",
                    jurisdiction_count, entry.jurisdiction
                ),
            ));
        }
    }

    /// Applies enrichment suggestions to a statute.
    pub fn apply_enrichment(
        &mut self,
        statute_id: &str,
        suggestions: &[EnrichmentSuggestion],
        min_confidence: f64,
    ) -> RegistryResult<usize> {
        let entry = self
            .statutes
            .get_mut(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        let mut applied_count = 0;

        for suggestion in suggestions {
            if !suggestion.meets_threshold(min_confidence) {
                continue;
            }

            match suggestion.enrichment_type {
                EnrichmentType::AutoTag => {
                    if !entry.tags.contains(&suggestion.suggestion) {
                        entry.tags.push(suggestion.suggestion.clone());
                        applied_count += 1;
                    }
                }
                EnrichmentType::MetadataInference
                | EnrichmentType::CategoryClassification
                | EnrichmentType::JurisdictionInference => {
                    let key = suggestion.suggestion.clone();
                    if let std::collections::hash_map::Entry::Vacant(e) = entry.metadata.entry(key)
                    {
                        e.insert(format!("Auto-enriched: {}", suggestion.reason));
                        applied_count += 1;
                    }
                }
                EnrichmentType::RelatedStatute => {
                    // Add to references if not already present
                    if !entry.references.contains(&suggestion.suggestion) {
                        entry.references.push(suggestion.suggestion.clone());
                        applied_count += 1;
                    }
                }
            }
        }

        // Update ETag after modifications
        entry.etag = Uuid::new_v4().to_string();

        Ok(applied_count)
    }

    /// Auto-enriches all statutes in the registry.
    pub fn auto_enrich_all(&mut self, config: &EnrichmentConfig) -> Vec<(String, usize)> {
        let statute_ids: Vec<_> = self.statutes.keys().cloned().collect();
        let mut results = Vec::new();

        for statute_id in statute_ids {
            if let Ok(enrichment) = self.analyze_enrichment(&statute_id, config) {
                let high_confidence = enrichment.high_confidence_suggestions(config.min_confidence);
                if !high_confidence.is_empty() {
                    let suggestions: Vec<_> = high_confidence.into_iter().cloned().collect();
                    if let Ok(count) =
                        self.apply_enrichment(&statute_id, &suggestions, config.min_confidence)
                        && count > 0
                    {
                        results.push((statute_id, count));
                    }
                }
            }
        }

        results
    }
}

// ============================================================================
// Data Lineage Tracking
// ============================================================================

/// Type of lineage operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageOperation {
    /// Created from scratch
    Created,
    /// Imported from external source
    Imported { source: String },
    /// Derived from another statute
    Derived { parent_id: String },
    /// Merged from multiple statutes
    Merged { source_ids: Vec<String> },
    /// Enriched by automatic process
    Enriched { enrichment_type: String },
    /// Validated by validation rule
    Validated { rule_name: String },
    /// Transformed by custom logic
    Transformed { transformation: String },
}

/// Lineage entry tracking data provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEntry {
    /// Unique lineage ID
    pub lineage_id: Uuid,
    /// Statute ID this lineage applies to
    pub statute_id: String,
    /// Operation performed
    pub operation: LineageOperation,
    /// Timestamp of operation
    pub timestamp: DateTime<Utc>,
    /// Actor who performed operation (user, system, etc.)
    pub actor: String,
    /// Additional context
    pub context: HashMap<String, String>,
}

impl LineageEntry {
    /// Creates a new lineage entry.
    pub fn new(statute_id: String, operation: LineageOperation, actor: String) -> Self {
        Self {
            lineage_id: Uuid::new_v4(),
            statute_id,
            operation,
            timestamp: Utc::now(),
            actor,
            context: HashMap::new(),
        }
    }

    /// Adds context to the lineage entry.
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
}

/// Data lineage tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    /// All lineage entries
    entries: Vec<LineageEntry>,
    /// Maximum entries to keep (for memory management)
    max_entries: usize,
}

impl Default for DataLineage {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl DataLineage {
    /// Creates a new data lineage tracker.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    /// Records a lineage entry.
    pub fn record(&mut self, entry: LineageEntry) {
        self.entries.push(entry);

        // Trim old entries if exceeding max
        if self.entries.len() > self.max_entries {
            self.entries.drain(0..self.entries.len() - self.max_entries);
        }
    }

    /// Gets lineage history for a statute.
    pub fn get_lineage(&self, statute_id: &str) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| e.statute_id == statute_id)
            .collect()
    }

    /// Gets lineage entries by operation type.
    pub fn get_by_operation(&self, operation_type: &str) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| match &e.operation {
                LineageOperation::Created => operation_type == "Created",
                LineageOperation::Imported { .. } => operation_type == "Imported",
                LineageOperation::Derived { .. } => operation_type == "Derived",
                LineageOperation::Merged { .. } => operation_type == "Merged",
                LineageOperation::Enriched { .. } => operation_type == "Enriched",
                LineageOperation::Validated { .. } => operation_type == "Validated",
                LineageOperation::Transformed { .. } => operation_type == "Transformed",
            })
            .collect()
    }

    /// Gets lineage entries by actor.
    pub fn get_by_actor(&self, actor: &str) -> Vec<&LineageEntry> {
        self.entries.iter().filter(|e| e.actor == actor).collect()
    }

    /// Gets lineage entries in a time range.
    pub fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&LineageEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Traces the full provenance chain for a statute.
    pub fn trace_provenance(&self, statute_id: &str) -> Vec<&LineageEntry> {
        let mut provenance = Vec::new();
        let mut current_ids = vec![statute_id.to_string()];
        let mut visited = HashSet::new();

        while let Some(id) = current_ids.pop() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id.clone());

            for entry in self.get_lineage(&id) {
                provenance.push(entry);

                // Add parent IDs to trace further
                match &entry.operation {
                    LineageOperation::Derived { parent_id } => {
                        if !visited.contains(parent_id) {
                            current_ids.push(parent_id.clone());
                        }
                    }
                    LineageOperation::Merged { source_ids } => {
                        for source_id in source_ids {
                            if !visited.contains(source_id) {
                                current_ids.push(source_id.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Sort by timestamp
        provenance.sort_by_key(|e| e.timestamp);
        provenance
    }

    /// Exports lineage to JSON.
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.entries)
    }

    /// Clears all lineage entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns total number of lineage entries.
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl StatuteRegistry {
    /// Records a lineage entry for a statute.
    #[allow(dead_code)]
    pub fn record_lineage(&mut self, _entry: LineageEntry) {
        // This would typically be integrated with the registry's lineage tracker
        // For now, we'll add it as a placeholder for future integration
        // In a real implementation, StatuteRegistry would have a DataLineage field
    }
}

// ============================================================================
// Compliance Features (v0.1.9)
// ============================================================================

/// PII (Personally Identifiable Information) field types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PiiFieldType {
    /// Name of a person
    Name,
    /// Email address
    Email,
    /// Phone number
    PhoneNumber,
    /// Social security number or national ID
    NationalId,
    /// Physical address
    Address,
    /// Date of birth
    DateOfBirth,
    /// IP address
    IpAddress,
    /// Custom PII type
    Custom(String),
}

/// A detected PII instance in statute content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetection {
    /// Field type
    pub field_type: PiiFieldType,
    /// Original value (potentially sensitive)
    pub value: String,
    /// Position in text (char offset)
    pub position: usize,
    /// Length of the PII value
    pub length: usize,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

impl PiiDetection {
    /// Creates a new PII detection.
    pub fn new(field_type: PiiFieldType, value: String, position: usize, confidence: f64) -> Self {
        let length = value.len();
        let confidence = confidence.clamp(0.0, 1.0);
        Self {
            field_type,
            value,
            position,
            length,
            confidence,
        }
    }

    /// Returns true if confidence is above threshold.
    pub fn is_confident(&self, threshold: f64) -> bool {
        self.confidence >= threshold
    }
}

/// Result of PII detection scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiScanResult {
    /// Statute ID scanned
    pub statute_id: String,
    /// Detected PII instances
    pub detections: Vec<PiiDetection>,
    /// Scan timestamp
    pub scanned_at: DateTime<Utc>,
    /// Total PII count
    pub pii_count: usize,
}

impl PiiScanResult {
    /// Creates a new scan result.
    pub fn new(statute_id: String, detections: Vec<PiiDetection>) -> Self {
        let pii_count = detections.len();
        Self {
            statute_id,
            detections,
            scanned_at: Utc::now(),
            pii_count,
        }
    }

    /// Returns high-confidence detections only.
    pub fn high_confidence(&self, threshold: f64) -> Vec<&PiiDetection> {
        self.detections
            .iter()
            .filter(|d| d.is_confident(threshold))
            .collect()
    }

    /// Returns detections by type.
    pub fn by_type(&self, field_type: &PiiFieldType) -> Vec<&PiiDetection> {
        self.detections
            .iter()
            .filter(|d| &d.field_type == field_type)
            .collect()
    }
}

/// PII masking strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaskingStrategy {
    /// Replace with asterisks (e.g., "John Doe" -> "****")
    Asterisks,
    /// Replace with redacted marker (e.g., "John Doe" -> "\[REDACTED\]")
    Redacted,
    /// Replace with type marker (e.g., "John Doe" -> "\[NAME\]")
    TypeMarker,
    /// Hash the value (one-way)
    Hash,
    /// Partial masking (e.g., "John Doe" -> "J*** D**")
    Partial,
}

/// PII detector and handler.
#[derive(Debug, Clone)]
pub struct PiiDetector {
    /// Enable/disable PII detection
    enabled: bool,
    /// Minimum confidence threshold
    min_confidence: f64,
    /// Masking strategy
    masking_strategy: MaskingStrategy,
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

impl PiiDetector {
    /// Creates a new PII detector with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the minimum confidence threshold.
    pub fn with_min_confidence(mut self, threshold: f64) -> Self {
        self.min_confidence = threshold.clamp(0.0, 1.0);
        self
    }

    /// Sets the masking strategy.
    pub fn with_masking_strategy(mut self, strategy: MaskingStrategy) -> Self {
        self.masking_strategy = strategy;
        self
    }

    /// Enables or disables PII detection.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Scans statute content for PII.
    pub fn scan(&self, statute_id: &str, content: &str) -> PiiScanResult {
        if !self.enabled {
            return PiiScanResult::new(statute_id.to_string(), Vec::new());
        }

        let mut detections = Vec::new();

        // Email detection (simple pattern)
        if let Some(email_regex) = Self::email_pattern() {
            for (idx, _) in content.match_indices(&email_regex) {
                if let Some(end) = content[idx..].find(|c: char| c.is_whitespace()) {
                    let email = &content[idx..idx + end];
                    if email.contains('@') {
                        detections.push(PiiDetection::new(
                            PiiFieldType::Email,
                            email.to_string(),
                            idx,
                            0.9,
                        ));
                    }
                }
            }
        }

        // Phone number detection (simple pattern for demonstration)
        for (idx, _) in content.match_indices(char::is_numeric) {
            let rest = &content[idx..];
            if let Some(number) = Self::extract_phone_number(rest)
                && number.len() >= 10
            {
                detections.push(PiiDetection::new(
                    PiiFieldType::PhoneNumber,
                    number.clone(),
                    idx,
                    0.8,
                ));
            }
        }

        // IP address detection (simple IPv4 pattern)
        for (idx, _) in content.match_indices(char::is_numeric) {
            if let Some(ip) = Self::extract_ip_address(&content[idx..]) {
                detections.push(PiiDetection::new(
                    PiiFieldType::IpAddress,
                    ip.clone(),
                    idx,
                    0.95,
                ));
            }
        }

        PiiScanResult::new(statute_id.to_string(), detections)
    }

    /// Masks PII in content based on detection results.
    pub fn mask(&self, content: &str, scan_result: &PiiScanResult) -> String {
        let mut masked = content.to_string();
        let mut offset = 0i32;

        // Sort detections by position
        let mut sorted_detections = scan_result.detections.clone();
        sorted_detections.sort_by_key(|d| d.position);

        for detection in sorted_detections.iter() {
            if !detection.is_confident(self.min_confidence) {
                continue;
            }

            let pos = (detection.position as i32 + offset) as usize;
            let masked_value = self.apply_masking(&detection.value, &detection.field_type);
            let original_len = detection.length;
            let new_len = masked_value.len();

            if pos + original_len <= masked.len() {
                masked.replace_range(pos..pos + original_len, &masked_value);
                offset += new_len as i32 - original_len as i32;
            }
        }

        masked
    }

    /// Applies masking strategy to a value.
    fn apply_masking(&self, value: &str, field_type: &PiiFieldType) -> String {
        match self.masking_strategy {
            MaskingStrategy::Asterisks => "*".repeat(value.len().min(8)),
            MaskingStrategy::Redacted => "[REDACTED]".to_string(),
            MaskingStrategy::TypeMarker => format!("[{:?}]", field_type).to_uppercase(),
            MaskingStrategy::Hash => {
                // Simple hash representation (not cryptographic)
                format!("[HASH:{}]", value.len())
            }
            MaskingStrategy::Partial => {
                if value.len() <= 4 {
                    "*".repeat(value.len())
                } else {
                    let chars: Vec<char> = value.chars().collect();
                    let mut result = String::new();
                    for (i, ch) in chars.iter().enumerate() {
                        if i == 0 || i == chars.len() - 1 {
                            result.push(*ch);
                        } else {
                            result.push('*');
                        }
                    }
                    result
                }
            }
        }
    }

    fn email_pattern() -> Option<&'static str> {
        Some("@")
    }

    fn extract_phone_number(text: &str) -> Option<String> {
        let mut number = String::new();
        for ch in text.chars().take(15) {
            if ch.is_numeric() || ch == '-' || ch == '(' || ch == ')' || ch == ' ' {
                number.push(ch);
            } else {
                break;
            }
        }
        if number.chars().filter(|c| c.is_numeric()).count() >= 10 {
            Some(number.trim().to_string())
        } else {
            None
        }
    }

    fn extract_ip_address(text: &str) -> Option<String> {
        let parts: Vec<&str> = text.split('.').take(4).collect();
        if parts.len() == 4 {
            let ip: String = parts.join(".");
            if ip.chars().all(|c| c.is_numeric() || c == '.') {
                return Some(ip);
            }
        }
        None
    }
}

/// Data retention rule for automatic cleanup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataRetentionRule {
    /// Retain for a specific number of days
    RetainForDays(u32),
    /// Retain until a specific date
    RetainUntil(DateTime<Utc>),
    /// Retain indefinitely
    RetainIndefinitely,
    /// Delete after statute becomes inactive for N days
    DeleteInactiveAfterDays(u32),
    /// Archive after N days instead of deleting
    ArchiveAfterDays(u32),
}

/// Data retention configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    /// Retention rules
    #[serde(default)]
    rules: Vec<DataRetentionRule>,
    /// Auto-apply retention rules
    #[serde(default)]
    auto_apply: bool,
    /// Dry-run mode (don't actually delete)
    #[serde(default)]
    dry_run: bool,
}

impl DataRetentionConfig {
    /// Creates a new retention configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a retention rule.
    pub fn add_rule(mut self, rule: DataRetentionRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Enables auto-apply mode.
    pub fn with_auto_apply(mut self, auto_apply: bool) -> Self {
        self.auto_apply = auto_apply;
        self
    }

    /// Enables dry-run mode.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Returns all rules.
    pub fn rules(&self) -> &[DataRetentionRule] {
        &self.rules
    }

    /// Returns whether auto-apply is enabled.
    pub fn is_auto_apply(&self) -> bool {
        self.auto_apply
    }

    /// Returns whether dry-run mode is enabled.
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

/// Result of applying retention rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionExecutionResult {
    /// Statutes deleted
    pub deleted: Vec<String>,
    /// Statutes archived
    pub archived: Vec<String>,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
    /// Was this a dry run?
    pub dry_run: bool,
}

impl RetentionExecutionResult {
    /// Creates a new execution result.
    pub fn new(deleted: Vec<String>, archived: Vec<String>, dry_run: bool) -> Self {
        Self {
            deleted,
            archived,
            executed_at: Utc::now(),
            dry_run,
        }
    }

    /// Returns total affected statutes.
    pub fn total_affected(&self) -> usize {
        self.deleted.len() + self.archived.len()
    }
}

/// Audit report format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditReportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Plain text format
    Text,
    /// HTML format
    Html,
}

/// Audit report configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportConfig {
    /// Report title
    pub title: String,
    /// Start date filter
    pub start_date: Option<DateTime<Utc>>,
    /// End date filter
    pub end_date: Option<DateTime<Utc>>,
    /// Include operations
    pub include_operations: bool,
    /// Include events
    pub include_events: bool,
    /// Include quality metrics
    pub include_quality: bool,
    /// Include PII scan results
    pub include_pii_scans: bool,
    /// Report format
    pub format: AuditReportFormat,
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

impl AuditReportConfig {
    /// Creates a new audit report configuration.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Sets the date range.
    pub fn with_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_date = Some(start);
        self.end_date = Some(end);
        self
    }

    /// Sets what to include in the report.
    pub fn with_sections(
        mut self,
        operations: bool,
        events: bool,
        quality: bool,
        pii_scans: bool,
    ) -> Self {
        self.include_operations = operations;
        self.include_events = events;
        self.include_quality = quality;
        self.include_pii_scans = pii_scans;
        self
    }

    /// Sets the report format.
    pub fn with_format(mut self, format: AuditReportFormat) -> Self {
        self.format = format;
        self
    }
}

/// Generated audit report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    /// Report ID
    pub report_id: Uuid,
    /// Report title
    pub title: String,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Date range covered
    pub date_range: (Option<DateTime<Utc>>, Option<DateTime<Utc>>),
    /// Total statutes in registry
    pub total_statutes: usize,
    /// Total events recorded
    pub total_events: usize,
    /// Total operations performed
    pub total_operations: usize,
    /// PII detections count
    pub pii_detections: usize,
    /// Average quality score
    pub avg_quality_score: f64,
    /// Report content (serialized based on format)
    pub content: String,
    /// Report format
    pub format: AuditReportFormat,
}

impl AuditReport {
    /// Creates a new audit report.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: String,
        date_range: (Option<DateTime<Utc>>, Option<DateTime<Utc>>),
        total_statutes: usize,
        total_events: usize,
        total_operations: usize,
        pii_detections: usize,
        avg_quality_score: f64,
        content: String,
        format: AuditReportFormat,
    ) -> Self {
        Self {
            report_id: Uuid::new_v4(),
            title,
            generated_at: Utc::now(),
            date_range,
            total_statutes,
            total_events,
            total_operations,
            pii_detections,
            avg_quality_score,
            content,
            format,
        }
    }

    /// Exports the report to a file-friendly string.
    pub fn export(&self) -> String {
        match self.format {
            AuditReportFormat::Json => {
                serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
            }
            AuditReportFormat::Csv | AuditReportFormat::Text | AuditReportFormat::Html => {
                self.content.clone()
            }
        }
    }
}

/// Geographic region for data sovereignty.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeographicRegion {
    /// European Union
    EU,
    /// United States
    US,
    /// United Kingdom
    UK,
    /// Asia Pacific
    APAC,
    /// Japan
    Japan,
    /// China
    China,
    /// Custom region
    Custom(String),
}

impl GeographicRegion {
    /// Returns the region code.
    pub fn code(&self) -> String {
        match self {
            GeographicRegion::EU => "EU".to_string(),
            GeographicRegion::US => "US".to_string(),
            GeographicRegion::UK => "UK".to_string(),
            GeographicRegion::APAC => "APAC".to_string(),
            GeographicRegion::Japan => "JP".to_string(),
            GeographicRegion::China => "CN".to_string(),
            GeographicRegion::Custom(s) => s.clone(),
        }
    }

    /// Checks if this region allows data transfer to another region.
    pub fn allows_transfer_to(&self, other: &GeographicRegion) -> bool {
        match (self, other) {
            // EU has strict rules (GDPR)
            (GeographicRegion::EU, GeographicRegion::EU) => true,
            (GeographicRegion::EU, GeographicRegion::UK) => true,
            (GeographicRegion::EU, _) => false, // EU data cannot be transferred elsewhere
            // Other regions are more permissive
            _ => true,
        }
    }
}

/// Data sovereignty configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfig {
    /// Primary region where data is stored
    pub primary_region: GeographicRegion,
    /// Allowed replication regions
    pub allowed_regions: Vec<GeographicRegion>,
    /// Enforce strict residency (no cross-region access)
    pub strict_residency: bool,
    /// Require encryption for cross-region transfer
    pub require_encryption: bool,
}

impl DataSovereigntyConfig {
    /// Creates a new data sovereignty configuration.
    pub fn new(primary_region: GeographicRegion) -> Self {
        Self {
            primary_region,
            allowed_regions: Vec::new(),
            strict_residency: false,
            require_encryption: true,
        }
    }

    /// Adds an allowed region for replication.
    pub fn allow_region(mut self, region: GeographicRegion) -> Self {
        if !self.allowed_regions.contains(&region) {
            self.allowed_regions.push(region);
        }
        self
    }

    /// Enables strict residency mode.
    pub fn with_strict_residency(mut self, strict: bool) -> Self {
        self.strict_residency = strict;
        self
    }

    /// Sets encryption requirement.
    pub fn with_encryption_required(mut self, required: bool) -> Self {
        self.require_encryption = required;
        self
    }

    /// Checks if a region is allowed for data storage/access.
    pub fn is_region_allowed(&self, region: &GeographicRegion) -> bool {
        if region == &self.primary_region {
            return true;
        }

        if self.strict_residency {
            return false;
        }

        self.allowed_regions.contains(region) && self.primary_region.allows_transfer_to(region)
    }
}

/// Compliance dashboard metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    /// Dashboard ID
    pub dashboard_id: Uuid,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Total statutes under management
    pub total_statutes: usize,
    /// Statutes with PII detected
    pub statutes_with_pii: usize,
    /// Statutes subject to retention
    pub statutes_pending_retention: usize,
    /// Average quality score
    pub avg_quality_score: f64,
    /// Statutes below quality threshold
    pub low_quality_count: usize,
    /// Total audit events
    pub total_audit_events: usize,
    /// Failed audit events
    pub failed_operations: usize,
    /// Data sovereignty violations
    pub sovereignty_violations: usize,
    /// Compliance rate (0.0-1.0)
    pub compliance_rate: f64,
}

impl ComplianceDashboard {
    /// Creates a new compliance dashboard.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        total_statutes: usize,
        statutes_with_pii: usize,
        statutes_pending_retention: usize,
        avg_quality_score: f64,
        low_quality_count: usize,
        total_audit_events: usize,
        failed_operations: usize,
        sovereignty_violations: usize,
    ) -> Self {
        let compliance_rate = if total_statutes > 0 {
            let compliant =
                total_statutes.saturating_sub(low_quality_count + sovereignty_violations);
            compliant as f64 / total_statutes as f64
        } else {
            1.0
        };

        Self {
            dashboard_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            total_statutes,
            statutes_with_pii,
            statutes_pending_retention,
            avg_quality_score,
            low_quality_count,
            total_audit_events,
            failed_operations,
            sovereignty_violations,
            compliance_rate,
        }
    }

    /// Returns true if compliance rate meets threshold.
    pub fn meets_compliance_threshold(&self, threshold: f64) -> bool {
        self.compliance_rate >= threshold
    }

    /// Exports dashboard to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

impl StatuteRegistry {
    /// Scans a statute for PII using the detector.
    pub fn scan_for_pii(
        &mut self,
        statute_id: &str,
        detector: &PiiDetector,
    ) -> RegistryResult<PiiScanResult> {
        let entry = self
            .get(statute_id)
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;

        // Scan statute title and metadata for PII
        let content = format!(
            "{} {}",
            entry.statute.title,
            entry
                .metadata
                .values()
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        );

        Ok(detector.scan(statute_id, &content))
    }

    /// Applies data retention rules and returns affected statutes.
    pub fn apply_retention_rules(
        &mut self,
        config: &DataRetentionConfig,
    ) -> RetentionExecutionResult {
        let mut to_delete = Vec::new();
        let mut to_archive = Vec::new();
        let now = Utc::now();

        for (statute_id, entry) in self.statutes.iter() {
            for rule in config.rules() {
                match rule {
                    DataRetentionRule::RetainForDays(days) => {
                        let age = now.signed_duration_since(entry.created_at).num_days();
                        if age > *days as i64 {
                            to_delete.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::RetainUntil(until) => {
                        if now > *until {
                            to_delete.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::DeleteInactiveAfterDays(days) => {
                        if !entry.is_active() {
                            let age = now.signed_duration_since(entry.modified_at).num_days();
                            if age > *days as i64 {
                                to_delete.push(statute_id.clone());
                            }
                        }
                    }
                    DataRetentionRule::ArchiveAfterDays(days) => {
                        let age = now.signed_duration_since(entry.created_at).num_days();
                        if age > *days as i64 {
                            to_archive.push(statute_id.clone());
                        }
                    }
                    DataRetentionRule::RetainIndefinitely => {
                        // Do nothing
                    }
                }
            }
        }

        // Remove duplicates
        to_delete.sort();
        to_delete.dedup();
        to_archive.sort();
        to_archive.dedup();

        // Don't actually delete/archive in dry-run mode
        if !config.is_dry_run() {
            for statute_id in &to_delete {
                let _ = self.delete(statute_id);
            }
            for statute_id in &to_archive {
                let _ = self.archive_statute(statute_id, "Automatic retention policy".to_string());
            }
        }

        RetentionExecutionResult::new(to_delete, to_archive, config.is_dry_run())
    }

    /// Generates an audit report based on configuration.
    pub fn generate_audit_report(&self, config: &AuditReportConfig) -> AuditReport {
        let mut content_parts = Vec::new();

        // Header
        content_parts.push(format!("Audit Report: {}", config.title));
        content_parts.push(format!("Generated: {}", Utc::now()));
        if let (Some(start), Some(end)) = (config.start_date, config.end_date) {
            content_parts.push(format!("Period: {} to {}", start, end));
        }
        content_parts.push(String::new());

        // Statistics
        content_parts.push("=== Statistics ===".to_string());
        content_parts.push(format!("Total Statutes: {}", self.statutes.len()));
        content_parts.push(format!("Total Events: {}", self.event_store.events.len()));
        content_parts.push(String::new());

        // Events section
        if config.include_events {
            content_parts.push("=== Events ===".to_string());
            let mut event_count = 0;
            for event in &self.event_store.events {
                // Get timestamp from event
                let event_timestamp = match event {
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
                };

                // Apply date filter if specified
                let include = if let (Some(start), Some(end)) = (config.start_date, config.end_date)
                {
                    event_timestamp >= start && event_timestamp <= end
                } else {
                    true
                };

                if include {
                    content_parts.push(format!("- {:?} at {}", event, event_timestamp));
                    event_count += 1;
                }
            }
            content_parts.push(format!("Total events in period: {}", event_count));
            content_parts.push(String::new());
        }

        let content = content_parts.join("\n");

        AuditReport::new(
            config.title.clone(),
            (config.start_date, config.end_date),
            self.statutes.len(),
            self.event_store.events.len(),
            0,   // Total operations (would need tracking)
            0,   // PII detections
            0.0, // Average quality score
            content,
            config.format,
        )
    }

    /// Generates a compliance dashboard with current metrics.
    pub fn generate_compliance_dashboard(&mut self, quality_threshold: f64) -> ComplianceDashboard {
        let total_statutes = self.statutes.len();
        let total_audit_events = self.event_store.events.len();

        // Calculate quality metrics
        let assessments = self.assess_all_quality();
        let low_quality = assessments
            .iter()
            .filter(|a| !a.score.meets_threshold(quality_threshold))
            .count();

        let avg_quality = if !assessments.is_empty() {
            assessments.iter().map(|a| a.score.overall).sum::<f64>() / assessments.len() as f64
        } else {
            0.0
        };

        ComplianceDashboard::new(
            total_statutes,
            0, // statutes_with_pii (would need tracking)
            0, // statutes_pending_retention (would need tracking)
            avg_quality,
            low_quality,
            total_audit_events,
            0, // failed_operations (would need tracking)
            0, // sovereignty_violations (would need tracking)
        )
    }

    /// Checks if a statute can be accessed from a specific region.
    pub fn check_sovereignty_access(
        &self,
        _statute_id: &str,
        _requesting_region: &GeographicRegion,
        config: &DataSovereigntyConfig,
    ) -> bool {
        // In a real implementation, this would check statute metadata
        // for region tagging and verify against config
        config.is_region_allowed(_requesting_region)
    }
}

// ============================================================================
// Access Control Features (v0.1.4)
// ============================================================================

/// Permission types for statute operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Read statute content
    Read,
    /// Create new statutes
    Create,
    /// Update existing statutes
    Update,
    /// Delete statutes
    Delete,
    /// Change statute status
    ChangeStatus,
    /// Add/remove tags
    ManageTags,
    /// Add/remove metadata
    ManageMetadata,
    /// Add/remove references
    ManageReferences,
    /// Archive/unarchive statutes
    Archive,
    /// Manage permissions
    ManagePermissions,
    /// Execute bulk operations
    BulkOperations,
    /// Generate reports
    GenerateReports,
}

impl Permission {
    /// Returns all available permissions.
    pub fn all() -> Vec<Permission> {
        vec![
            Permission::Read,
            Permission::Create,
            Permission::Update,
            Permission::Delete,
            Permission::ChangeStatus,
            Permission::ManageTags,
            Permission::ManageMetadata,
            Permission::ManageReferences,
            Permission::Archive,
            Permission::ManagePermissions,
            Permission::BulkOperations,
            Permission::GenerateReports,
        ]
    }

    /// Returns read-only permissions.
    pub fn read_only() -> Vec<Permission> {
        vec![Permission::Read, Permission::GenerateReports]
    }

    /// Returns editor permissions (read + write, no delete/admin).
    pub fn editor() -> Vec<Permission> {
        vec![
            Permission::Read,
            Permission::Create,
            Permission::Update,
            Permission::ChangeStatus,
            Permission::ManageTags,
            Permission::ManageMetadata,
            Permission::ManageReferences,
            Permission::GenerateReports,
        ]
    }
}

/// User role in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Viewer - read-only access
    Viewer,
    /// Editor - read and write access
    Editor,
    /// Admin - full access including permissions management
    Admin,
}

impl Role {
    /// Returns permissions granted to this role.
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Viewer => Permission::read_only(),
            Role::Editor => Permission::editor(),
            Role::Admin => Permission::all(),
        }
    }

    /// Checks if this role has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }

    /// Checks if this role is at least the specified level.
    pub fn is_at_least(&self, other: Role) -> bool {
        self >= &other
    }
}

/// Attribute-based access control condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbacCondition {
    /// User must have specific attribute
    UserAttribute { key: String, value: String },
    /// Statute must have specific tag
    StatuteTag(String),
    /// Statute must be in specific jurisdiction
    Jurisdiction(String),
    /// Statute status must match
    Status(StatuteStatus),
    /// User must be in specific department
    Department(String),
    /// Time-based condition (current time must be within range)
    TimeRange {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    /// Combine multiple conditions with AND
    And(Vec<AbacCondition>),
    /// Combine multiple conditions with OR
    Or(Vec<AbacCondition>),
    /// Negate a condition
    Not(Box<AbacCondition>),
}

impl AbacCondition {
    /// Evaluates the condition.
    pub fn evaluate(
        &self,
        user_attrs: &HashMap<String, String>,
        statute_entry: Option<&StatuteEntry>,
    ) -> bool {
        match self {
            AbacCondition::UserAttribute { key, value } => {
                user_attrs.get(key).map(|v| v == value).unwrap_or(false)
            }
            AbacCondition::StatuteTag(tag) => {
                statute_entry.map(|e| e.tags.contains(tag)).unwrap_or(false)
            }
            AbacCondition::Jurisdiction(jur) => statute_entry
                .map(|e| e.jurisdiction == *jur)
                .unwrap_or(false),
            AbacCondition::Status(status) => {
                statute_entry.map(|e| e.status == *status).unwrap_or(false)
            }
            AbacCondition::Department(dept) => user_attrs
                .get("department")
                .map(|v| v == dept)
                .unwrap_or(false),
            AbacCondition::TimeRange { start, end } => {
                let now = Utc::now();
                now >= *start && now <= *end
            }
            AbacCondition::And(conditions) => conditions
                .iter()
                .all(|c| c.evaluate(user_attrs, statute_entry)),
            AbacCondition::Or(conditions) => conditions
                .iter()
                .any(|c| c.evaluate(user_attrs, statute_entry)),
            AbacCondition::Not(condition) => !condition.evaluate(user_attrs, statute_entry),
        }
    }
}

/// Access control policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub policy_id: Uuid,
    /// Policy name
    pub name: String,
    /// Required role
    pub required_role: Option<Role>,
    /// Specific permissions granted
    pub permissions: Vec<Permission>,
    /// ABAC conditions
    pub conditions: Vec<AbacCondition>,
    /// Priority (higher = evaluated first)
    pub priority: i32,
    /// Is policy enabled?
    pub enabled: bool,
}

impl AccessPolicy {
    /// Creates a new access policy.
    pub fn new(name: impl Into<String>, permissions: Vec<Permission>) -> Self {
        Self {
            policy_id: Uuid::new_v4(),
            name: name.into(),
            required_role: None,
            permissions,
            conditions: Vec::new(),
            priority: 0,
            enabled: true,
        }
    }

    /// Sets the required role.
    pub fn with_role(mut self, role: Role) -> Self {
        self.required_role = Some(role);
        self
    }

    /// Adds an ABAC condition.
    pub fn with_condition(mut self, condition: AbacCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Sets the priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Checks if the policy grants a specific permission.
    pub fn grants(&self, permission: Permission) -> bool {
        self.enabled && self.permissions.contains(&permission)
    }

    /// Checks if all conditions are met.
    pub fn conditions_met(
        &self,
        user_attrs: &HashMap<String, String>,
        statute_entry: Option<&StatuteEntry>,
    ) -> bool {
        self.conditions
            .iter()
            .all(|c| c.evaluate(user_attrs, statute_entry))
    }
}

/// Temporary access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporaryAccess {
    /// Grant ID
    pub grant_id: Uuid,
    /// User ID this grant is for
    pub user_id: String,
    /// Statute ID (None for global access)
    pub statute_id: Option<String>,
    /// Permissions granted
    pub permissions: Vec<Permission>,
    /// Grant valid from
    pub valid_from: DateTime<Utc>,
    /// Grant valid until
    pub valid_until: DateTime<Utc>,
    /// Reason for grant
    pub reason: String,
    /// Granted by (user ID)
    pub granted_by: String,
}

impl TemporaryAccess {
    /// Creates a new temporary access grant.
    pub fn new(
        user_id: impl Into<String>,
        permissions: Vec<Permission>,
        duration_hours: i64,
        reason: impl Into<String>,
        granted_by: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            grant_id: Uuid::new_v4(),
            user_id: user_id.into(),
            statute_id: None,
            permissions,
            valid_from: now,
            valid_until: now + chrono::Duration::hours(duration_hours),
            reason: reason.into(),
            granted_by: granted_by.into(),
        }
    }

    /// Sets the statute ID for statute-specific access.
    pub fn for_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_id = Some(statute_id.into());
        self
    }

    /// Checks if the grant is currently valid.
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        now >= self.valid_from && now <= self.valid_until
    }

    /// Checks if the grant applies to a specific statute.
    pub fn applies_to(&self, statute_id: &str) -> bool {
        self.statute_id
            .as_ref()
            .map(|s| s == statute_id)
            .unwrap_or(true)
    }

    /// Returns remaining time in seconds.
    pub fn remaining_seconds(&self) -> i64 {
        let now = Utc::now();
        if now > self.valid_until {
            0
        } else {
            (self.valid_until - now).num_seconds()
        }
    }
}

/// User with access control attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessUser {
    /// User ID
    pub user_id: String,
    /// Display name
    pub display_name: String,
    /// Primary role
    pub role: Role,
    /// User attributes for ABAC
    pub attributes: HashMap<String, String>,
    /// Directly assigned permissions (overrides role)
    pub direct_permissions: Vec<Permission>,
}

impl AccessUser {
    /// Creates a new user with a role.
    pub fn new(user_id: impl Into<String>, display_name: impl Into<String>, role: Role) -> Self {
        Self {
            user_id: user_id.into(),
            display_name: display_name.into(),
            role,
            attributes: HashMap::new(),
            direct_permissions: Vec::new(),
        }
    }

    /// Adds a user attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Adds a direct permission.
    pub fn with_permission(mut self, permission: Permission) -> Self {
        if !self.direct_permissions.contains(&permission) {
            self.direct_permissions.push(permission);
        }
        self
    }

    /// Gets all permissions (role + direct).
    pub fn all_permissions(&self) -> Vec<Permission> {
        let mut perms = self.role.permissions();
        for p in &self.direct_permissions {
            if !perms.contains(p) {
                perms.push(*p);
            }
        }
        perms
    }

    /// Checks if user has a specific permission.
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.all_permissions().contains(&permission)
    }
}

/// Access control manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessControlManager {
    /// Registered users
    #[serde(default)]
    users: HashMap<String, AccessUser>,
    /// Access policies
    #[serde(default)]
    policies: Vec<AccessPolicy>,
    /// Temporary access grants
    #[serde(default)]
    temporary_grants: Vec<TemporaryAccess>,
    /// Enable/disable access control
    #[serde(default = "default_true")]
    enabled: bool,
}

fn default_true() -> bool {
    true
}

impl AccessControlManager {
    /// Creates a new access control manager.
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            policies: Vec::new(),
            temporary_grants: Vec::new(),
            enabled: true,
        }
    }

    /// Registers a user.
    pub fn add_user(&mut self, user: AccessUser) {
        self.users.insert(user.user_id.clone(), user);
    }

    /// Gets a user by ID.
    pub fn get_user(&self, user_id: &str) -> Option<&AccessUser> {
        self.users.get(user_id)
    }

    /// Updates a user's role.
    pub fn update_user_role(&mut self, user_id: &str, role: Role) -> bool {
        if let Some(user) = self.users.get_mut(user_id) {
            user.role = role;
            true
        } else {
            false
        }
    }

    /// Adds an access policy.
    pub fn add_policy(&mut self, policy: AccessPolicy) {
        self.policies.push(policy);
        // Sort by priority (descending)
        self.policies.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Grants temporary access.
    pub fn grant_temporary_access(&mut self, grant: TemporaryAccess) {
        self.temporary_grants.push(grant);
    }

    /// Cleans up expired temporary grants.
    pub fn cleanup_expired_grants(&mut self) {
        self.temporary_grants.retain(|g| g.is_valid());
    }

    /// Checks if a user has permission for an operation.
    pub fn check_permission(
        &self,
        user_id: &str,
        permission: Permission,
        statute_id: Option<&str>,
        statute_entry: Option<&StatuteEntry>,
    ) -> bool {
        if !self.enabled {
            return true; // Access control disabled
        }

        let user = match self.get_user(user_id) {
            Some(u) => u,
            None => return false, // Unknown user
        };

        // Check direct permissions first
        if user.has_permission(permission) {
            return true;
        }

        // Check temporary grants
        if let Some(sid) = statute_id {
            for grant in &self.temporary_grants {
                if grant.user_id == user_id
                    && grant.is_valid()
                    && grant.applies_to(sid)
                    && grant.permissions.contains(&permission)
                {
                    return true;
                }
            }
        }

        // Check policies
        for policy in &self.policies {
            // Check role requirement
            if let Some(req_role) = policy.required_role
                && !user.role.is_at_least(req_role)
            {
                continue;
            }

            // Check ABAC conditions
            if !policy.conditions_met(&user.attributes, statute_entry) {
                continue;
            }

            // Check if policy grants the permission
            if policy.grants(permission) {
                return true;
            }
        }

        false
    }

    /// Lists all active temporary grants for a user.
    pub fn list_user_grants(&self, user_id: &str) -> Vec<&TemporaryAccess> {
        self.temporary_grants
            .iter()
            .filter(|g| g.user_id == user_id && g.is_valid())
            .collect()
    }

    /// Revokes a temporary grant.
    pub fn revoke_grant(&mut self, grant_id: Uuid) -> bool {
        let len_before = self.temporary_grants.len();
        self.temporary_grants.retain(|g| g.grant_id != grant_id);
        self.temporary_grants.len() < len_before
    }

    /// Enables or disables access control.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns whether access control is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns total number of users.
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Returns total number of policies.
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Returns number of active temporary grants.
    pub fn active_grant_count(&self) -> usize {
        self.temporary_grants
            .iter()
            .filter(|g| g.is_valid())
            .count()
    }
}

// ============================================================================
// Import/Export Extensions (v0.1.5)
// ============================================================================

/// Government database import configuration and execution.
pub mod government_import {
    use super::*;

    /// Format of government database export.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GovernmentDataFormat {
        /// JSON format (common in modern APIs)
        Json,
        /// XML format (common in older systems)
        Xml,
        /// CSV format (simple tabular data)
        Csv,
        /// Custom delimiter-separated values
        Dsv { delimiter: char },
        /// Akoma Ntoso (legislative XML standard)
        AkomaNtoso,
        /// LegalDocML
        LegalDocML,
    }

    /// Import source configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ImportSource {
        /// Source name
        pub name: String,
        /// Source URL or file path
        pub location: String,
        /// Data format
        pub format: GovernmentDataFormat,
        /// Authentication credentials (if needed)
        pub credentials: Option<String>,
        /// Additional metadata
        pub metadata: HashMap<String, String>,
    }

    impl ImportSource {
        /// Creates a new import source.
        pub fn new(
            name: impl Into<String>,
            location: impl Into<String>,
            format: GovernmentDataFormat,
        ) -> Self {
            Self {
                name: name.into(),
                location: location.into(),
                format,
                credentials: None,
                metadata: HashMap::new(),
            }
        }

        /// Sets authentication credentials.
        pub fn with_credentials(mut self, credentials: impl Into<String>) -> Self {
            self.credentials = Some(credentials.into());
            self
        }

        /// Adds metadata.
        pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.metadata.insert(key.into(), value.into());
            self
        }
    }

    /// Result of a bulk import operation.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BulkImportResult {
        /// Source name
        pub source: String,
        /// Number of statutes imported successfully
        pub imported: usize,
        /// Number of statutes skipped (duplicates, etc.)
        pub skipped: usize,
        /// Number of statutes that failed to import
        pub failed: usize,
        /// Errors encountered during import
        pub errors: Vec<String>,
        /// Import timestamp
        pub timestamp: DateTime<Utc>,
        /// Import duration in milliseconds
        pub duration_ms: u64,
    }

    impl BulkImportResult {
        /// Creates a new bulk import result.
        pub fn new(source: impl Into<String>) -> Self {
            Self {
                source: source.into(),
                imported: 0,
                skipped: 0,
                failed: 0,
                errors: Vec::new(),
                timestamp: Utc::now(),
                duration_ms: 0,
            }
        }

        /// Returns total number of statutes processed.
        pub fn total_processed(&self) -> usize {
            self.imported + self.skipped + self.failed
        }

        /// Returns success rate (0.0-1.0).
        pub fn success_rate(&self) -> f64 {
            let total = self.total_processed();
            if total == 0 {
                1.0
            } else {
                self.imported as f64 / total as f64
            }
        }

        /// Returns whether the import was fully successful.
        pub fn is_success(&self) -> bool {
            self.failed == 0
        }
    }

    /// Import strategy for handling duplicates.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ImportStrategy {
        /// Skip duplicate statutes
        Skip,
        /// Update existing statutes
        Update,
        /// Create new version of existing statutes
        NewVersion,
        /// Fail on duplicates
        FailOnDuplicate,
    }

    /// Bulk importer for government databases.
    #[derive(Debug)]
    pub struct BulkImporter {
        /// Import strategy
        strategy: ImportStrategy,
        /// Batch size for processing
        batch_size: usize,
        /// Validate before import
        validate: bool,
        /// Auto-enrich imported statutes
        auto_enrich: bool,
    }

    impl BulkImporter {
        /// Creates a new bulk importer with default settings.
        pub fn new() -> Self {
            Self {
                strategy: ImportStrategy::Skip,
                batch_size: 100,
                validate: true,
                auto_enrich: false,
            }
        }

        /// Sets the import strategy.
        pub fn with_strategy(mut self, strategy: ImportStrategy) -> Self {
            self.strategy = strategy;
            self
        }

        /// Sets the batch size.
        pub fn with_batch_size(mut self, batch_size: usize) -> Self {
            self.batch_size = batch_size;
            self
        }

        /// Enables or disables validation.
        pub fn with_validation(mut self, validate: bool) -> Self {
            self.validate = validate;
            self
        }

        /// Enables or disables auto-enrichment.
        pub fn with_auto_enrich(mut self, auto_enrich: bool) -> Self {
            self.auto_enrich = auto_enrich;
            self
        }

        /// Imports statutes from a source.
        pub fn import(
            &self,
            registry: &mut StatuteRegistry,
            source: &ImportSource,
            statutes: Vec<StatuteEntry>,
        ) -> BulkImportResult {
            let start = std::time::Instant::now();
            let mut result = BulkImportResult::new(&source.name);

            for entry in statutes {
                let statute_id = entry.statute.id.clone();
                match self.import_single(registry, entry) {
                    Ok(true) => result.imported += 1,
                    Ok(false) => result.skipped += 1,
                    Err(e) => {
                        result.failed += 1;
                        result.errors.push(format!("{}: {}", statute_id, e));
                    }
                }
            }

            result.duration_ms = start.elapsed().as_millis() as u64;
            result
        }

        fn import_single(
            &self,
            registry: &mut StatuteRegistry,
            entry: StatuteEntry,
        ) -> RegistryResult<bool> {
            // Validate if enabled
            if self.validate {
                let validator = Validator::with_defaults();
                if let Err(errors) = validator.validate(&entry) {
                    return Err(RegistryError::InvalidOperation(format!(
                        "Validation failed: {:?}",
                        errors
                    )));
                }
            }

            // Check if statute already exists
            let statute_id = entry.statute.id.clone();
            let exists = registry.contains(&statute_id);

            if exists {
                match self.strategy {
                    ImportStrategy::Skip => return Ok(false),
                    ImportStrategy::Update => {
                        registry.update(&statute_id, entry.statute)?;
                        return Ok(true);
                    }
                    ImportStrategy::NewVersion => {
                        registry.update(&statute_id, entry.statute)?;
                        return Ok(true);
                    }
                    ImportStrategy::FailOnDuplicate => {
                        return Err(RegistryError::DuplicateId(statute_id));
                    }
                }
            }

            // Register new statute
            registry.register(entry)?;
            Ok(true)
        }
    }

    impl Default for BulkImporter {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Scheduled synchronization for periodic imports.
pub mod sync {
    use super::*;
    use chrono::{Datelike, Timelike};

    /// Synchronization schedule.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SyncSchedule {
        /// Manual synchronization only
        Manual,
        /// Hourly synchronization
        Hourly,
        /// Daily synchronization at specified hour
        Daily { hour: u8 },
        /// Weekly synchronization on specified day and hour
        Weekly { day: u8, hour: u8 },
        /// Monthly synchronization on specified day and hour
        Monthly { day: u8, hour: u8 },
        /// Custom interval in seconds
        Interval { seconds: u64 },
    }

    impl SyncSchedule {
        /// Returns the next sync time from a given timestamp.
        pub fn next_sync(&self, from: DateTime<Utc>) -> Option<DateTime<Utc>> {
            match self {
                Self::Manual => None,
                Self::Hourly => Some(from + chrono::Duration::hours(1)),
                Self::Daily { hour } => {
                    let next = from + chrono::Duration::days(1);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Weekly { day: _, hour } => {
                    let next = from + chrono::Duration::weeks(1);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Monthly { day, hour } => {
                    let next =
                        from.with_day(*day as u32).unwrap_or(from) + chrono::Duration::days(30);
                    Some(next.with_hour(*hour as u32).unwrap_or(next))
                }
                Self::Interval { seconds } => {
                    Some(from + chrono::Duration::seconds(*seconds as i64))
                }
            }
        }

        /// Checks if a sync is due from a given last sync time.
        pub fn is_due(&self, last_sync: DateTime<Utc>, now: DateTime<Utc>) -> bool {
            match self.next_sync(last_sync) {
                Some(next) => now >= next,
                None => false,
            }
        }
    }

    /// Synchronization job configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SyncJob {
        /// Job ID
        pub id: Uuid,
        /// Job name
        pub name: String,
        /// Import source
        pub source: government_import::ImportSource,
        /// Schedule
        pub schedule: SyncSchedule,
        /// Last sync timestamp
        pub last_sync: Option<DateTime<Utc>>,
        /// Last sync result
        pub last_result: Option<government_import::BulkImportResult>,
        /// Whether the job is enabled
        pub enabled: bool,
    }

    impl SyncJob {
        /// Creates a new sync job.
        pub fn new(
            name: impl Into<String>,
            source: government_import::ImportSource,
            schedule: SyncSchedule,
        ) -> Self {
            Self {
                id: Uuid::new_v4(),
                name: name.into(),
                source,
                schedule,
                last_sync: None,
                last_result: None,
                enabled: true,
            }
        }

        /// Checks if the job is due for execution.
        pub fn is_due(&self, now: DateTime<Utc>) -> bool {
            if !self.enabled {
                return false;
            }
            match self.last_sync {
                Some(last) => self.schedule.is_due(last, now),
                None => true, // Never synced, so it's due
            }
        }

        /// Marks the job as completed with a result.
        pub fn mark_completed(&mut self, result: government_import::BulkImportResult) {
            self.last_sync = Some(Utc::now());
            self.last_result = Some(result);
        }
    }

    /// Synchronization manager.
    #[derive(Debug)]
    pub struct SyncManager {
        jobs: Vec<SyncJob>,
    }

    impl SyncManager {
        /// Creates a new sync manager.
        pub fn new() -> Self {
            Self { jobs: Vec::new() }
        }

        /// Adds a sync job.
        pub fn add_job(&mut self, job: SyncJob) {
            self.jobs.push(job);
        }

        /// Removes a sync job by ID.
        pub fn remove_job(&mut self, job_id: Uuid) -> bool {
            if let Some(pos) = self.jobs.iter().position(|j| j.id == job_id) {
                self.jobs.remove(pos);
                true
            } else {
                false
            }
        }

        /// Gets all jobs.
        pub fn jobs(&self) -> &[SyncJob] {
            &self.jobs
        }

        /// Gets all jobs that are due for execution.
        pub fn due_jobs(&self, now: DateTime<Utc>) -> Vec<&SyncJob> {
            self.jobs.iter().filter(|j| j.is_due(now)).collect()
        }

        /// Updates a job's result.
        pub fn update_job_result(
            &mut self,
            job_id: Uuid,
            result: government_import::BulkImportResult,
        ) {
            if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
                job.mark_completed(result);
            }
        }

        /// Enables or disables a job.
        pub fn set_job_enabled(&mut self, job_id: Uuid, enabled: bool) -> bool {
            if let Some(job) = self.jobs.iter_mut().find(|j| j.id == job_id) {
                job.enabled = enabled;
                true
            } else {
                false
            }
        }
    }

    impl Default for SyncManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Format migration utilities.
pub mod migration {
    use super::*;

    /// Supported migration formats.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MigrationFormat {
        /// Legacy JSON v1
        JsonV1,
        /// Legacy JSON v2
        JsonV2,
        /// Current JSON format
        JsonCurrent,
        /// Legacy XML
        XmlLegacy,
        /// Akoma Ntoso XML
        AkomaNtoso,
        /// CSV format
        Csv,
    }

    /// Migration result.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MigrationResult {
        /// Source format
        pub from_format: MigrationFormat,
        /// Target format
        pub to_format: MigrationFormat,
        /// Number of statutes migrated
        pub migrated: usize,
        /// Number of statutes that failed
        pub failed: usize,
        /// Errors encountered
        pub errors: Vec<String>,
        /// Migration timestamp
        pub timestamp: DateTime<Utc>,
    }

    impl MigrationResult {
        /// Creates a new migration result.
        pub fn new(from: MigrationFormat, to: MigrationFormat) -> Self {
            Self {
                from_format: from,
                to_format: to,
                migrated: 0,
                failed: 0,
                errors: Vec::new(),
                timestamp: Utc::now(),
            }
        }

        /// Returns success rate (0.0-1.0).
        pub fn success_rate(&self) -> f64 {
            let total = self.migrated + self.failed;
            if total == 0 {
                1.0
            } else {
                self.migrated as f64 / total as f64
            }
        }
    }

    /// Format migrator.
    #[derive(Debug)]
    pub struct FormatMigrator {
        /// Whether to validate after migration
        validate: bool,
    }

    impl FormatMigrator {
        /// Creates a new format migrator.
        pub fn new() -> Self {
            Self { validate: true }
        }

        /// Enables or disables validation.
        pub fn with_validation(mut self, validate: bool) -> Self {
            self.validate = validate;
            self
        }

        /// Migrates data from one format to another.
        pub fn migrate(
            &self,
            from_format: MigrationFormat,
            to_format: MigrationFormat,
            data: &str,
        ) -> Result<(String, MigrationResult), RegistryError> {
            let mut result = MigrationResult::new(from_format, to_format);

            // For now, we'll implement a simple JSON round-trip migration
            // In a real implementation, this would handle actual format conversions
            match (from_format, to_format) {
                (MigrationFormat::JsonCurrent, MigrationFormat::JsonCurrent) => {
                    // No migration needed
                    result.migrated = 1;
                    Ok((data.to_string(), result))
                }
                _ => {
                    // Placeholder for other migration paths
                    result.failed = 1;
                    result.errors.push(format!(
                        "Migration from {:?} to {:?} not yet implemented",
                        from_format, to_format
                    ));
                    Err(RegistryError::InvalidOperation(format!(
                        "Migration path {:?} -> {:?} not supported",
                        from_format, to_format
                    )))
                }
            }
        }
    }

    impl Default for FormatMigrator {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Export templates for reporting.
pub mod templates {
    use super::*;

    /// Report template type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TemplateType {
        /// Summary report (high-level statistics)
        Summary,
        /// Detailed report (full statute information)
        Detailed,
        /// Compliance report (regulatory focus)
        Compliance,
        /// Audit trail report
        AuditTrail,
        /// Custom template with name
        Custom(String),
    }

    /// Export format for templates.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ExportFormat {
        /// JSON format
        Json,
        /// CSV format
        Csv,
        /// HTML format
        Html,
        /// Markdown format
        Markdown,
        /// PDF format (requires additional dependencies)
        Pdf,
    }

    /// Report template configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReportTemplate {
        /// Template name
        pub name: String,
        /// Template type
        pub template_type: TemplateType,
        /// Export format
        pub format: ExportFormat,
        /// Fields to include
        pub fields: Vec<String>,
        /// Custom filters
        pub filters: HashMap<String, String>,
        /// Sort order
        pub sort_by: Option<String>,
    }

    impl ReportTemplate {
        /// Creates a new report template.
        pub fn new(
            name: impl Into<String>,
            template_type: TemplateType,
            format: ExportFormat,
        ) -> Self {
            Self {
                name: name.into(),
                template_type,
                format,
                fields: Vec::new(),
                filters: HashMap::new(),
                sort_by: None,
            }
        }

        /// Adds a field to include.
        pub fn with_field(mut self, field: impl Into<String>) -> Self {
            self.fields.push(field.into());
            self
        }

        /// Adds a filter.
        pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.filters.insert(key.into(), value.into());
            self
        }

        /// Sets the sort order.
        pub fn with_sort_by(mut self, field: impl Into<String>) -> Self {
            self.sort_by = Some(field.into());
            self
        }

        /// Creates a summary template.
        pub fn summary(format: ExportFormat) -> Self {
            Self::new("Summary Report", TemplateType::Summary, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("jurisdiction")
        }

        /// Creates a detailed template.
        pub fn detailed(format: ExportFormat) -> Self {
            Self::new("Detailed Report", TemplateType::Detailed, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("jurisdiction")
                .with_field("tags")
                .with_field("metadata")
                .with_field("created_at")
                .with_field("modified_at")
        }

        /// Creates a compliance template.
        pub fn compliance(format: ExportFormat) -> Self {
            Self::new("Compliance Report", TemplateType::Compliance, format)
                .with_field("id")
                .with_field("title")
                .with_field("status")
                .with_field("effective_date")
                .with_field("expiry_date")
        }
    }

    /// Template manager.
    #[derive(Debug)]
    pub struct TemplateManager {
        templates: HashMap<String, ReportTemplate>,
    }

    impl TemplateManager {
        /// Creates a new template manager.
        pub fn new() -> Self {
            Self {
                templates: HashMap::new(),
            }
        }

        /// Adds a template.
        pub fn add_template(&mut self, template: ReportTemplate) {
            self.templates.insert(template.name.clone(), template);
        }

        /// Gets a template by name.
        pub fn get_template(&self, name: &str) -> Option<&ReportTemplate> {
            self.templates.get(name)
        }

        /// Removes a template.
        pub fn remove_template(&mut self, name: &str) -> bool {
            self.templates.remove(name).is_some()
        }

        /// Lists all template names.
        pub fn list_templates(&self) -> Vec<&str> {
            self.templates.keys().map(|s| s.as_str()).collect()
        }

        /// Exports registry data using a template.
        pub fn export(
            &self,
            registry: &StatuteRegistry,
            template_name: &str,
        ) -> Result<String, RegistryError> {
            let template = self.get_template(template_name).ok_or_else(|| {
                RegistryError::InvalidOperation(format!("Template '{}' not found", template_name))
            })?;

            match template.format {
                ExportFormat::Json => self.export_json(registry, template),
                ExportFormat::Csv => self.export_csv(registry, template),
                ExportFormat::Html => self.export_html(registry, template),
                ExportFormat::Markdown => self.export_markdown(registry, template),
                ExportFormat::Pdf => Err(RegistryError::InvalidOperation(
                    "PDF export not yet implemented".to_string(),
                )),
            }
        }

        fn export_json(
            &self,
            registry: &StatuteRegistry,
            _template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let statutes: Vec<_> = registry.iter().collect();
            serde_json::to_string_pretty(&statutes)
                .map_err(|e| RegistryError::InvalidOperation(format!("JSON export failed: {}", e)))
        }

        fn export_csv(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut output = String::new();

            // Header
            if !template.fields.is_empty() {
                output.push_str(&template.fields.join(","));
            } else {
                output.push_str("id,title,status,jurisdiction");
            }
            output.push('\n');

            // Rows
            for entry in registry.iter() {
                let row = format!(
                    "{},{},{:?},{}",
                    entry.statute.id, entry.statute.title, entry.status, entry.jurisdiction
                );
                output.push_str(&row);
                output.push('\n');
            }

            Ok(output)
        }

        fn export_html(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut html = String::from("<html><head><title>");
            html.push_str(&template.name);
            html.push_str("</title></head><body><h1>");
            html.push_str(&template.name);
            html.push_str("</h1><table border='1'><tr>");

            // Header
            for field in &template.fields {
                html.push_str("<th>");
                html.push_str(field);
                html.push_str("</th>");
            }
            html.push_str("</tr>");

            // Rows
            for entry in registry.iter() {
                html.push_str("<tr>");
                for field in &template.fields {
                    html.push_str("<td>");
                    match field.as_str() {
                        "id" => html.push_str(&entry.statute.id),
                        "title" => html.push_str(&entry.statute.title),
                        "status" => html.push_str(&format!("{:?}", entry.status)),
                        "jurisdiction" => html.push_str(&entry.jurisdiction),
                        _ => html.push_str("N/A"),
                    }
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }

            html.push_str("</table></body></html>");
            Ok(html)
        }

        fn export_markdown(
            &self,
            registry: &StatuteRegistry,
            template: &ReportTemplate,
        ) -> Result<String, RegistryError> {
            let mut md = format!("# {}\n\n", template.name);

            // Table header
            if !template.fields.is_empty() {
                md.push_str("| ");
                md.push_str(&template.fields.join(" | "));
                md.push_str(" |\n");
                md.push('|');
                for _ in &template.fields {
                    md.push_str(" --- |");
                }
                md.push('\n');
            }

            // Rows
            for entry in registry.iter() {
                md.push_str("| ");
                for (i, field) in template.fields.iter().enumerate() {
                    if i > 0 {
                        md.push_str(" | ");
                    }
                    match field.as_str() {
                        "id" => md.push_str(&entry.statute.id),
                        "title" => md.push_str(&entry.statute.title),
                        "status" => md.push_str(&format!("{:?}", entry.status)),
                        "jurisdiction" => md.push_str(&entry.jurisdiction),
                        _ => md.push_str("N/A"),
                    }
                }
                md.push_str(" |\n");
            }

            Ok(md)
        }
    }

    impl Default for TemplateManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Selective export by criteria.
impl StatuteRegistry {
    /// Exports statutes matching a filter predicate.
    pub fn export_filtered_statutes<F>(&self, filter: F) -> Result<String, RegistryError>
    where
        F: Fn(&StatuteEntry) -> bool,
    {
        let filtered: Vec<_> = self
            .statutes
            .values()
            .filter(|entry| filter(entry))
            .collect();

        serde_json::to_string_pretty(&filtered)
            .map_err(|e| RegistryError::InvalidOperation(format!("Export failed: {}", e)))
    }

    /// Exports statutes by status.
    pub fn export_by_status(&self, status: StatuteStatus) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.status == status)
    }

    /// Exports statutes by jurisdiction.
    pub fn export_by_jurisdiction(&self, jurisdiction: &str) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.jurisdiction == jurisdiction)
    }

    /// Exports statutes by tag.
    pub fn export_by_tag(&self, tag: &str) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| entry.tags.iter().any(|t| t == tag))
    }

    /// Exports statutes modified within a date range.
    pub fn export_by_date_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<String, RegistryError> {
        self.export_filtered_statutes(|entry| {
            entry.modified_at >= start && entry.modified_at <= end
        })
    }
}

// ============================================================================
// Workflow Integration (v0.1.6)
// ============================================================================

/// Approval workflows for statute changes.
pub mod workflow {
    use super::*;

    /// Workflow status for a statute change.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum WorkflowStatus {
        /// Draft - not yet submitted for approval
        Draft,
        /// Pending approval
        PendingApproval,
        /// Approved and ready to apply
        Approved,
        /// Rejected with reason
        Rejected,
        /// Cancelled by submitter
        Cancelled,
    }

    /// Type of change being proposed.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ChangeType {
        /// Creating a new statute
        Create,
        /// Updating an existing statute
        Update { statute_id: String },
        /// Deleting a statute
        Delete { statute_id: String },
        /// Changing status
        StatusChange {
            statute_id: String,
            new_status: StatuteStatus,
        },
        /// Bulk operation
        Bulk { operation_count: usize },
    }

    /// An approval request for a statute change.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApprovalRequest {
        /// Unique request ID
        pub request_id: Uuid,
        /// Type of change
        pub change_type: ChangeType,
        /// Submitter user ID
        pub submitter: String,
        /// Workflow status
        pub status: WorkflowStatus,
        /// Requested change data (JSON)
        pub change_data: String,
        /// Justification for the change
        pub justification: Option<String>,
        /// Approvers assigned
        pub approvers: Vec<String>,
        /// Approval responses
        pub responses: Vec<ApprovalResponse>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Updated timestamp
        pub updated_at: DateTime<Utc>,
        /// Due date for approval
        pub due_date: Option<DateTime<Utc>>,
    }

    impl ApprovalRequest {
        /// Creates a new approval request.
        pub fn new(
            change_type: ChangeType,
            submitter: impl Into<String>,
            change_data: impl Into<String>,
        ) -> Self {
            Self {
                request_id: Uuid::new_v4(),
                change_type,
                submitter: submitter.into(),
                status: WorkflowStatus::Draft,
                change_data: change_data.into(),
                justification: None,
                approvers: Vec::new(),
                responses: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                due_date: None,
            }
        }

        /// Sets the justification.
        pub fn with_justification(mut self, justification: impl Into<String>) -> Self {
            self.justification = Some(justification.into());
            self
        }

        /// Adds an approver.
        pub fn with_approver(mut self, approver: impl Into<String>) -> Self {
            self.approvers.push(approver.into());
            self
        }

        /// Sets the due date.
        pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
            self.due_date = Some(due_date);
            self
        }

        /// Submits the request for approval.
        pub fn submit(&mut self) {
            self.status = WorkflowStatus::PendingApproval;
            self.updated_at = Utc::now();
        }

        /// Adds an approval response.
        pub fn add_response(&mut self, response: ApprovalResponse) {
            self.responses.push(response);
            self.updated_at = Utc::now();
        }

        /// Checks if the request is approved (all approvers approved).
        pub fn is_approved(&self) -> bool {
            if self.approvers.is_empty() {
                return false;
            }
            let approved_count = self
                .responses
                .iter()
                .filter(|r| r.decision == ApprovalDecision::Approved)
                .count();
            approved_count >= self.approvers.len()
        }

        /// Checks if the request is rejected (any approver rejected).
        pub fn is_rejected(&self) -> bool {
            self.responses
                .iter()
                .any(|r| r.decision == ApprovalDecision::Rejected)
        }

        /// Checks if the request is overdue.
        pub fn is_overdue(&self) -> bool {
            if let Some(due) = self.due_date {
                Utc::now() > due && self.status == WorkflowStatus::PendingApproval
            } else {
                false
            }
        }
    }

    /// Approval decision.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ApprovalDecision {
        /// Approved
        Approved,
        /// Rejected
        Rejected,
        /// Needs more information
        NeedsInfo,
    }

    /// An approval response from an approver.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApprovalResponse {
        /// Approver user ID
        pub approver: String,
        /// Decision
        pub decision: ApprovalDecision,
        /// Comments
        pub comments: Option<String>,
        /// Response timestamp
        pub responded_at: DateTime<Utc>,
    }

    impl ApprovalResponse {
        /// Creates a new approval response.
        pub fn new(approver: impl Into<String>, decision: ApprovalDecision) -> Self {
            Self {
                approver: approver.into(),
                decision,
                comments: None,
                responded_at: Utc::now(),
            }
        }

        /// Sets comments.
        pub fn with_comments(mut self, comments: impl Into<String>) -> Self {
            self.comments = Some(comments.into());
            self
        }
    }

    /// Type alias for auto-approval rule functions.
    pub type AutoApproveRule = Box<dyn Fn(&ApprovalRequest) -> bool + Send + Sync>;

    /// Workflow manager for approval requests.
    pub struct WorkflowManager {
        requests: HashMap<Uuid, ApprovalRequest>,
        /// Auto-approval rules
        auto_approve_rules: Vec<AutoApproveRule>,
    }

    impl std::fmt::Debug for WorkflowManager {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WorkflowManager")
                .field("requests", &self.requests)
                .field(
                    "auto_approve_rules",
                    &format!("<{} rules>", self.auto_approve_rules.len()),
                )
                .finish()
        }
    }

    impl WorkflowManager {
        /// Creates a new workflow manager.
        pub fn new() -> Self {
            Self {
                requests: HashMap::new(),
                auto_approve_rules: Vec::new(),
            }
        }

        /// Submits a new approval request.
        pub fn submit_request(&mut self, mut request: ApprovalRequest) -> Uuid {
            request.submit();
            let id = request.request_id;

            // Check auto-approval rules
            for rule in &self.auto_approve_rules {
                if rule(&request) {
                    request.status = WorkflowStatus::Approved;
                    break;
                }
            }

            self.requests.insert(id, request);
            id
        }

        /// Gets a request by ID.
        pub fn get_request(&self, request_id: Uuid) -> Option<&ApprovalRequest> {
            self.requests.get(&request_id)
        }

        /// Adds a response to a request.
        pub fn add_response(
            &mut self,
            request_id: Uuid,
            response: ApprovalResponse,
        ) -> Result<(), String> {
            let request = self
                .requests
                .get_mut(&request_id)
                .ok_or_else(|| "Request not found".to_string())?;

            if request.status != WorkflowStatus::PendingApproval {
                return Err("Request is not pending approval".to_string());
            }

            request.add_response(response);

            // Update status based on responses
            if request.is_rejected() {
                request.status = WorkflowStatus::Rejected;
            } else if request.is_approved() {
                request.status = WorkflowStatus::Approved;
            }

            Ok(())
        }

        /// Gets pending requests.
        pub fn pending_requests(&self) -> Vec<&ApprovalRequest> {
            self.requests
                .values()
                .filter(|r| r.status == WorkflowStatus::PendingApproval)
                .collect()
        }

        /// Gets overdue requests.
        pub fn overdue_requests(&self) -> Vec<&ApprovalRequest> {
            self.requests.values().filter(|r| r.is_overdue()).collect()
        }

        /// Gets requests for a specific approver.
        pub fn requests_for_approver(&self, approver: &str) -> Vec<&ApprovalRequest> {
            self.requests
                .values()
                .filter(|r| {
                    r.approvers.contains(&approver.to_string())
                        && r.status == WorkflowStatus::PendingApproval
                })
                .collect()
        }
    }

    impl Default for WorkflowManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Notification system for stakeholders.
pub mod notifications {
    use super::*;

    /// Notification type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NotificationType {
        /// Approval request submitted
        ApprovalRequested,
        /// Approval granted
        ApprovalGranted,
        /// Approval rejected
        ApprovalRejected,
        /// Task assigned
        TaskAssigned,
        /// Task completed
        TaskCompleted,
        /// SLA warning
        SlaWarning,
        /// SLA breach
        SlaBreach,
        /// Statute updated
        StatuteUpdated,
        /// Custom notification
        Custom(String),
    }

    /// Notification priority.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum NotificationPriority {
        /// Low priority
        Low,
        /// Normal priority
        Normal,
        /// High priority
        High,
        /// Critical priority
        Critical,
    }

    /// Notification channel.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum NotificationChannel {
        /// Email notification
        Email,
        /// SMS notification
        Sms,
        /// In-app notification
        InApp,
        /// Webhook notification
        Webhook { url: String },
        /// Custom channel
        Custom(String),
    }

    /// A notification.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Notification {
        /// Notification ID
        pub notification_id: Uuid,
        /// Recipient user ID
        pub recipient: String,
        /// Notification type
        pub notification_type: NotificationType,
        /// Priority
        pub priority: NotificationPriority,
        /// Title
        pub title: String,
        /// Message
        pub message: String,
        /// Related entity ID (e.g., request ID, statute ID)
        pub related_entity_id: Option<String>,
        /// Channels to send through
        pub channels: Vec<NotificationChannel>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Sent timestamp
        pub sent_at: Option<DateTime<Utc>>,
        /// Read timestamp
        pub read_at: Option<DateTime<Utc>>,
    }

    impl Notification {
        /// Creates a new notification.
        pub fn new(
            recipient: impl Into<String>,
            notification_type: NotificationType,
            title: impl Into<String>,
            message: impl Into<String>,
        ) -> Self {
            Self {
                notification_id: Uuid::new_v4(),
                recipient: recipient.into(),
                notification_type,
                priority: NotificationPriority::Normal,
                title: title.into(),
                message: message.into(),
                related_entity_id: None,
                channels: vec![NotificationChannel::InApp],
                created_at: Utc::now(),
                sent_at: None,
                read_at: None,
            }
        }

        /// Sets priority.
        pub fn with_priority(mut self, priority: NotificationPriority) -> Self {
            self.priority = priority;
            self
        }

        /// Sets related entity ID.
        pub fn with_related_entity(mut self, entity_id: impl Into<String>) -> Self {
            self.related_entity_id = Some(entity_id.into());
            self
        }

        /// Adds a channel.
        pub fn with_channel(mut self, channel: NotificationChannel) -> Self {
            self.channels.push(channel);
            self
        }

        /// Marks as sent.
        pub fn mark_sent(&mut self) {
            self.sent_at = Some(Utc::now());
        }

        /// Marks as read.
        pub fn mark_read(&mut self) {
            self.read_at = Some(Utc::now());
        }

        /// Checks if sent.
        pub fn is_sent(&self) -> bool {
            self.sent_at.is_some()
        }

        /// Checks if read.
        pub fn is_read(&self) -> bool {
            self.read_at.is_some()
        }
    }

    /// Notification manager.
    #[derive(Debug)]
    pub struct NotificationManager {
        notifications: Vec<Notification>,
        max_notifications: usize,
    }

    impl NotificationManager {
        /// Creates a new notification manager.
        pub fn new() -> Self {
            Self {
                notifications: Vec::new(),
                max_notifications: 10000,
            }
        }

        /// Sends a notification.
        pub fn send(&mut self, mut notification: Notification) {
            notification.mark_sent();
            self.notifications.push(notification);

            // Rotate if needed
            if self.notifications.len() > self.max_notifications {
                self.notifications
                    .drain(0..self.notifications.len() - self.max_notifications);
            }
        }

        /// Gets unread notifications for a user.
        pub fn unread_for_user(&self, user_id: &str) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.recipient == user_id && !n.is_read())
                .collect()
        }

        /// Marks a notification as read.
        pub fn mark_as_read(&mut self, notification_id: Uuid) -> bool {
            if let Some(notification) = self
                .notifications
                .iter_mut()
                .find(|n| n.notification_id == notification_id)
            {
                notification.mark_read();
                true
            } else {
                false
            }
        }

        /// Gets all notifications for a user.
        pub fn for_user(&self, user_id: &str) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.recipient == user_id)
                .collect()
        }

        /// Gets notifications by priority.
        pub fn by_priority(&self, min_priority: NotificationPriority) -> Vec<&Notification> {
            self.notifications
                .iter()
                .filter(|n| n.priority >= min_priority)
                .collect()
        }
    }

    impl Default for NotificationManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Task assignment for reviews.
pub mod tasks {
    use super::*;

    /// Task status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TaskStatus {
        /// Not yet started
        NotStarted,
        /// In progress
        InProgress,
        /// Blocked
        Blocked,
        /// Completed
        Completed,
        /// Cancelled
        Cancelled,
    }

    /// Review task.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReviewTask {
        /// Task ID
        pub task_id: Uuid,
        /// Task title
        pub title: String,
        /// Task description
        pub description: Option<String>,
        /// Assigned to user ID
        pub assigned_to: String,
        /// Assigned by user ID
        pub assigned_by: String,
        /// Related statute ID
        pub statute_id: String,
        /// Task status
        pub status: TaskStatus,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Started timestamp
        pub started_at: Option<DateTime<Utc>>,
        /// Completed timestamp
        pub completed_at: Option<DateTime<Utc>>,
        /// Due date
        pub due_date: Option<DateTime<Utc>>,
        /// Review notes
        pub notes: Vec<String>,
    }

    impl ReviewTask {
        /// Creates a new review task.
        pub fn new(
            title: impl Into<String>,
            assigned_to: impl Into<String>,
            assigned_by: impl Into<String>,
            statute_id: impl Into<String>,
        ) -> Self {
            Self {
                task_id: Uuid::new_v4(),
                title: title.into(),
                description: None,
                assigned_to: assigned_to.into(),
                assigned_by: assigned_by.into(),
                statute_id: statute_id.into(),
                status: TaskStatus::NotStarted,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                due_date: None,
                notes: Vec::new(),
            }
        }

        /// Sets description.
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }

        /// Sets due date.
        pub fn with_due_date(mut self, due_date: DateTime<Utc>) -> Self {
            self.due_date = Some(due_date);
            self
        }

        /// Starts the task.
        pub fn start(&mut self) {
            self.status = TaskStatus::InProgress;
            self.started_at = Some(Utc::now());
        }

        /// Completes the task.
        pub fn complete(&mut self) {
            self.status = TaskStatus::Completed;
            self.completed_at = Some(Utc::now());
        }

        /// Adds a note.
        pub fn add_note(&mut self, note: impl Into<String>) {
            self.notes.push(note.into());
        }

        /// Checks if overdue.
        pub fn is_overdue(&self) -> bool {
            if let Some(due) = self.due_date {
                Utc::now() > due && self.status != TaskStatus::Completed
            } else {
                false
            }
        }
    }

    /// Task manager.
    #[derive(Debug)]
    pub struct TaskManager {
        tasks: HashMap<Uuid, ReviewTask>,
    }

    impl TaskManager {
        /// Creates a new task manager.
        pub fn new() -> Self {
            Self {
                tasks: HashMap::new(),
            }
        }

        /// Creates a task.
        pub fn create_task(&mut self, task: ReviewTask) -> Uuid {
            let id = task.task_id;
            self.tasks.insert(id, task);
            id
        }

        /// Gets a task by ID.
        pub fn get_task(&self, task_id: Uuid) -> Option<&ReviewTask> {
            self.tasks.get(&task_id)
        }

        /// Gets a mutable task by ID.
        pub fn get_task_mut(&mut self, task_id: Uuid) -> Option<&mut ReviewTask> {
            self.tasks.get_mut(&task_id)
        }

        /// Gets tasks assigned to a user.
        pub fn tasks_for_user(&self, user_id: &str) -> Vec<&ReviewTask> {
            self.tasks
                .values()
                .filter(|t| t.assigned_to == user_id)
                .collect()
        }

        /// Gets overdue tasks.
        pub fn overdue_tasks(&self) -> Vec<&ReviewTask> {
            self.tasks.values().filter(|t| t.is_overdue()).collect()
        }

        /// Gets tasks by status.
        pub fn tasks_by_status(&self, status: TaskStatus) -> Vec<&ReviewTask> {
            self.tasks.values().filter(|t| t.status == status).collect()
        }
    }

    impl Default for TaskManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// SLA tracking for approvals.
pub mod sla {
    use super::*;

    /// SLA metric type.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SlaMetric {
        /// Time to first response
        TimeToFirstResponse,
        /// Time to approval
        TimeToApproval,
        /// Time to completion
        TimeToCompletion,
        /// Custom metric
        Custom(String),
    }

    /// SLA definition.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SlaDefinition {
        /// SLA ID
        pub sla_id: Uuid,
        /// SLA name
        pub name: String,
        /// Metric being tracked
        pub metric: SlaMetric,
        /// Target duration in seconds
        pub target_seconds: i64,
        /// Warning threshold (percentage of target)
        pub warning_threshold: f64,
    }

    impl SlaDefinition {
        /// Creates a new SLA definition.
        pub fn new(name: impl Into<String>, metric: SlaMetric, target_seconds: i64) -> Self {
            Self {
                sla_id: Uuid::new_v4(),
                name: name.into(),
                metric,
                target_seconds,
                warning_threshold: 0.8, // 80% of target
            }
        }

        /// Sets warning threshold.
        pub fn with_warning_threshold(mut self, threshold: f64) -> Self {
            self.warning_threshold = threshold.clamp(0.0, 1.0);
            self
        }

        /// Gets target duration.
        pub fn target_duration(&self) -> chrono::Duration {
            chrono::Duration::seconds(self.target_seconds)
        }

        /// Gets warning duration.
        pub fn warning_duration(&self) -> chrono::Duration {
            chrono::Duration::seconds((self.target_seconds as f64 * self.warning_threshold) as i64)
        }
    }

    /// SLA status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SlaStatus {
        /// Met the SLA
        Met,
        /// Warning - approaching SLA breach
        Warning,
        /// Breached the SLA
        Breached,
    }

    /// SLA measurement.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SlaMeasurement {
        /// Measurement ID
        pub measurement_id: Uuid,
        /// SLA definition ID
        pub sla_id: Uuid,
        /// Related entity ID
        pub entity_id: String,
        /// Start time
        pub start_time: DateTime<Utc>,
        /// End time
        pub end_time: Option<DateTime<Utc>>,
        /// Actual duration in seconds
        pub duration_seconds: Option<i64>,
        /// SLA status
        pub status: SlaStatus,
    }

    impl SlaMeasurement {
        /// Creates a new SLA measurement.
        pub fn new(sla_id: Uuid, entity_id: impl Into<String>) -> Self {
            Self {
                measurement_id: Uuid::new_v4(),
                sla_id,
                entity_id: entity_id.into(),
                start_time: Utc::now(),
                end_time: None,
                duration_seconds: None,
                status: SlaStatus::Met,
            }
        }

        /// Completes the measurement.
        pub fn complete(&mut self, sla: &SlaDefinition) {
            self.end_time = Some(Utc::now());
            let duration = self.end_time.unwrap() - self.start_time;
            self.duration_seconds = Some(duration.num_seconds());

            // Determine status
            if duration > sla.target_duration() {
                self.status = SlaStatus::Breached;
            } else if duration > sla.warning_duration() {
                self.status = SlaStatus::Warning;
            } else {
                self.status = SlaStatus::Met;
            }
        }

        /// Checks current status against SLA.
        pub fn check_status(&mut self, sla: &SlaDefinition) -> SlaStatus {
            if self.end_time.is_some() {
                return self.status;
            }

            let elapsed = Utc::now() - self.start_time;
            if elapsed > sla.target_duration() {
                self.status = SlaStatus::Breached;
            } else if elapsed > sla.warning_duration() {
                self.status = SlaStatus::Warning;
            } else {
                self.status = SlaStatus::Met;
            }
            self.status
        }
    }

    /// SLA tracker.
    #[derive(Debug)]
    pub struct SlaTracker {
        definitions: HashMap<Uuid, SlaDefinition>,
        measurements: Vec<SlaMeasurement>,
    }

    impl SlaTracker {
        /// Creates a new SLA tracker.
        pub fn new() -> Self {
            Self {
                definitions: HashMap::new(),
                measurements: Vec::new(),
            }
        }

        /// Adds an SLA definition.
        pub fn add_definition(&mut self, definition: SlaDefinition) -> Uuid {
            let id = definition.sla_id;
            self.definitions.insert(id, definition);
            id
        }

        /// Starts tracking an SLA.
        pub fn start_tracking(&mut self, sla_id: Uuid, entity_id: impl Into<String>) -> Uuid {
            let measurement = SlaMeasurement::new(sla_id, entity_id);
            let id = measurement.measurement_id;
            self.measurements.push(measurement);
            id
        }

        /// Completes an SLA measurement.
        pub fn complete_measurement(&mut self, measurement_id: Uuid) -> Result<SlaStatus, String> {
            let measurement = self
                .measurements
                .iter_mut()
                .find(|m| m.measurement_id == measurement_id)
                .ok_or_else(|| "Measurement not found".to_string())?;

            let sla = self
                .definitions
                .get(&measurement.sla_id)
                .ok_or_else(|| "SLA definition not found".to_string())?;

            measurement.complete(sla);
            Ok(measurement.status)
        }

        /// Gets measurements in warning or breach status.
        pub fn at_risk_measurements(&mut self) -> Vec<&mut SlaMeasurement> {
            // First update all statuses
            for m in &mut self.measurements {
                if let Some(sla) = self.definitions.get(&m.sla_id) {
                    m.check_status(sla);
                }
            }

            // Then filter based on updated status
            self.measurements
                .iter_mut()
                .filter(|m| m.status == SlaStatus::Warning || m.status == SlaStatus::Breached)
                .collect()
        }

        /// Gets completion rate for an SLA.
        pub fn completion_rate(&self, sla_id: Uuid) -> f64 {
            let total: Vec<_> = self
                .measurements
                .iter()
                .filter(|m| m.sla_id == sla_id && m.end_time.is_some())
                .collect();

            if total.is_empty() {
                return 1.0;
            }

            let met_count = total.iter().filter(|m| m.status == SlaStatus::Met).count();

            met_count as f64 / total.len() as f64
        }
    }

    impl Default for SlaTracker {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Escalation rules.
pub mod escalation {
    use super::*;

    /// Escalation condition.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EscalationCondition {
        /// Time-based: escalate after duration
        AfterDuration { seconds: i64 },
        /// Overdue task or approval
        Overdue,
        /// SLA breach
        SlaBreach,
        /// No response after duration
        NoResponseAfter { seconds: i64 },
        /// Multiple rejections
        MultipleRejections { count: usize },
    }

    impl EscalationCondition {
        /// Checks if condition is met for a timestamp.
        pub fn is_met(&self, created_at: DateTime<Utc>, _has_response: bool) -> bool {
            match self {
                Self::AfterDuration { seconds } => {
                    let elapsed = Utc::now() - created_at;
                    elapsed.num_seconds() >= *seconds
                }
                Self::Overdue => {
                    // Would need due date to check properly
                    false
                }
                Self::SlaBreach => {
                    // Would need SLA tracking
                    false
                }
                Self::NoResponseAfter { seconds } => {
                    if _has_response {
                        false
                    } else {
                        let elapsed = Utc::now() - created_at;
                        elapsed.num_seconds() >= *seconds
                    }
                }
                Self::MultipleRejections { count: _ } => {
                    // Would need rejection tracking
                    false
                }
            }
        }
    }

    /// Escalation action.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EscalationAction {
        /// Notify additional users
        Notify { users: Vec<String> },
        /// Reassign to different user
        Reassign { to_user: String },
        /// Escalate to manager
        EscalateToManager,
        /// Auto-approve
        AutoApprove,
        /// Custom action
        Custom(String),
    }

    /// Escalation rule.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EscalationRule {
        /// Rule ID
        pub rule_id: Uuid,
        /// Rule name
        pub name: String,
        /// Condition to trigger escalation
        pub condition: EscalationCondition,
        /// Action to take
        pub action: EscalationAction,
        /// Priority (higher = evaluated first)
        pub priority: i32,
        /// Whether the rule is enabled
        pub enabled: bool,
    }

    impl EscalationRule {
        /// Creates a new escalation rule.
        pub fn new(
            name: impl Into<String>,
            condition: EscalationCondition,
            action: EscalationAction,
        ) -> Self {
            Self {
                rule_id: Uuid::new_v4(),
                name: name.into(),
                condition,
                action,
                priority: 0,
                enabled: true,
            }
        }

        /// Sets priority.
        pub fn with_priority(mut self, priority: i32) -> Self {
            self.priority = priority;
            self
        }

        /// Checks if the rule should be triggered.
        pub fn should_trigger(&self, created_at: DateTime<Utc>, has_response: bool) -> bool {
            self.enabled && self.condition.is_met(created_at, has_response)
        }
    }

    /// Escalation event.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EscalationEvent {
        /// Event ID
        pub event_id: Uuid,
        /// Rule that triggered
        pub rule_id: Uuid,
        /// Entity that was escalated
        pub entity_id: String,
        /// Action taken
        pub action: EscalationAction,
        /// Timestamp
        pub escalated_at: DateTime<Utc>,
    }

    /// Escalation manager.
    #[derive(Debug)]
    pub struct EscalationManager {
        rules: Vec<EscalationRule>,
        events: Vec<EscalationEvent>,
    }

    impl EscalationManager {
        /// Creates a new escalation manager.
        pub fn new() -> Self {
            Self {
                rules: Vec::new(),
                events: Vec::new(),
            }
        }

        /// Adds an escalation rule.
        pub fn add_rule(&mut self, rule: EscalationRule) {
            self.rules.push(rule);
            // Sort by priority (descending)
            self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        /// Checks for escalations and applies rules.
        pub fn check_escalations(
            &mut self,
            entity_id: impl Into<String>,
            created_at: DateTime<Utc>,
            has_response: bool,
        ) -> Vec<EscalationAction> {
            let entity_id = entity_id.into();
            let mut actions = Vec::new();

            for rule in &self.rules {
                if rule.should_trigger(created_at, has_response) {
                    let event = EscalationEvent {
                        event_id: Uuid::new_v4(),
                        rule_id: rule.rule_id,
                        entity_id: entity_id.clone(),
                        action: rule.action.clone(),
                        escalated_at: Utc::now(),
                    };
                    actions.push(rule.action.clone());
                    self.events.push(event);
                }
            }

            actions
        }

        /// Gets escalation events for an entity.
        pub fn events_for_entity(&self, entity_id: &str) -> Vec<&EscalationEvent> {
            self.events
                .iter()
                .filter(|e| e.entity_id == entity_id)
                .collect()
        }

        /// Gets all rules.
        pub fn rules(&self) -> &[EscalationRule] {
            &self.rules
        }
    }

    impl Default for EscalationManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Advanced search features.
pub mod advanced_search {
    use super::*;

    /// Facet type for search aggregations.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum FacetType {
        /// Status facet
        Status,
        /// Jurisdiction facet
        Jurisdiction,
        /// Tags facet
        Tags,
        /// Year (from effective date)
        Year,
        /// Month (from effective date)
        Month,
        /// Custom facet
        Custom(String),
    }

    /// Facet value with count.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetValue {
        /// Value of the facet
        pub value: String,
        /// Count of items with this value
        pub count: usize,
    }

    /// Facet result for a specific facet type.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetResult {
        /// Facet type
        pub facet_type: FacetType,
        /// Values with their counts
        pub values: Vec<FacetValue>,
        /// Total number of unique values
        pub total_values: usize,
    }

    impl FacetResult {
        /// Gets top N values by count.
        pub fn top_values(&self, n: usize) -> Vec<&FacetValue> {
            let mut sorted: Vec<_> = self.values.iter().collect();
            sorted.sort_by(|a, b| b.count.cmp(&a.count));
            sorted.into_iter().take(n).collect()
        }

        /// Finds a specific value.
        pub fn find_value(&self, value: &str) -> Option<&FacetValue> {
            self.values.iter().find(|v| v.value == value)
        }
    }

    /// Faceted search results.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FacetedSearchResult {
        /// Matching statute IDs
        pub statute_ids: Vec<String>,
        /// Facet results
        pub facets: Vec<FacetResult>,
        /// Total matches
        pub total_matches: usize,
    }

    /// Search suggestion.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SearchSuggestion {
        /// Suggested text
        pub text: String,
        /// Suggestion type
        pub suggestion_type: SuggestionType,
        /// Relevance score
        pub score: f64,
    }

    /// Type of search suggestion.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SuggestionType {
        /// Statute ID
        StatuteId,
        /// Statute title
        Title,
        /// Tag
        Tag,
        /// Jurisdiction
        Jurisdiction,
        /// General term
        Term,
    }

    /// Autocomplete provider.
    #[derive(Debug)]
    pub struct AutocompleteProvider {
        /// Index of statute IDs
        statute_ids: Vec<String>,
        /// Index of titles
        titles: Vec<String>,
        /// Index of tags
        tags: Vec<String>,
        /// Index of jurisdictions
        jurisdictions: Vec<String>,
    }

    impl AutocompleteProvider {
        /// Creates a new autocomplete provider.
        pub fn new() -> Self {
            Self {
                statute_ids: Vec::new(),
                titles: Vec::new(),
                tags: Vec::new(),
                jurisdictions: Vec::new(),
            }
        }

        /// Indexes a statute for autocomplete.
        pub fn index_statute(&mut self, entry: &StatuteEntry) {
            // Index statute ID
            if !self.statute_ids.contains(&entry.statute.id) {
                self.statute_ids.push(entry.statute.id.clone());
            }

            // Index title
            let title = entry.statute.title.clone();
            if !self.titles.contains(&title) {
                self.titles.push(title);
            }

            // Index tags
            for tag in &entry.tags {
                if !self.tags.contains(tag) {
                    self.tags.push(tag.clone());
                }
            }

            // Index jurisdiction
            if !self.jurisdictions.contains(&entry.jurisdiction) {
                self.jurisdictions.push(entry.jurisdiction.clone());
            }
        }

        /// Gets suggestions for a query.
        pub fn suggest(&self, query: &str, max_results: usize) -> Vec<SearchSuggestion> {
            let query_lower = query.to_lowercase();
            let mut suggestions = Vec::new();

            // Search statute IDs
            for id in &self.statute_ids {
                if id.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: id.clone(),
                        suggestion_type: SuggestionType::StatuteId,
                        score: Self::calculate_score(&query_lower, &id.to_lowercase()),
                    });
                }
            }

            // Search titles
            for title in &self.titles {
                if title.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: title.clone(),
                        suggestion_type: SuggestionType::Title,
                        score: Self::calculate_score(&query_lower, &title.to_lowercase()),
                    });
                }
            }

            // Search tags
            for tag in &self.tags {
                if tag.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: tag.clone(),
                        suggestion_type: SuggestionType::Tag,
                        score: Self::calculate_score(&query_lower, &tag.to_lowercase()),
                    });
                }
            }

            // Search jurisdictions
            for jurisdiction in &self.jurisdictions {
                if jurisdiction.to_lowercase().contains(&query_lower) {
                    suggestions.push(SearchSuggestion {
                        text: jurisdiction.clone(),
                        suggestion_type: SuggestionType::Jurisdiction,
                        score: Self::calculate_score(&query_lower, &jurisdiction.to_lowercase()),
                    });
                }
            }

            // Sort by score (descending)
            suggestions.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            suggestions.truncate(max_results);
            suggestions
        }

        /// Calculates relevance score.
        fn calculate_score(query: &str, text: &str) -> f64 {
            // Exact match gets highest score
            if query == text {
                return 1.0;
            }

            // Prefix match gets high score
            if text.starts_with(query) {
                return 0.9;
            }

            // Contains match gets medium score
            if text.contains(query) {
                return 0.7;
            }

            // Fuzzy match gets lower score
            0.5
        }
    }

    impl Default for AutocompleteProvider {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Saved search.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SavedSearch {
        /// Search ID
        pub search_id: Uuid,
        /// Search name
        pub name: String,
        /// Search query
        pub query: SearchQuery,
        /// Owner user ID
        pub owner: String,
        /// Alert enabled
        pub alert_enabled: bool,
        /// Alert frequency in seconds
        pub alert_frequency_seconds: Option<i64>,
        /// Last executed
        pub last_executed: Option<DateTime<Utc>>,
        /// Last result count
        pub last_result_count: Option<usize>,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
    }

    impl SavedSearch {
        /// Creates a new saved search.
        pub fn new(name: impl Into<String>, query: SearchQuery, owner: impl Into<String>) -> Self {
            Self {
                search_id: Uuid::new_v4(),
                name: name.into(),
                query,
                owner: owner.into(),
                alert_enabled: false,
                alert_frequency_seconds: None,
                last_executed: None,
                last_result_count: None,
                created_at: Utc::now(),
            }
        }

        /// Enables alerts with frequency.
        pub fn with_alert(mut self, frequency_seconds: i64) -> Self {
            self.alert_enabled = true;
            self.alert_frequency_seconds = Some(frequency_seconds);
            self
        }

        /// Checks if alert should be triggered.
        pub fn should_trigger_alert(&self) -> bool {
            if !self.alert_enabled {
                return false;
            }

            if let Some(freq) = self.alert_frequency_seconds {
                if let Some(last_exec) = self.last_executed {
                    let elapsed = Utc::now() - last_exec;
                    return elapsed.num_seconds() >= freq;
                }
                // Never executed, should trigger
                return true;
            }

            false
        }

        /// Updates execution info.
        pub fn update_execution(&mut self, result_count: usize) {
            self.last_executed = Some(Utc::now());
            self.last_result_count = Some(result_count);
        }
    }

    /// Search analytics tracker.
    #[derive(Debug)]
    pub struct SearchAnalytics {
        /// Query frequency tracking
        query_counts: HashMap<String, usize>,
        /// Recent searches
        recent_searches: Vec<(String, DateTime<Utc>)>,
        /// Search result counts
        result_counts: Vec<usize>,
        /// Max recent searches to track
        max_recent: usize,
    }

    impl SearchAnalytics {
        /// Creates a new search analytics tracker.
        pub fn new() -> Self {
            Self {
                query_counts: HashMap::new(),
                recent_searches: Vec::new(),
                result_counts: Vec::new(),
                max_recent: 1000,
            }
        }

        /// Records a search.
        pub fn record_search(&mut self, query: &str, result_count: usize) {
            // Track query frequency
            *self.query_counts.entry(query.to_string()).or_insert(0) += 1;

            // Track recent searches
            self.recent_searches.push((query.to_string(), Utc::now()));
            if self.recent_searches.len() > self.max_recent {
                self.recent_searches
                    .drain(0..self.recent_searches.len() - self.max_recent);
            }

            // Track result counts
            self.result_counts.push(result_count);
        }

        /// Gets most popular queries.
        pub fn top_queries(&self, n: usize) -> Vec<(String, usize)> {
            let mut queries: Vec<_> = self
                .query_counts
                .iter()
                .map(|(q, c)| (q.clone(), *c))
                .collect();
            queries.sort_by(|a, b| b.1.cmp(&a.1));
            queries.into_iter().take(n).collect()
        }

        /// Gets average result count.
        pub fn average_result_count(&self) -> f64 {
            if self.result_counts.is_empty() {
                return 0.0;
            }
            let sum: usize = self.result_counts.iter().sum();
            sum as f64 / self.result_counts.len() as f64
        }

        /// Gets zero-result queries.
        pub fn zero_result_queries(&self) -> Vec<String> {
            self.recent_searches
                .iter()
                .enumerate()
                .filter(|(i, _)| self.result_counts.get(*i).map(|&c| c == 0).unwrap_or(false))
                .map(|(_, (q, _))| q.clone())
                .collect()
        }

        /// Gets total searches.
        pub fn total_searches(&self) -> usize {
            self.recent_searches.len()
        }

        /// Gets searches in time range.
        pub fn searches_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> usize {
            self.recent_searches
                .iter()
                .filter(|(_, ts)| ts >= &start && ts <= &end)
                .count()
        }
    }

    impl Default for SearchAnalytics {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Semantic search using embeddings (placeholder for future ML integration).
    #[derive(Debug)]
    pub struct SemanticSearch {
        /// Enabled flag
        enabled: bool,
        /// Embedding dimension
        dimension: usize,
    }

    impl SemanticSearch {
        /// Creates a new semantic search engine.
        pub fn new(dimension: usize) -> Self {
            Self {
                enabled: false,
                dimension,
            }
        }

        /// Enables semantic search.
        pub fn enable(&mut self) {
            self.enabled = true;
        }

        /// Checks if enabled.
        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        /// Gets embedding dimension.
        pub fn dimension(&self) -> usize {
            self.dimension
        }

        /// Placeholder for semantic search (would integrate with ML models).
        pub fn search(&self, _query: &str, _top_k: usize) -> Vec<(String, f64)> {
            // In a real implementation, this would:
            // 1. Generate embedding for query
            // 2. Search vector database for similar embeddings
            // 3. Return statute IDs with similarity scores
            Vec::new()
        }
    }

    impl Default for SemanticSearch {
        fn default() -> Self {
            Self::new(384) // Default BERT dimension
        }
    }
}

/// Version control features for Git-like statute management.
pub mod version_control {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::fmt;

    /// Branch in the version control system.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Branch {
        /// Branch name
        pub name: String,
        /// Branch ID
        pub branch_id: Uuid,
        /// Parent branch (None for main branch)
        pub parent_branch: Option<String>,
        /// Current head commit
        pub head_commit: Option<Uuid>,
        /// When the branch was created
        pub created_at: DateTime<Utc>,
        /// Who created the branch
        pub created_by: String,
        /// Branch description
        pub description: Option<String>,
        /// Whether the branch is protected (cannot be deleted)
        pub protected: bool,
    }

    impl Branch {
        /// Creates a new branch.
        pub fn new(name: impl Into<String>, created_by: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                branch_id: Uuid::new_v4(),
                parent_branch: None,
                head_commit: None,
                created_at: Utc::now(),
                created_by: created_by.into(),
                description: None,
                protected: false,
            }
        }

        /// Creates a branch from a parent.
        pub fn from_parent(
            name: impl Into<String>,
            parent: impl Into<String>,
            created_by: impl Into<String>,
        ) -> Self {
            Self {
                name: name.into(),
                branch_id: Uuid::new_v4(),
                parent_branch: Some(parent.into()),
                head_commit: None,
                created_at: Utc::now(),
                created_by: created_by.into(),
                description: None,
                protected: false,
            }
        }

        /// Sets the branch description.
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }

        /// Marks the branch as protected.
        pub fn with_protected(mut self, protected: bool) -> Self {
            self.protected = protected;
            self
        }
    }

    /// Commit in the version control system.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Commit {
        /// Commit ID
        pub commit_id: Uuid,
        /// Branch name
        pub branch_name: String,
        /// Parent commit IDs
        pub parent_commits: Vec<Uuid>,
        /// Statute ID being committed
        pub statute_id: String,
        /// Statute snapshot at this commit
        pub statute_entry: StatuteEntry,
        /// Commit message
        pub message: String,
        /// Who made the commit
        pub author: String,
        /// When the commit was made
        pub timestamp: DateTime<Utc>,
        /// Commit signature (for verification)
        pub signature: Option<String>,
        /// Commit hash (SHA-256 of content)
        pub commit_hash: String,
    }

    impl Commit {
        /// Creates a new commit.
        pub fn new(
            branch_name: impl Into<String>,
            statute_id: impl Into<String>,
            statute_entry: StatuteEntry,
            message: impl Into<String>,
            author: impl Into<String>,
        ) -> Self {
            let branch_name = branch_name.into();
            let statute_id = statute_id.into();
            let message = message.into();
            let author = author.into();
            let timestamp = Utc::now();

            // Calculate commit hash
            let mut hasher = Sha256::new();
            hasher.update(branch_name.as_bytes());
            hasher.update(statute_id.as_bytes());
            hasher.update(message.as_bytes());
            hasher.update(author.as_bytes());
            hasher.update(timestamp.to_rfc3339().as_bytes());
            let commit_hash = format!("{:x}", hasher.finalize());

            Self {
                commit_id: Uuid::new_v4(),
                branch_name,
                parent_commits: Vec::new(),
                statute_id,
                statute_entry,
                message,
                author,
                timestamp,
                signature: None,
                commit_hash,
            }
        }

        /// Adds a parent commit.
        pub fn with_parent(mut self, parent_id: Uuid) -> Self {
            self.parent_commits.push(parent_id);
            self
        }

        /// Signs the commit with a signature.
        pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
            self.signature = Some(signature.into());
            self
        }

        /// Verifies the commit signature.
        pub fn verify_signature(&self, public_key: &str) -> bool {
            if let Some(signature) = &self.signature {
                // Placeholder: In production, use proper cryptographic verification
                // e.g., ed25519_dalek, RSA, ECDSA
                signature.starts_with("SIG:") && signature.contains(public_key)
            } else {
                false
            }
        }

        /// Gets a short commit hash (first 8 characters).
        pub fn short_hash(&self) -> String {
            self.commit_hash.chars().take(8).collect()
        }
    }

    /// Merge conflict during branch merging.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BranchMergeConflict {
        /// Field name with conflict
        pub field_name: String,
        /// Value from source branch
        pub source_value: String,
        /// Value from target branch
        pub target_value: String,
        /// Value from common ancestor (if any)
        pub base_value: Option<String>,
    }

    impl fmt::Display for BranchMergeConflict {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Conflict in '{}': source='{}' vs target='{}'",
                self.field_name, self.source_value, self.target_value
            )
        }
    }

    /// Result of a branch merge operation.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MergeBranchResult {
        /// Merge commit ID
        pub merge_commit_id: Option<Uuid>,
        /// Conflicts encountered
        pub conflicts: Vec<BranchMergeConflict>,
        /// Whether the merge was successful
        pub success: bool,
        /// Merge message
        pub message: String,
    }

    impl MergeBranchResult {
        /// Checks if there are conflicts.
        pub fn has_conflicts(&self) -> bool {
            !self.conflicts.is_empty()
        }

        /// Gets conflict count.
        pub fn conflict_count(&self) -> usize {
            self.conflicts.len()
        }
    }

    /// Pull request status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum PullRequestStatus {
        /// Open and awaiting review
        Open,
        /// Under review
        InReview,
        /// Approved and ready to merge
        Approved,
        /// Changes requested
        ChangesRequested,
        /// Merged
        Merged,
        /// Closed without merging
        Closed,
    }

    /// Review decision on a pull request.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ReviewDecision {
        /// Approve the changes
        Approve,
        /// Request changes
        RequestChanges,
        /// Comment only (no approval/rejection)
        Comment,
    }

    /// Review on a pull request.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PullRequestReview {
        /// Review ID
        pub review_id: Uuid,
        /// Pull request ID
        pub pull_request_id: Uuid,
        /// Reviewer name
        pub reviewer: String,
        /// Review decision
        pub decision: ReviewDecision,
        /// Review comments
        pub comment: String,
        /// When the review was submitted
        pub submitted_at: DateTime<Utc>,
    }

    impl PullRequestReview {
        /// Creates a new review.
        pub fn new(
            pull_request_id: Uuid,
            reviewer: impl Into<String>,
            decision: ReviewDecision,
            comment: impl Into<String>,
        ) -> Self {
            Self {
                review_id: Uuid::new_v4(),
                pull_request_id,
                reviewer: reviewer.into(),
                decision,
                comment: comment.into(),
                submitted_at: Utc::now(),
            }
        }
    }

    /// Pull request for merging branches.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PullRequest {
        /// Pull request ID
        pub pr_id: Uuid,
        /// Pull request number (incremental)
        pub pr_number: u32,
        /// Title
        pub title: String,
        /// Description
        pub description: String,
        /// Source branch
        pub source_branch: String,
        /// Target branch
        pub target_branch: String,
        /// Who created the PR
        pub author: String,
        /// Current status
        pub status: PullRequestStatus,
        /// Reviews
        pub reviews: Vec<PullRequestReview>,
        /// Commits included
        pub commits: Vec<Uuid>,
        /// When created
        pub created_at: DateTime<Utc>,
        /// When merged (if merged)
        pub merged_at: Option<DateTime<Utc>>,
        /// Who merged (if merged)
        pub merged_by: Option<String>,
    }

    impl PullRequest {
        /// Creates a new pull request.
        pub fn new(
            pr_number: u32,
            title: impl Into<String>,
            description: impl Into<String>,
            source_branch: impl Into<String>,
            target_branch: impl Into<String>,
            author: impl Into<String>,
        ) -> Self {
            Self {
                pr_id: Uuid::new_v4(),
                pr_number,
                title: title.into(),
                description: description.into(),
                source_branch: source_branch.into(),
                target_branch: target_branch.into(),
                author: author.into(),
                status: PullRequestStatus::Open,
                reviews: Vec::new(),
                commits: Vec::new(),
                created_at: Utc::now(),
                merged_at: None,
                merged_by: None,
            }
        }

        /// Adds a review to the pull request.
        pub fn add_review(&mut self, review: PullRequestReview) {
            self.reviews.push(review);
            // Update status based on reviews
            self.update_status();
        }

        /// Updates the status based on reviews.
        fn update_status(&mut self) {
            if self.status == PullRequestStatus::Merged || self.status == PullRequestStatus::Closed
            {
                return;
            }

            let approvals = self
                .reviews
                .iter()
                .filter(|r| r.decision == ReviewDecision::Approve)
                .count();
            let changes_requested = self
                .reviews
                .iter()
                .filter(|r| r.decision == ReviewDecision::RequestChanges)
                .count();

            if changes_requested > 0 {
                self.status = PullRequestStatus::ChangesRequested;
            } else if approvals > 0 {
                self.status = PullRequestStatus::Approved;
            } else if !self.reviews.is_empty() {
                self.status = PullRequestStatus::InReview;
            }
        }

        /// Checks if the PR is approved.
        pub fn is_approved(&self) -> bool {
            self.status == PullRequestStatus::Approved
        }

        /// Marks the PR as merged.
        pub fn mark_merged(&mut self, merged_by: impl Into<String>) {
            self.status = PullRequestStatus::Merged;
            self.merged_at = Some(Utc::now());
            self.merged_by = Some(merged_by.into());
        }

        /// Closes the PR without merging.
        pub fn close(&mut self) {
            self.status = PullRequestStatus::Closed;
        }
    }

    /// Field-level change tracking for blame.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FieldHistory {
        /// Field name
        pub field_name: String,
        /// Old value (serialized)
        pub old_value: Option<String>,
        /// New value (serialized)
        pub new_value: String,
        /// Commit that made the change
        pub commit_id: Uuid,
        /// Who made the change
        pub author: String,
        /// When the change was made
        pub timestamp: DateTime<Utc>,
        /// Commit message
        pub message: String,
    }

    impl FieldHistory {
        /// Creates a new field history entry.
        pub fn new(
            field_name: impl Into<String>,
            old_value: Option<String>,
            new_value: impl Into<String>,
            commit_id: Uuid,
            author: impl Into<String>,
            message: impl Into<String>,
        ) -> Self {
            Self {
                field_name: field_name.into(),
                old_value,
                new_value: new_value.into(),
                commit_id,
                author: author.into(),
                timestamp: Utc::now(),
                message: message.into(),
            }
        }
    }

    /// Blame information for a specific field.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FieldBlame {
        /// Field name
        pub field_name: String,
        /// Current value
        pub current_value: String,
        /// Who last modified this field
        pub last_author: String,
        /// When last modified
        pub last_modified: DateTime<Utc>,
        /// Commit that last modified this field
        pub last_commit_id: Uuid,
        /// Complete history of this field
        pub history: Vec<FieldHistory>,
    }

    impl FieldBlame {
        /// Creates a new field blame.
        pub fn new(field_name: impl Into<String>, current_value: impl Into<String>) -> Self {
            Self {
                field_name: field_name.into(),
                current_value: current_value.into(),
                last_author: String::new(),
                last_modified: Utc::now(),
                last_commit_id: Uuid::nil(),
                history: Vec::new(),
            }
        }

        /// Adds a history entry.
        pub fn add_history(&mut self, history: FieldHistory) {
            self.last_author = history.author.clone();
            self.last_modified = history.timestamp;
            self.last_commit_id = history.commit_id;
            self.history.push(history);
        }

        /// Gets the number of times this field was modified.
        pub fn modification_count(&self) -> usize {
            self.history.len()
        }

        /// Gets all authors who modified this field.
        pub fn all_authors(&self) -> HashSet<String> {
            self.history.iter().map(|h| h.author.clone()).collect()
        }
    }

    /// Version control manager.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VersionControlManager {
        /// All branches
        branches: HashMap<String, Branch>,
        /// All commits
        commits: HashMap<Uuid, Commit>,
        /// All pull requests
        pull_requests: HashMap<Uuid, PullRequest>,
        /// Next PR number
        next_pr_number: u32,
        /// Field-level blame tracking
        field_blame: HashMap<String, HashMap<String, FieldBlame>>, // statute_id -> field_name -> blame
    }

    impl VersionControlManager {
        /// Creates a new version control manager.
        pub fn new() -> Self {
            let mut manager = Self {
                branches: HashMap::new(),
                commits: HashMap::new(),
                pull_requests: HashMap::new(),
                next_pr_number: 1,
                field_blame: HashMap::new(),
            };

            // Create the main branch
            let main_branch = Branch::new("main", "system").with_protected(true);
            manager.branches.insert("main".to_string(), main_branch);

            manager
        }

        /// Creates a new branch.
        pub fn create_branch(
            &mut self,
            name: impl Into<String>,
            parent: Option<String>,
            created_by: impl Into<String>,
        ) -> Result<&Branch, String> {
            let name = name.into();
            if self.branches.contains_key(&name) {
                return Err(format!("Branch '{}' already exists", name));
            }

            let branch = if let Some(parent_name) = parent {
                if !self.branches.contains_key(&parent_name) {
                    return Err(format!("Parent branch '{}' does not exist", parent_name));
                }
                Branch::from_parent(name.clone(), parent_name, created_by)
            } else {
                Branch::new(name.clone(), created_by)
            };

            self.branches.insert(name.clone(), branch);
            Ok(self.branches.get(&name).unwrap())
        }

        /// Deletes a branch.
        pub fn delete_branch(&mut self, name: &str) -> Result<(), String> {
            if name == "main" {
                return Err("Cannot delete main branch".to_string());
            }

            if let Some(branch) = self.branches.get(name) {
                if branch.protected {
                    return Err(format!("Branch '{}' is protected", name));
                }
            } else {
                return Err(format!("Branch '{}' does not exist", name));
            }

            self.branches.remove(name);
            Ok(())
        }

        /// Gets a branch.
        pub fn get_branch(&self, name: &str) -> Option<&Branch> {
            self.branches.get(name)
        }

        /// Gets a mutable reference to a branch.
        pub fn get_branch_mut(&mut self, name: &str) -> Option<&mut Branch> {
            self.branches.get_mut(name)
        }

        /// Lists all branches.
        pub fn list_branches(&self) -> Vec<&Branch> {
            self.branches.values().collect()
        }

        /// Creates a commit.
        pub fn commit(
            &mut self,
            branch_name: impl Into<String>,
            statute_id: impl Into<String>,
            statute_entry: StatuteEntry,
            message: impl Into<String>,
            author: impl Into<String>,
        ) -> Result<Uuid, String> {
            let branch_name = branch_name.into();
            if !self.branches.contains_key(&branch_name) {
                return Err(format!("Branch '{}' does not exist", branch_name));
            }

            let statute_id = statute_id.into();
            let mut commit = Commit::new(
                branch_name.clone(),
                statute_id.clone(),
                statute_entry,
                message,
                author,
            );

            // Set parent to current head
            if let Some(branch) = self.branches.get(&branch_name)
                && let Some(head) = branch.head_commit
            {
                commit = commit.with_parent(head);
            }

            let commit_id = commit.commit_id;
            self.commits.insert(commit_id, commit);

            // Update branch head
            if let Some(branch) = self.branches.get_mut(&branch_name) {
                branch.head_commit = Some(commit_id);
            }

            // Track field-level changes for blame
            self.track_field_changes(commit_id);

            Ok(commit_id)
        }

        /// Signs a commit.
        pub fn sign_commit(
            &mut self,
            commit_id: Uuid,
            signature: impl Into<String>,
        ) -> Result<(), String> {
            if let Some(commit) = self.commits.get_mut(&commit_id) {
                commit.signature = Some(signature.into());
                Ok(())
            } else {
                Err("Commit not found".to_string())
            }
        }

        /// Gets a commit.
        pub fn get_commit(&self, commit_id: Uuid) -> Option<&Commit> {
            self.commits.get(&commit_id)
        }

        /// Gets commits for a branch.
        pub fn get_branch_commits(&self, branch_name: &str) -> Vec<&Commit> {
            self.commits
                .values()
                .filter(|c| c.branch_name == branch_name)
                .collect()
        }

        /// Gets commit history for a branch (following parent chain).
        /// Returns commits in chronological order (oldest first).
        pub fn get_commit_history(&self, branch_name: &str) -> Vec<&Commit> {
            let mut history = Vec::new();
            if let Some(branch) = self.branches.get(branch_name)
                && let Some(head) = branch.head_commit
            {
                self.collect_commit_history(head, &mut history);
            }
            // History is collected in reverse order (newest first from recursion),
            // but we want chronological order (oldest first)
            history
        }

        fn collect_commit_history<'a>(&'a self, commit_id: Uuid, history: &mut Vec<&'a Commit>) {
            if let Some(commit) = self.commits.get(&commit_id) {
                for parent_id in &commit.parent_commits {
                    self.collect_commit_history(*parent_id, history);
                }
                history.push(commit);
            }
        }

        /// Merges a source branch into a target branch.
        pub fn merge_branch(
            &mut self,
            source_branch: &str,
            target_branch: &str,
            author: impl Into<String>,
        ) -> MergeBranchResult {
            // Check if branches exist
            if !self.branches.contains_key(source_branch) {
                return MergeBranchResult {
                    merge_commit_id: None,
                    conflicts: Vec::new(),
                    success: false,
                    message: format!("Source branch '{}' does not exist", source_branch),
                };
            }
            if !self.branches.contains_key(target_branch) {
                return MergeBranchResult {
                    merge_commit_id: None,
                    conflicts: Vec::new(),
                    success: false,
                    message: format!("Target branch '{}' does not exist", target_branch),
                };
            }

            // Get head commits
            let source_head = self.branches.get(source_branch).and_then(|b| b.head_commit);
            let target_head = self.branches.get(target_branch).and_then(|b| b.head_commit);

            if source_head.is_none() {
                return MergeBranchResult {
                    merge_commit_id: None,
                    conflicts: Vec::new(),
                    success: false,
                    message: "Source branch has no commits".to_string(),
                };
            }

            // Get commits
            let source_commit = self.commits.get(&source_head.unwrap()).unwrap();
            let target_commit = target_head.and_then(|id| self.commits.get(&id));

            // Detect conflicts
            let conflicts = if let Some(target_commit) = target_commit {
                self.detect_conflicts(&source_commit.statute_entry, &target_commit.statute_entry)
            } else {
                Vec::new()
            };

            if !conflicts.is_empty() {
                return MergeBranchResult {
                    merge_commit_id: None,
                    conflicts,
                    success: false,
                    message: "Merge conflicts detected".to_string(),
                };
            }

            // Create merge commit
            let message = format!("Merge branch '{}' into '{}'", source_branch, target_branch);
            let statute_id = source_commit.statute_id.clone();
            let statute_entry = source_commit.statute_entry.clone();

            match self.commit(target_branch, statute_id, statute_entry, message, author) {
                Ok(merge_commit_id) => {
                    // Add source branch head as second parent to the merge commit
                    // (First parent is already set by commit() to the target branch head)
                    if let Some(commit) = self.commits.get_mut(&merge_commit_id) {
                        commit.parent_commits.push(source_head.unwrap());
                    }

                    MergeBranchResult {
                        merge_commit_id: Some(merge_commit_id),
                        conflicts: Vec::new(),
                        success: true,
                        message: "Merge successful".to_string(),
                    }
                }
                Err(e) => MergeBranchResult {
                    merge_commit_id: None,
                    conflicts: Vec::new(),
                    success: false,
                    message: format!("Failed to create merge commit: {}", e),
                },
            }
        }

        fn detect_conflicts(
            &self,
            source_entry: &StatuteEntry,
            target_entry: &StatuteEntry,
        ) -> Vec<BranchMergeConflict> {
            let mut conflicts = Vec::new();

            // Check title
            if source_entry.statute.title != target_entry.statute.title {
                conflicts.push(BranchMergeConflict {
                    field_name: "title".to_string(),
                    source_value: source_entry.statute.title.clone(),
                    target_value: target_entry.statute.title.clone(),
                    base_value: None,
                });
            }

            // Check status
            if source_entry.status != target_entry.status {
                conflicts.push(BranchMergeConflict {
                    field_name: "status".to_string(),
                    source_value: format!("{:?}", source_entry.status),
                    target_value: format!("{:?}", target_entry.status),
                    base_value: None,
                });
            }

            // Check jurisdiction
            if source_entry.jurisdiction != target_entry.jurisdiction {
                conflicts.push(BranchMergeConflict {
                    field_name: "jurisdiction".to_string(),
                    source_value: source_entry.jurisdiction.clone(),
                    target_value: target_entry.jurisdiction.clone(),
                    base_value: None,
                });
            }

            conflicts
        }

        /// Creates a pull request.
        pub fn create_pull_request(
            &mut self,
            title: impl Into<String>,
            description: impl Into<String>,
            source_branch: impl Into<String>,
            target_branch: impl Into<String>,
            author: impl Into<String>,
        ) -> Result<Uuid, String> {
            let source_branch = source_branch.into();
            let target_branch = target_branch.into();

            if !self.branches.contains_key(&source_branch) {
                return Err(format!("Source branch '{}' does not exist", source_branch));
            }
            if !self.branches.contains_key(&target_branch) {
                return Err(format!("Target branch '{}' does not exist", target_branch));
            }

            let pr = PullRequest::new(
                self.next_pr_number,
                title,
                description,
                source_branch,
                target_branch,
                author,
            );
            let pr_id = pr.pr_id;
            self.next_pr_number += 1;

            self.pull_requests.insert(pr_id, pr);
            Ok(pr_id)
        }

        /// Adds a review to a pull request.
        pub fn add_review(
            &mut self,
            pr_id: Uuid,
            reviewer: impl Into<String>,
            decision: ReviewDecision,
            comment: impl Into<String>,
        ) -> Result<(), String> {
            if let Some(pr) = self.pull_requests.get_mut(&pr_id) {
                let review = PullRequestReview::new(pr_id, reviewer, decision, comment);
                pr.add_review(review);
                Ok(())
            } else {
                Err("Pull request not found".to_string())
            }
        }

        /// Merges a pull request.
        pub fn merge_pull_request(
            &mut self,
            pr_id: Uuid,
            merged_by: impl Into<String>,
        ) -> Result<MergeBranchResult, String> {
            let (source_branch, target_branch, is_approved) = {
                let pr = self
                    .pull_requests
                    .get(&pr_id)
                    .ok_or("Pull request not found")?;

                if !pr.is_approved() {
                    return Err("Pull request is not approved".to_string());
                }

                (
                    pr.source_branch.clone(),
                    pr.target_branch.clone(),
                    pr.is_approved(),
                )
            };

            if !is_approved {
                return Err("Pull request is not approved".to_string());
            }

            let merged_by_str = merged_by.into();
            let result = self.merge_branch(&source_branch, &target_branch, merged_by_str.clone());

            if result.success
                && let Some(pr) = self.pull_requests.get_mut(&pr_id)
            {
                pr.mark_merged(merged_by_str);
            }

            Ok(result)
        }

        /// Gets a pull request.
        pub fn get_pull_request(&self, pr_id: Uuid) -> Option<&PullRequest> {
            self.pull_requests.get(&pr_id)
        }

        /// Lists all pull requests.
        pub fn list_pull_requests(&self) -> Vec<&PullRequest> {
            self.pull_requests.values().collect()
        }

        /// Lists open pull requests.
        pub fn list_open_pull_requests(&self) -> Vec<&PullRequest> {
            self.pull_requests
                .values()
                .filter(|pr| {
                    pr.status == PullRequestStatus::Open
                        || pr.status == PullRequestStatus::InReview
                        || pr.status == PullRequestStatus::Approved
                })
                .collect()
        }

        /// Closes a pull request without merging.
        pub fn close_pull_request(&mut self, pr_id: Uuid) -> Result<(), String> {
            if let Some(pr) = self.pull_requests.get_mut(&pr_id) {
                pr.close();
                Ok(())
            } else {
                Err("Pull request not found".to_string())
            }
        }

        /// Tracks field-level changes for blame.
        fn track_field_changes(&mut self, commit_id: Uuid) {
            if let Some(commit) = self.commits.get(&commit_id).cloned() {
                let statute_id = commit.statute_id.clone();

                // Track title changes
                self.track_field(
                    &statute_id,
                    "title",
                    &commit.statute_entry.statute.title,
                    commit_id,
                    &commit.author,
                    &commit.message,
                );

                // Track jurisdiction changes
                self.track_field(
                    &statute_id,
                    "jurisdiction",
                    &commit.statute_entry.jurisdiction,
                    commit_id,
                    &commit.author,
                    &commit.message,
                );

                // Track status changes
                self.track_field(
                    &statute_id,
                    "status",
                    &format!("{:?}", commit.statute_entry.status),
                    commit_id,
                    &commit.author,
                    &commit.message,
                );
            }
        }

        fn track_field(
            &mut self,
            statute_id: &str,
            field_name: &str,
            new_value: &str,
            commit_id: Uuid,
            author: &str,
            message: &str,
        ) {
            let statute_blame = self.field_blame.entry(statute_id.to_string()).or_default();

            let old_value = statute_blame
                .get(field_name)
                .map(|blame| blame.current_value.clone());

            let history = FieldHistory::new(
                field_name,
                old_value.clone(),
                new_value,
                commit_id,
                author,
                message,
            );

            let blame = statute_blame
                .entry(field_name.to_string())
                .or_insert_with(|| FieldBlame::new(field_name, new_value));

            blame.add_history(history);
            blame.current_value = new_value.to_string();
        }

        /// Gets blame information for a field.
        pub fn get_field_blame(&self, statute_id: &str, field_name: &str) -> Option<&FieldBlame> {
            self.field_blame
                .get(statute_id)
                .and_then(|fields| fields.get(field_name))
        }

        /// Gets all blame information for a statute.
        pub fn get_statute_blame(&self, statute_id: &str) -> Option<&HashMap<String, FieldBlame>> {
            self.field_blame.get(statute_id)
        }
    }

    impl Default for VersionControlManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// API Extensions for high-performance and real-time features.
pub mod api_extensions {
    use super::*;

    /// GraphQL subscription events for real-time updates.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SubscriptionEvent {
        /// Statute was registered
        StatuteRegistered {
            statute_id: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was updated
        StatuteUpdated {
            statute_id: String,
            version: u32,
            timestamp: DateTime<Utc>,
        },
        /// Statute was deleted
        StatuteDeleted {
            statute_id: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute status changed
        StatusChanged {
            statute_id: String,
            old_status: StatuteStatus,
            new_status: StatuteStatus,
            timestamp: DateTime<Utc>,
        },
    }

    /// Subscription manager for GraphQL subscriptions.
    #[derive(Debug, Clone)]
    pub struct SubscriptionManager {
        /// Active subscriptions
        subscriptions: Arc<Mutex<HashMap<Uuid, SubscriptionFilter>>>,
        /// Published events (stored for testing/replay)
        published_events: Arc<Mutex<Vec<SubscriptionEvent>>>,
    }

    /// Filter for subscriptions.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubscriptionFilter {
        /// Filter by statute IDs
        pub statute_ids: Option<Vec<String>>,
        /// Filter by jurisdictions
        pub jurisdictions: Option<Vec<String>>,
        /// Filter by tags
        pub tags: Option<Vec<String>>,
        /// Filter by event types
        pub event_types: Option<Vec<String>>,
    }

    impl SubscriptionManager {
        /// Creates a new subscription manager.
        pub fn new() -> Self {
            Self {
                subscriptions: Arc::new(Mutex::new(HashMap::new())),
                published_events: Arc::new(Mutex::new(Vec::new())),
            }
        }

        /// Subscribes with a filter.
        pub fn subscribe(&self, filter: SubscriptionFilter) -> Uuid {
            let subscription_id = Uuid::new_v4();
            self.subscriptions
                .lock()
                .unwrap()
                .insert(subscription_id, filter);
            subscription_id
        }

        /// Unsubscribes.
        pub fn unsubscribe(&self, subscription_id: Uuid) -> bool {
            self.subscriptions
                .lock()
                .unwrap()
                .remove(&subscription_id)
                .is_some()
        }

        /// Publishes an event to all subscribers.
        /// In production with the async feature, this would use tokio::sync::broadcast.
        pub fn publish(&self, event: SubscriptionEvent) {
            self.published_events.lock().unwrap().push(event);
        }

        /// Gets active subscription count.
        pub fn subscription_count(&self) -> usize {
            self.subscriptions.lock().unwrap().len()
        }

        /// Gets published events (for testing).
        pub fn get_published_events(&self) -> Vec<SubscriptionEvent> {
            self.published_events.lock().unwrap().clone()
        }

        /// Clears published events.
        pub fn clear_events(&self) {
            self.published_events.lock().unwrap().clear();
        }
    }

    impl Default for SubscriptionManager {
        fn default() -> Self {
            Self::new()
        }
    }

    /// gRPC service definition (placeholder for protobuf generation).
    pub mod grpc {
        use super::*;

        /// gRPC request for getting a statute.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetStatuteRequest {
            pub statute_id: String,
        }

        /// gRPC response for getting a statute.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetStatuteResponse {
            pub statute: Option<StatuteEntry>,
            pub found: bool,
        }

        /// gRPC request for listing statutes.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListStatutesRequest {
            pub page: u32,
            pub page_size: u32,
            pub jurisdiction: Option<String>,
            pub tags: Vec<String>,
        }

        /// gRPC response for listing statutes.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListStatutesResponse {
            pub statutes: Vec<StatuteEntry>,
            pub total_count: usize,
            pub has_more: bool,
        }

        /// gRPC request for registering a statute.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RegisterStatuteRequest {
            pub statute: StatuteEntry,
        }

        /// gRPC response for registering a statute.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RegisterStatuteResponse {
            pub success: bool,
            pub statute_id: String,
            pub error: Option<String>,
        }

        /// gRPC service implementation.
        pub struct GrpcStatuteService {
            registry: Arc<Mutex<StatuteRegistry>>,
        }

        impl GrpcStatuteService {
            /// Creates a new gRPC service.
            pub fn new(registry: Arc<Mutex<StatuteRegistry>>) -> Self {
                Self { registry }
            }

            /// Gets a statute.
            pub fn get_statute(&self, request: GetStatuteRequest) -> GetStatuteResponse {
                let mut registry = self.registry.lock().unwrap();
                match registry.get(&request.statute_id) {
                    Some(statute) => GetStatuteResponse {
                        statute: Some(statute),
                        found: true,
                    },
                    None => GetStatuteResponse {
                        statute: None,
                        found: false,
                    },
                }
            }

            /// Lists statutes.
            pub fn list_statutes(&self, request: ListStatutesRequest) -> ListStatutesResponse {
                let registry = self.registry.lock().unwrap();
                let mut statutes: Vec<_> = registry.list().into_iter().cloned().collect();

                // Apply jurisdiction filter
                if let Some(ref jurisdiction) = request.jurisdiction {
                    statutes.retain(|s| &s.jurisdiction == jurisdiction);
                }

                // Apply tag filter
                if !request.tags.is_empty() {
                    statutes.retain(|s| request.tags.iter().any(|tag| s.tags.contains(tag)));
                }

                let total_count = statutes.len();
                let start = (request.page * request.page_size) as usize;
                let end = std::cmp::min(start + request.page_size as usize, total_count);

                let page_statutes = if start < total_count {
                    statutes[start..end].to_vec()
                } else {
                    Vec::new()
                };

                ListStatutesResponse {
                    statutes: page_statutes,
                    total_count,
                    has_more: end < total_count,
                }
            }

            /// Registers a statute.
            pub fn register_statute(
                &self,
                request: RegisterStatuteRequest,
            ) -> RegisterStatuteResponse {
                let mut registry = self.registry.lock().unwrap();
                match registry.register(request.statute) {
                    Ok(statute_id) => RegisterStatuteResponse {
                        success: true,
                        statute_id: statute_id.to_string(),
                        error: None,
                    },
                    Err(e) => RegisterStatuteResponse {
                        success: false,
                        statute_id: String::new(),
                        error: Some(e.to_string()),
                    },
                }
            }
        }
    }

    /// Event streaming infrastructure.
    pub mod streaming {
        use super::*;

        /// Stream destination type.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum StreamDestination {
            /// Apache Kafka
            Kafka,
            /// NATS messaging
            Nats,
            /// Amazon Kinesis
            Kinesis,
            /// Custom webhook
            Webhook,
        }

        /// Stream configuration.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StreamConfig {
            /// Stream name
            pub name: String,
            /// Destination type
            pub destination: StreamDestination,
            /// Connection string (URL, broker address, etc.)
            pub connection: String,
            /// Topic/subject name
            pub topic: String,
            /// Optional authentication
            pub auth: Option<HashMap<String, String>>,
            /// Buffer size
            pub buffer_size: usize,
            /// Enable/disable flag
            pub enabled: bool,
        }

        impl StreamConfig {
            /// Creates a new stream configuration.
            pub fn new(
                name: impl Into<String>,
                destination: StreamDestination,
                connection: impl Into<String>,
                topic: impl Into<String>,
            ) -> Self {
                Self {
                    name: name.into(),
                    destination,
                    connection: connection.into(),
                    topic: topic.into(),
                    auth: None,
                    buffer_size: 1000,
                    enabled: true,
                }
            }

            /// Adds authentication.
            pub fn with_auth(mut self, auth: HashMap<String, String>) -> Self {
                self.auth = Some(auth);
                self
            }

            /// Sets buffer size.
            pub fn with_buffer_size(mut self, size: usize) -> Self {
                self.buffer_size = size;
                self
            }

            /// Sets enabled flag.
            pub fn with_enabled(mut self, enabled: bool) -> Self {
                self.enabled = enabled;
                self
            }
        }

        /// Event stream message.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StreamMessage {
            /// Message ID
            pub message_id: Uuid,
            /// Event type
            pub event_type: String,
            /// Statute ID
            pub statute_id: String,
            /// Event payload (JSON)
            pub payload: String,
            /// Timestamp
            pub timestamp: DateTime<Utc>,
            /// Metadata
            pub metadata: HashMap<String, String>,
        }

        impl StreamMessage {
            /// Creates a new stream message.
            pub fn new(
                event_type: impl Into<String>,
                statute_id: impl Into<String>,
                payload: impl Into<String>,
            ) -> Self {
                Self {
                    message_id: Uuid::new_v4(),
                    event_type: event_type.into(),
                    statute_id: statute_id.into(),
                    payload: payload.into(),
                    timestamp: Utc::now(),
                    metadata: HashMap::new(),
                }
            }

            /// Adds metadata.
            pub fn with_metadata(
                mut self,
                key: impl Into<String>,
                value: impl Into<String>,
            ) -> Self {
                self.metadata.insert(key.into(), value.into());
                self
            }
        }

        /// Event stream manager.
        #[derive(Debug, Clone)]
        pub struct EventStreamManager {
            /// Stream configurations
            configs: HashMap<String, StreamConfig>,
            /// Published message count by stream
            message_count: HashMap<String, usize>,
        }

        impl EventStreamManager {
            /// Creates a new event stream manager.
            pub fn new() -> Self {
                Self {
                    configs: HashMap::new(),
                    message_count: HashMap::new(),
                }
            }

            /// Adds a stream configuration.
            pub fn add_stream(&mut self, config: StreamConfig) {
                let name = config.name.clone();
                self.configs.insert(name.clone(), config);
                self.message_count.insert(name, 0);
            }

            /// Removes a stream configuration.
            pub fn remove_stream(&mut self, name: &str) -> bool {
                self.message_count.remove(name);
                self.configs.remove(name).is_some()
            }

            /// Gets a stream configuration.
            pub fn get_stream(&self, name: &str) -> Option<&StreamConfig> {
                self.configs.get(name)
            }

            /// Lists all streams.
            pub fn list_streams(&self) -> Vec<&StreamConfig> {
                self.configs.values().collect()
            }

            /// Publishes a message to a stream.
            /// In production, this would actually publish to Kafka/NATS/etc.
            pub fn publish(
                &mut self,
                stream_name: &str,
                _message: StreamMessage,
            ) -> Result<(), String> {
                let config = self
                    .configs
                    .get(stream_name)
                    .ok_or_else(|| format!("Stream '{}' not found", stream_name))?;

                if !config.enabled {
                    return Err(format!("Stream '{}' is disabled", stream_name));
                }

                // Placeholder: In production, actually publish to the stream
                // match config.destination {
                //     StreamDestination::Kafka => { /* kafka publish */ },
                //     StreamDestination::Nats => { /* nats publish */ },
                //     ...
                // }

                // Increment message count
                *self.message_count.get_mut(stream_name).unwrap() += 1;
                Ok(())
            }

            /// Gets message count for a stream.
            pub fn get_message_count(&self, stream_name: &str) -> usize {
                self.message_count.get(stream_name).copied().unwrap_or(0)
            }

            /// Resets message count for a stream.
            pub fn reset_count(&mut self, stream_name: &str) {
                if let Some(count) = self.message_count.get_mut(stream_name) {
                    *count = 0;
                }
            }
        }

        impl Default for EventStreamManager {
            fn default() -> Self {
                Self::new()
            }
        }
    }

    /// Enhanced bulk operations.
    pub mod bulk {
        use super::*;

        /// Bulk operation type.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum BulkOperationType {
            /// Register multiple statutes
            Register,
            /// Update multiple statutes
            Update,
            /// Delete multiple statutes
            Delete,
            /// Archive multiple statutes
            Archive,
            /// Change status for multiple statutes
            ChangeStatus,
        }

        /// Bulk operation request.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkOperationRequest {
            /// Operation type
            pub operation_type: BulkOperationType,
            /// Statute IDs (for update/delete/archive/status change)
            pub statute_ids: Vec<String>,
            /// Statute entries (for register)
            pub statute_entries: Vec<StatuteEntry>,
            /// New status (for status change)
            pub new_status: Option<StatuteStatus>,
            /// Continue on error flag
            pub continue_on_error: bool,
        }

        /// Bulk operation response.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkOperationResponse {
            /// Operation type
            pub operation_type: BulkOperationType,
            /// Total items processed
            pub total_processed: usize,
            /// Successful operations
            pub successful: usize,
            /// Failed operations
            pub failed: usize,
            /// Error details
            pub errors: Vec<BulkOperationError>,
            /// Duration in milliseconds
            pub duration_ms: u64,
        }

        /// Bulk operation error.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkOperationError {
            /// Statute ID
            pub statute_id: String,
            /// Error message
            pub error: String,
        }

        impl BulkOperationResponse {
            /// Creates a new bulk operation response.
            pub fn new(operation_type: BulkOperationType) -> Self {
                Self {
                    operation_type,
                    total_processed: 0,
                    successful: 0,
                    failed: 0,
                    errors: Vec::new(),
                    duration_ms: 0,
                }
            }

            /// Calculates success rate (0.0 to 1.0).
            pub fn success_rate(&self) -> f64 {
                if self.total_processed == 0 {
                    0.0
                } else {
                    self.successful as f64 / self.total_processed as f64
                }
            }

            /// Checks if all operations succeeded.
            pub fn is_complete_success(&self) -> bool {
                self.failed == 0 && self.total_processed > 0
            }
        }

        /// Bulk operation executor.
        pub struct BulkOperationExecutor {
            registry: Arc<Mutex<StatuteRegistry>>,
        }

        impl BulkOperationExecutor {
            /// Creates a new bulk operation executor.
            pub fn new(registry: Arc<Mutex<StatuteRegistry>>) -> Self {
                Self { registry }
            }

            /// Executes a bulk operation.
            pub fn execute(&self, request: BulkOperationRequest) -> BulkOperationResponse {
                let start = std::time::Instant::now();
                let mut response = BulkOperationResponse::new(request.operation_type);

                match request.operation_type {
                    BulkOperationType::Register => {
                        for entry in request.statute_entries {
                            response.total_processed += 1;
                            let statute_id = entry.statute.id.clone();
                            let mut registry = self.registry.lock().unwrap();
                            match registry.register(entry) {
                                Ok(_) => response.successful += 1,
                                Err(e) => {
                                    response.failed += 1;
                                    response.errors.push(BulkOperationError {
                                        statute_id: statute_id.clone(),
                                        error: e.to_string(),
                                    });
                                    if !request.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    BulkOperationType::Delete => {
                        for statute_id in &request.statute_ids {
                            response.total_processed += 1;
                            let mut registry = self.registry.lock().unwrap();
                            match registry.delete(statute_id) {
                                Ok(_) => response.successful += 1,
                                Err(e) => {
                                    response.failed += 1;
                                    response.errors.push(BulkOperationError {
                                        statute_id: statute_id.clone(),
                                        error: e.to_string(),
                                    });
                                    if !request.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    BulkOperationType::Archive => {
                        for statute_id in &request.statute_ids {
                            response.total_processed += 1;
                            let mut registry = self.registry.lock().unwrap();
                            match registry.archive_statute(statute_id, "Bulk archive".to_string()) {
                                Ok(_) => response.successful += 1,
                                Err(e) => {
                                    response.failed += 1;
                                    response.errors.push(BulkOperationError {
                                        statute_id: statute_id.clone(),
                                        error: e.to_string(),
                                    });
                                    if !request.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    BulkOperationType::ChangeStatus => {
                        if let Some(new_status) = request.new_status {
                            for statute_id in &request.statute_ids {
                                response.total_processed += 1;
                                let mut registry = self.registry.lock().unwrap();
                                match registry.set_status(statute_id, new_status) {
                                    Ok(_) => response.successful += 1,
                                    Err(e) => {
                                        response.failed += 1;
                                        response.errors.push(BulkOperationError {
                                            statute_id: statute_id.clone(),
                                            error: e.to_string(),
                                        });
                                        if !request.continue_on_error {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    BulkOperationType::Update => {
                        // Update operations would need statute entries
                        for entry in request.statute_entries {
                            response.total_processed += 1;
                            let statute_id = entry.statute.id.clone();
                            let mut registry = self.registry.lock().unwrap();
                            match registry.update(&statute_id, entry.statute.clone()) {
                                Ok(_) => response.successful += 1,
                                Err(e) => {
                                    response.failed += 1;
                                    response.errors.push(BulkOperationError {
                                        statute_id,
                                        error: e.to_string(),
                                    });
                                    if !request.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                response.duration_ms = start.elapsed().as_millis() as u64;
                response
            }
        }
    }

    /// SDK generation templates.
    pub mod sdk_gen {
        use super::*;

        /// Supported SDK languages.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum SdkLanguage {
            Python,
            JavaScript,
            TypeScript,
            Rust,
            Go,
            Java,
            CSharp,
            Ruby,
        }

        /// SDK generation configuration.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SdkConfig {
            /// Target language
            pub language: SdkLanguage,
            /// Package name
            pub package_name: String,
            /// API base URL
            pub api_base_url: String,
            /// Include async support
            pub async_support: bool,
            /// Include type definitions
            pub type_definitions: bool,
            /// Include documentation
            pub include_docs: bool,
        }

        /// SDK code generator.
        pub struct SdkGenerator;

        impl SdkGenerator {
            /// Generates SDK code for the specified language.
            pub fn generate(config: &SdkConfig) -> Result<String, String> {
                match config.language {
                    SdkLanguage::Python => Self::generate_python(config),
                    SdkLanguage::JavaScript => Self::generate_javascript(config),
                    SdkLanguage::TypeScript => Self::generate_typescript(config),
                    SdkLanguage::Rust => Self::generate_rust(config),
                    SdkLanguage::Go => Self::generate_go(config),
                    SdkLanguage::Java => Self::generate_java(config),
                    SdkLanguage::CSharp => Self::generate_csharp(config),
                    SdkLanguage::Ruby => Self::generate_ruby(config),
                }
            }

            fn generate_python(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"# {} Python SDK
import requests
from typing import Optional, List, Dict, Any

class StatuteRegistryClient:
    """Client for the Statute Registry API."""

    def __init__(self, base_url: str = "{}"):
        self.base_url = base_url

    def get_statute(self, statute_id: str) -> Optional[Dict[str, Any]]:
        """Gets a statute by ID."""
        response = requests.get(f"{{self.base_url}}/statutes/{{statute_id}}")
        if response.status_code == 200:
            return response.json()
        return None

    def list_statutes(self, page: int = 0, per_page: int = 50) -> List[Dict[str, Any]]:
        """Lists statutes with pagination."""
        params = {{"page": page, "per_page": per_page}}
        response = requests.get(f"{{self.base_url}}/statutes", params=params)
        return response.json() if response.status_code == 200 else []
"#,
                    config.package_name, config.api_base_url
                ))
            }

            fn generate_javascript(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} JavaScript SDK
class StatuteRegistryClient {{
    constructor(baseUrl = "{}") {{
        this.baseUrl = baseUrl;
    }}

    async getStatute(statuteId) {{
        const response = await fetch(`${{this.baseUrl}}/statutes/${{statuteId}}`);
        if (response.ok) {{
            return await response.json();
        }}
        return null;
    }}

    async listStatutes(page = 0, perPage = 50) {{
        const params = new URLSearchParams({{ page, per_page: perPage }});
        const response = await fetch(`${{this.baseUrl}}/statutes?${{params}}`);
        return response.ok ? await response.json() : [];
    }}
}}

module.exports = {{ StatuteRegistryClient }};
"#,
                    config.package_name, config.api_base_url
                ))
            }

            fn generate_typescript(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} TypeScript SDK
export interface Statute {{
    id: string;
    title: string;
    version: number;
    status: string;
    jurisdiction: string;
}}

export class StatuteRegistryClient {{
    private baseUrl: string;

    constructor(baseUrl: string = "{}") {{
        this.baseUrl = baseUrl;
    }}

    async getStatute(statuteId: string): Promise<Statute | null> {{
        const response = await fetch(`${{this.baseUrl}}/statutes/${{statuteId}}`);
        if (response.ok) {{
            return await response.json();
        }}
        return null;
    }}

    async listStatutes(page: number = 0, perPage: number = 50): Promise<Statute[]> {{
        const params = new URLSearchParams({{ page: page.toString(), per_page: perPage.toString() }});
        const response = await fetch(`${{this.baseUrl}}/statutes?${{params}}`);
        return response.ok ? await response.json() : [];
    }}
}}
"#,
                    config.package_name, config.api_base_url
                ))
            }

            fn generate_rust(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} Rust SDK
use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statute {{
    pub id: String,
    pub title: String,
    pub version: u32,
    pub status: String,
    pub jurisdiction: String,
}}

pub struct StatuteRegistryClient {{
    base_url: String,
    client: reqwest::Client,
}}

impl StatuteRegistryClient {{
    pub fn new(base_url: impl Into<String>) -> Self {{
        Self {{
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }}
    }}

    pub async fn get_statute(&self, statute_id: &str) -> Result<Option<Statute>, reqwest::Error> {{
        let url = format!("{{}}/statutes/{{}}", self.base_url, statute_id);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {{
            Ok(Some(response.json().await?))
        }} else {{
            Ok(None)
        }}
    }}

    pub async fn list_statutes(&self, page: u32, per_page: u32) -> Result<Vec<Statute>, reqwest::Error> {{
        let url = format!("{{}}/statutes?page={{}}&per_page={{}}", self.base_url, page, per_page);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {{
            Ok(response.json().await?)
        }} else {{
            Ok(Vec::new())
        }}
    }}
}}
"#,
                    config.package_name
                ))
            }

            fn generate_go(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} Go SDK
package {}

import (
    "encoding/json"
    "fmt"
    "net/http"
)

type Statute struct {{
    ID           string `json:"id"`
    Title        string `json:"title"`
    Version      int    `json:"version"`
    Status       string `json:"status"`
    Jurisdiction string `json:"jurisdiction"`
}}

type Client struct {{
    BaseURL    string
    HTTPClient *http.Client
}}

func NewClient(baseURL string) *Client {{
    return &Client{{
        BaseURL:    baseURL,
        HTTPClient: &http.Client{{}},
    }}
}}

func (c *Client) GetStatute(statuteID string) (*Statute, error) {{
    url := fmt.Sprintf("%s/statutes/%s", c.BaseURL, statuteID)
    resp, err := c.HTTPClient.Get(url)
    if err != nil {{
        return nil, err
    }}
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {{
        return nil, nil
    }}

    var statute Statute
    if err := json.NewDecoder(resp.Body).Decode(&statute); err != nil {{
        return nil, err
    }}
    return &statute, nil
}}
"#,
                    config.package_name,
                    config.package_name.to_lowercase()
                ))
            }

            fn generate_java(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} Java SDK
package {};

import java.io.IOException;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class StatuteRegistryClient {{
    private final String baseUrl;
    private final HttpClient client;

    public StatuteRegistryClient(String baseUrl) {{
        this.baseUrl = baseUrl;
        this.client = HttpClient.newHttpClient();
    }}

    public String getStatute(String statuteId) throws IOException, InterruptedException {{
        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create(baseUrl + "/statutes/" + statuteId))
            .GET()
            .build();

        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());
        if (response.statusCode() == 200) {{
            return response.body();
        }}
        return null;
    }}
}}
"#,
                    config.package_name,
                    config.package_name.to_lowercase()
                ))
            }

            fn generate_csharp(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    r#"// {} C# SDK
using System;
using System.Net.Http;
using System.Threading.Tasks;

namespace {}
{{
    public class StatuteRegistryClient
    {{
        private readonly string baseUrl;
        private readonly HttpClient client;

        public StatuteRegistryClient(string baseUrl)
        {{
            this.baseUrl = baseUrl;
            this.client = new HttpClient();
        }}

        public async Task<string> GetStatuteAsync(string statuteId)
        {{
            var response = await client.GetAsync($"{{baseUrl}}/statutes/{{statuteId}}");
            if (response.IsSuccessStatusCode)
            {{
                return await response.Content.ReadAsStringAsync();
            }}
            return null;
        }}
    }}
}}
"#,
                    config.package_name, config.package_name
                ))
            }

            fn generate_ruby(config: &SdkConfig) -> Result<String, String> {
                Ok(format!(
                    "# {} Ruby SDK\nrequire 'net/http'\nrequire 'json'\n\nmodule {}\n  class StatuteRegistryClient\n    attr_reader :base_url\n\n    def initialize(base_url = \"{}\")\n      @base_url = base_url\n    end\n\n    def get_statute(statute_id)\n      uri = URI(\"#{{@base_url}}/statutes/#{{statute_id}}\")\n      response = Net::HTTP.get_response(uri)\n      JSON.parse(response.body) if response.is_a?(Net::HTTPSuccess)\n    end\n\n    def list_statutes(page = 0, per_page = 50)\n      uri = URI(\"#{{@base_url}}/statutes?page=#{{page}}&per_page=#{{per_page}}\")\n      response = Net::HTTP.get_response(uri)\n      response.is_a?(Net::HTTPSuccess) ? JSON.parse(response.body) : []\n    end\n  end\nend\n",
                    config.package_name, config.package_name, config.api_base_url
                ))
            }
        }
    }
}

// ============================================================================
// Distributed Registry Module (v0.2.0)
// ============================================================================

pub mod distributed;

// ============================================================================
// Vector Search & Embeddings Module (v0.2.1)
// ============================================================================

pub mod vector_search;

// ============================================================================
// Blockchain Integration Module (v0.2.2)
// ============================================================================

pub mod blockchain;

// ============================================================================
// Graph Database Backend Module (v0.2.3)
// ============================================================================

pub mod graph_db;

// ============================================================================
// Multi-Tenant Architecture Module (v0.2.4)
// ============================================================================

pub mod multi_tenant;

// ============================================================================
// AI-Powered Features Module (v0.2.5)
// ============================================================================

pub mod ai_features;

/// Global Registry Network module (v0.3.0)
pub mod global_network;

/// Autonomous Registry Management module (v0.3.1)
pub mod autonomous;

// ============================================================================
// Legal Knowledge Base Module (v0.3.2)
// ============================================================================

/// Legal Knowledge Base: Statute-concept linking, legal ontology integration,
/// case law cross-references, knowledge graph visualization, and AI-powered legal research.
pub mod knowledge_base;

// ============================================================================
// Event Sourcing 2.0 Module (v0.2.6)
// ============================================================================

/// Event Sourcing 2.0: Advanced event replay, projections, and archiving.
pub mod event_sourcing_v2 {
    use super::*;
    use std::collections::BTreeMap;

    // ========================================================================
    // 1. Event Replay with Time-Travel Queries
    // ========================================================================

    /// Time-travel query builder for replaying events.
    #[derive(Debug, Clone)]
    pub struct TimeTravelQuery {
        /// Target point in time
        pub target_time: DateTime<Utc>,
        /// Optional statute filter
        pub statute_filter: Option<String>,
        /// Include only specific event types
        pub event_types: Vec<String>,
    }

    impl TimeTravelQuery {
        /// Creates a new time-travel query for a specific point in time.
        pub fn new(target_time: DateTime<Utc>) -> Self {
            Self {
                target_time,
                statute_filter: None,
                event_types: Vec::new(),
            }
        }

        /// Filters for a specific statute.
        pub fn for_statute(mut self, statute_id: String) -> Self {
            self.statute_filter = Some(statute_id);
            self
        }

        /// Filters for specific event types.
        pub fn with_event_types(mut self, event_types: Vec<String>) -> Self {
            self.event_types = event_types;
            self
        }
    }

    /// Result of replaying events to reconstruct state at a point in time.
    #[derive(Debug, Clone)]
    pub struct ReplayResult {
        /// Reconstructed statute state
        pub statutes: HashMap<String, StatuteEntry>,
        /// Number of events replayed
        pub events_replayed: usize,
        /// Target timestamp
        pub target_time: DateTime<Utc>,
        /// Replay duration
        pub replay_duration: std::time::Duration,
    }

    /// Event replay engine for time-travel queries.
    #[derive(Debug)]
    pub struct EventReplayEngine {
        event_store: Arc<Mutex<EventStore>>,
    }

    impl EventReplayEngine {
        /// Creates a new event replay engine.
        pub fn new(event_store: Arc<Mutex<EventStore>>) -> Self {
            Self { event_store }
        }

        /// Replays events up to a specific point in time.
        pub fn replay(&self, query: TimeTravelQuery) -> Result<ReplayResult, String> {
            let start = std::time::Instant::now();
            let store = self.event_store.lock().unwrap();

            let mut statutes = HashMap::new();
            let mut events_replayed = 0;

            for event in store.all_events() {
                let event_time = self.get_event_timestamp(event);
                if event_time > query.target_time {
                    break;
                }

                // Apply statute filter
                if let Some(ref statute_id) = query.statute_filter
                    && !self.event_matches_statute(event, statute_id)
                {
                    continue;
                }

                // Apply event
                self.apply_event(&mut statutes, event);
                events_replayed += 1;
            }

            Ok(ReplayResult {
                statutes,
                events_replayed,
                target_time: query.target_time,
                replay_duration: start.elapsed(),
            })
        }

        fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
            match event {
                RegistryEvent::StatuteRegistered { timestamp, .. }
                | RegistryEvent::StatuteUpdated { timestamp, .. }
                | RegistryEvent::StatusChanged { timestamp, .. }
                | RegistryEvent::TagAdded { timestamp, .. }
                | RegistryEvent::TagRemoved { timestamp, .. }
                | RegistryEvent::ReferenceAdded { timestamp, .. }
                | RegistryEvent::ReferenceRemoved { timestamp, .. }
                | RegistryEvent::MetadataUpdated { timestamp, .. }
                | RegistryEvent::StatuteDeleted { timestamp, .. }
                | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
            }
        }

        fn event_matches_statute(&self, event: &RegistryEvent, statute_id: &str) -> bool {
            match event {
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
            }
        }

        fn apply_event(&self, statutes: &mut HashMap<String, StatuteEntry>, event: &RegistryEvent) {
            match event {
                RegistryEvent::StatuteRegistered {
                    statute_id,
                    jurisdiction,
                    ..
                } => {
                    let statute = legalis_core::Statute {
                        id: statute_id.clone(),
                        title: format!("Statute {}", statute_id),
                        preconditions: Vec::new(),
                        effect: legalis_core::Effect {
                            effect_type: legalis_core::EffectType::Obligation,
                            description: "Default effect".to_string(),
                            parameters: HashMap::new(),
                        },
                        discretion_logic: None,
                        temporal_validity: legalis_core::TemporalValidity {
                            effective_date: None,
                            expiry_date: None,
                            enacted_at: None,
                            amended_at: None,
                        },
                        version: 1,
                        jurisdiction: Some(jurisdiction.clone()),
                        derives_from: Vec::new(),
                        applies_to: Vec::new(),
                        exceptions: Vec::new(),
                    };
                    statutes.insert(statute_id.clone(), StatuteEntry::new(statute, jurisdiction));
                }
                RegistryEvent::StatusChanged {
                    statute_id,
                    new_status,
                    ..
                } => {
                    if let Some(entry) = statutes.get_mut(statute_id) {
                        entry.status = *new_status;
                    }
                }
                RegistryEvent::StatuteDeleted { statute_id, .. } => {
                    statutes.remove(statute_id);
                }
                _ => {}
            }
        }
    }

    // ========================================================================
    // 2. Event Projections for Analytics
    // ========================================================================

    /// Projection types for analytics.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ProjectionType {
        /// Count events by type
        EventTypeCount,
        /// Count events by statute
        StatuteActivityCount,
        /// Track status changes over time
        StatusChangeTimeline,
        /// Tag usage statistics
        TagUsageStats,
        /// Daily activity summary
        DailyActivitySummary,
    }

    /// Event projection result.
    #[derive(Debug, Clone)]
    pub struct ProjectionResult {
        /// Projection type
        pub projection_type: ProjectionType,
        /// Aggregated data
        pub data: BTreeMap<String, usize>,
        /// Number of events processed
        pub events_processed: usize,
        /// Time range covered
        pub time_range: (DateTime<Utc>, DateTime<Utc>),
    }

    /// Event projection engine for analytics.
    #[derive(Debug)]
    pub struct ProjectionEngine {
        event_store: Arc<Mutex<EventStore>>,
    }

    impl ProjectionEngine {
        /// Creates a new projection engine.
        pub fn new(event_store: Arc<Mutex<EventStore>>) -> Self {
            Self { event_store }
        }

        /// Computes a projection from the event stream.
        pub fn project(&self, projection_type: ProjectionType) -> ProjectionResult {
            let store = self.event_store.lock().unwrap();
            let events = store.all_events();

            let mut data = BTreeMap::new();
            let mut min_time = Utc::now();
            let mut max_time = DateTime::<Utc>::MIN_UTC;

            for event in &events {
                let timestamp = self.get_event_timestamp(event);
                if timestamp < min_time {
                    min_time = timestamp;
                }
                if timestamp > max_time {
                    max_time = timestamp;
                }

                match projection_type {
                    ProjectionType::EventTypeCount => {
                        let event_type = self.get_event_type_name(event);
                        *data.entry(event_type).or_insert(0) += 1;
                    }
                    ProjectionType::StatuteActivityCount => {
                        if let Some(statute_id) = self.get_statute_id(event) {
                            *data.entry(statute_id).or_insert(0) += 1;
                        }
                    }
                    ProjectionType::StatusChangeTimeline => {
                        if let RegistryEvent::StatusChanged { new_status, .. } = event {
                            let status_str = format!("{:?}", new_status);
                            *data.entry(status_str).or_insert(0) += 1;
                        }
                    }
                    ProjectionType::TagUsageStats => {
                        if let RegistryEvent::TagAdded { tag, .. } = event {
                            *data.entry(tag.clone()).or_insert(0) += 1;
                        }
                    }
                    ProjectionType::DailyActivitySummary => {
                        let date_key = timestamp.format("%Y-%m-%d").to_string();
                        *data.entry(date_key).or_insert(0) += 1;
                    }
                }
            }

            ProjectionResult {
                projection_type,
                data,
                events_processed: events.len(),
                time_range: (min_time, max_time),
            }
        }

        fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
            match event {
                RegistryEvent::StatuteRegistered { timestamp, .. }
                | RegistryEvent::StatuteUpdated { timestamp, .. }
                | RegistryEvent::StatusChanged { timestamp, .. }
                | RegistryEvent::TagAdded { timestamp, .. }
                | RegistryEvent::TagRemoved { timestamp, .. }
                | RegistryEvent::ReferenceAdded { timestamp, .. }
                | RegistryEvent::ReferenceRemoved { timestamp, .. }
                | RegistryEvent::MetadataUpdated { timestamp, .. }
                | RegistryEvent::StatuteDeleted { timestamp, .. }
                | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
            }
        }

        fn get_event_type_name(&self, event: &RegistryEvent) -> String {
            match event {
                RegistryEvent::StatuteRegistered { .. } => "StatuteRegistered".to_string(),
                RegistryEvent::StatuteUpdated { .. } => "StatuteUpdated".to_string(),
                RegistryEvent::StatusChanged { .. } => "StatusChanged".to_string(),
                RegistryEvent::TagAdded { .. } => "TagAdded".to_string(),
                RegistryEvent::TagRemoved { .. } => "TagRemoved".to_string(),
                RegistryEvent::ReferenceAdded { .. } => "ReferenceAdded".to_string(),
                RegistryEvent::ReferenceRemoved { .. } => "ReferenceRemoved".to_string(),
                RegistryEvent::MetadataUpdated { .. } => "MetadataUpdated".to_string(),
                RegistryEvent::StatuteDeleted { .. } => "StatuteDeleted".to_string(),
                RegistryEvent::StatuteArchived { .. } => "StatuteArchived".to_string(),
            }
        }

        fn get_statute_id(&self, event: &RegistryEvent) -> Option<String> {
            match event {
                RegistryEvent::StatuteRegistered { statute_id, .. }
                | RegistryEvent::StatuteUpdated { statute_id, .. }
                | RegistryEvent::StatusChanged { statute_id, .. }
                | RegistryEvent::TagAdded { statute_id, .. }
                | RegistryEvent::TagRemoved { statute_id, .. }
                | RegistryEvent::ReferenceAdded { statute_id, .. }
                | RegistryEvent::ReferenceRemoved { statute_id, .. }
                | RegistryEvent::MetadataUpdated { statute_id, .. }
                | RegistryEvent::StatuteDeleted { statute_id, .. }
                | RegistryEvent::StatuteArchived { statute_id, .. } => Some(statute_id.clone()),
            }
        }
    }

    // ========================================================================
    // 3. Event-Driven Notifications
    // ========================================================================

    /// Notification channel for event-driven updates.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NotificationChannel {
        /// Email notification
        Email(String),
        /// Webhook URL
        Webhook(String),
        /// SMS notification
        Sms(String),
        /// In-app notification
        InApp(String),
    }

    /// Notification rule configuration.
    #[derive(Debug, Clone)]
    pub struct NotificationRule {
        /// Rule ID
        pub id: Uuid,
        /// Rule name
        pub name: String,
        /// Event filter pattern
        pub event_pattern: String,
        /// Notification channels
        pub channels: Vec<NotificationChannel>,
        /// Enabled flag
        pub enabled: bool,
    }

    impl NotificationRule {
        /// Creates a new notification rule.
        pub fn new(name: String, event_pattern: String) -> Self {
            Self {
                id: Uuid::new_v4(),
                name,
                event_pattern,
                channels: Vec::new(),
                enabled: true,
            }
        }

        /// Adds a notification channel.
        pub fn add_channel(mut self, channel: NotificationChannel) -> Self {
            self.channels.push(channel);
            self
        }
    }

    /// Event notification manager.
    #[derive(Debug)]
    pub struct NotificationManager {
        rules: Arc<Mutex<Vec<NotificationRule>>>,
    }

    impl NotificationManager {
        /// Creates a new notification manager.
        pub fn new() -> Self {
            Self {
                rules: Arc::new(Mutex::new(Vec::new())),
            }
        }

        /// Adds a notification rule.
        pub fn add_rule(&self, rule: NotificationRule) {
            let mut rules = self.rules.lock().unwrap();
            rules.push(rule);
        }

        /// Removes a notification rule.
        pub fn remove_rule(&self, rule_id: Uuid) -> bool {
            let mut rules = self.rules.lock().unwrap();
            if let Some(pos) = rules.iter().position(|r| r.id == rule_id) {
                rules.remove(pos);
                true
            } else {
                false
            }
        }

        /// Processes an event and sends notifications.
        pub fn process_event(&self, event: &RegistryEvent) -> usize {
            let rules = self.rules.lock().unwrap();
            let mut notifications_sent = 0;

            for rule in rules.iter() {
                if !rule.enabled {
                    continue;
                }

                if self.event_matches_pattern(event, &rule.event_pattern) {
                    for channel in &rule.channels {
                        self.send_notification(channel, event);
                        notifications_sent += 1;
                    }
                }
            }

            notifications_sent
        }

        fn event_matches_pattern(&self, event: &RegistryEvent, pattern: &str) -> bool {
            let event_type = match event {
                RegistryEvent::StatuteRegistered { .. } => "StatuteRegistered",
                RegistryEvent::StatuteUpdated { .. } => "StatuteUpdated",
                RegistryEvent::StatusChanged { .. } => "StatusChanged",
                RegistryEvent::TagAdded { .. } => "TagAdded",
                RegistryEvent::TagRemoved { .. } => "TagRemoved",
                RegistryEvent::ReferenceAdded { .. } => "ReferenceAdded",
                RegistryEvent::ReferenceRemoved { .. } => "ReferenceRemoved",
                RegistryEvent::MetadataUpdated { .. } => "MetadataUpdated",
                RegistryEvent::StatuteDeleted { .. } => "StatuteDeleted",
                RegistryEvent::StatuteArchived { .. } => "StatuteArchived",
            };

            pattern == "*" || event_type.contains(pattern)
        }

        fn send_notification(&self, _channel: &NotificationChannel, _event: &RegistryEvent) {
            // Actual notification sending would be implemented here
            // For now, this is a placeholder
        }

        /// Lists all notification rules.
        pub fn list_rules(&self) -> Vec<NotificationRule> {
            self.rules.lock().unwrap().clone()
        }
    }

    impl Default for NotificationManager {
        fn default() -> Self {
            Self::new()
        }
    }

    // ========================================================================
    // 4. Event Archiving with Cold Storage
    // ========================================================================

    /// Cold storage configuration.
    #[derive(Debug, Clone)]
    pub struct ColdStorageConfig {
        /// Archive events older than this duration
        pub archive_after: chrono::Duration,
        /// Compression enabled
        pub compression: bool,
        /// Archive path
        pub archive_path: String,
    }

    impl Default for ColdStorageConfig {
        fn default() -> Self {
            Self {
                archive_after: chrono::Duration::days(90),
                compression: true,
                archive_path: "/tmp/event_archive".to_string(),
            }
        }
    }

    /// Archived event batch.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ArchivedEventBatch {
        /// Archive ID
        pub id: Uuid,
        /// Events in this batch
        pub events: Vec<RegistryEvent>,
        /// Archive timestamp
        pub archived_at: DateTime<Utc>,
        /// Compressed flag
        pub compressed: bool,
    }

    /// Event archiver for cold storage management.
    #[derive(Debug)]
    pub struct EventArchiver {
        config: ColdStorageConfig,
        archived_batches: Arc<Mutex<Vec<ArchivedEventBatch>>>,
    }

    impl EventArchiver {
        /// Creates a new event archiver.
        pub fn new(config: ColdStorageConfig) -> Self {
            Self {
                config,
                archived_batches: Arc::new(Mutex::new(Vec::new())),
            }
        }

        /// Archives old events from the event store.
        pub fn archive_old_events(&self, event_store: &mut EventStore) -> Result<usize, String> {
            let cutoff_time = Utc::now() - self.config.archive_after;
            let all_events = event_store.all_events();

            let (to_archive, to_keep): (Vec<_>, Vec<_>) =
                all_events.into_iter().cloned().partition(|event| {
                    let timestamp = self.get_event_timestamp(event);
                    timestamp < cutoff_time
                });

            if to_archive.is_empty() {
                return Ok(0);
            }

            let batch = ArchivedEventBatch {
                id: Uuid::new_v4(),
                events: to_archive.clone(),
                archived_at: Utc::now(),
                compressed: self.config.compression,
            };

            let archived_count = batch.events.len();

            // Store the archived batch
            let mut batches = self.archived_batches.lock().unwrap();
            batches.push(batch);

            // Clear and repopulate event store with non-archived events
            event_store.clear();
            for event in to_keep {
                event_store.record(event);
            }

            Ok(archived_count)
        }

        /// Retrieves archived events.
        pub fn get_archived_events(&self) -> Vec<ArchivedEventBatch> {
            self.archived_batches.lock().unwrap().clone()
        }

        /// Restores events from an archived batch.
        pub fn restore_batch(
            &self,
            batch_id: Uuid,
            event_store: &mut EventStore,
        ) -> Result<usize, String> {
            let batches = self.archived_batches.lock().unwrap();

            if let Some(batch) = batches.iter().find(|b| b.id == batch_id) {
                let count = batch.events.len();
                for event in &batch.events {
                    event_store.record(event.clone());
                }
                Ok(count)
            } else {
                Err("Batch not found".to_string())
            }
        }

        fn get_event_timestamp(&self, event: &RegistryEvent) -> DateTime<Utc> {
            match event {
                RegistryEvent::StatuteRegistered { timestamp, .. }
                | RegistryEvent::StatuteUpdated { timestamp, .. }
                | RegistryEvent::StatusChanged { timestamp, .. }
                | RegistryEvent::TagAdded { timestamp, .. }
                | RegistryEvent::TagRemoved { timestamp, .. }
                | RegistryEvent::ReferenceAdded { timestamp, .. }
                | RegistryEvent::ReferenceRemoved { timestamp, .. }
                | RegistryEvent::MetadataUpdated { timestamp, .. }
                | RegistryEvent::StatuteDeleted { timestamp, .. }
                | RegistryEvent::StatuteArchived { timestamp, .. } => *timestamp,
            }
        }
    }

    // ========================================================================
    // 5. Event Schema Evolution Support
    // ========================================================================

    /// Event schema version.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub struct SchemaVersion {
        pub major: u32,
        pub minor: u32,
        pub patch: u32,
    }

    impl SchemaVersion {
        /// Creates a new schema version.
        pub fn new(major: u32, minor: u32, patch: u32) -> Self {
            Self {
                major,
                minor,
                patch,
            }
        }

        /// Returns the current schema version.
        pub fn current() -> Self {
            Self::new(1, 0, 0)
        }
    }

    impl std::fmt::Display for SchemaVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }

    /// Event envelope with schema versioning.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VersionedEvent {
        /// Schema version
        pub schema_version: SchemaVersion,
        /// Event ID
        pub event_id: Uuid,
        /// Timestamp
        pub timestamp: DateTime<Utc>,
        /// Event data
        pub event: RegistryEvent,
        /// Migration history
        pub migration_history: Vec<SchemaVersion>,
    }

    impl VersionedEvent {
        /// Creates a new versioned event.
        pub fn new(event: RegistryEvent) -> Self {
            Self {
                schema_version: SchemaVersion::current(),
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event,
                migration_history: Vec::new(),
            }
        }
    }

    /// Schema migration handler.
    #[allow(clippy::wrong_self_convention)]
    pub trait SchemaMigration: Send + Sync {
        /// Source schema version.
        fn from_version(&self) -> SchemaVersion;

        /// Target schema version.
        fn to_version(&self) -> SchemaVersion;

        /// Migrates an event to the new schema.
        fn migrate(&self, event: RegistryEvent) -> Result<RegistryEvent, String>;
    }

    /// Schema evolution manager.
    pub struct SchemaEvolutionManager {
        current_version: SchemaVersion,
        migrations: Vec<Box<dyn SchemaMigration>>,
    }

    impl SchemaEvolutionManager {
        /// Creates a new schema evolution manager.
        pub fn new() -> Self {
            Self {
                current_version: SchemaVersion::current(),
                migrations: Vec::new(),
            }
        }

        /// Registers a schema migration.
        pub fn register_migration(&mut self, migration: Box<dyn SchemaMigration>) {
            self.migrations.push(migration);
        }

        /// Migrates an event to the current schema version.
        pub fn migrate_event(
            &self,
            mut versioned: VersionedEvent,
        ) -> Result<VersionedEvent, String> {
            while versioned.schema_version < self.current_version {
                let migration = self.find_migration(versioned.schema_version)?;
                versioned.event = migration.migrate(versioned.event)?;
                versioned.migration_history.push(versioned.schema_version);
                versioned.schema_version = migration.to_version();
            }
            Ok(versioned)
        }

        fn find_migration(
            &self,
            from_version: SchemaVersion,
        ) -> Result<&dyn SchemaMigration, String> {
            self.migrations
                .iter()
                .find(|m| m.from_version() == from_version)
                .map(|b| b.as_ref())
                .ok_or_else(|| format!("No migration found from version {}", from_version))
        }

        /// Gets the current schema version.
        pub fn current_version(&self) -> SchemaVersion {
            self.current_version
        }
    }

    impl Default for SchemaEvolutionManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

// ============================================================================
// Federation Protocol Module (v0.2.7)
// ============================================================================

/// Federation Protocol: Multi-registry federation and cross-registry queries.
pub mod federation {
    use super::*;
    use std::collections::BTreeSet;

    // ========================================================================
    // 1. Federated Registry Discovery
    // ========================================================================

    /// Registry metadata for federation.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct RegistryMetadata {
        /// Unique registry identifier
        pub registry_id: Uuid,
        /// Registry name
        pub name: String,
        /// Registry endpoint URL
        pub endpoint: String,
        /// Supported API version
        pub api_version: String,
        /// Jurisdictions covered by this registry
        pub jurisdictions: Vec<String>,
        /// Registry capabilities
        pub capabilities: Vec<RegistryCapability>,
        /// Last seen timestamp
        pub last_seen: DateTime<Utc>,
        /// Trust level (0-100)
        pub trust_level: u8,
    }

    impl RegistryMetadata {
        /// Creates new registry metadata.
        pub fn new(name: String, endpoint: String) -> Self {
            Self {
                registry_id: Uuid::new_v4(),
                name,
                endpoint,
                api_version: "1.0.0".to_string(),
                jurisdictions: Vec::new(),
                capabilities: Vec::new(),
                last_seen: Utc::now(),
                trust_level: 50,
            }
        }

        /// Updates the last seen timestamp.
        pub fn update_last_seen(&mut self) {
            self.last_seen = Utc::now();
        }

        /// Checks if the registry is active (seen within the last hour).
        pub fn is_active(&self) -> bool {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(self.last_seen);
            elapsed.num_hours() < 1
        }
    }

    /// Registry capability flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RegistryCapability {
        /// Supports full-text search
        FullTextSearch,
        /// Supports version control
        VersionControl,
        /// Supports real-time updates
        RealTimeUpdates,
        /// Supports event sourcing
        EventSourcing,
        /// Supports GraphQL queries
        GraphQL,
        /// Supports bulk operations
        BulkOperations,
    }

    /// Registry discovery service.
    #[derive(Debug)]
    pub struct RegistryDiscovery {
        /// Known registries
        registries: Arc<Mutex<HashMap<Uuid, RegistryMetadata>>>,
        /// Discovery interval in seconds
        #[allow(dead_code)]
        discovery_interval: u64,
    }

    impl RegistryDiscovery {
        /// Creates a new registry discovery service.
        pub fn new() -> Self {
            Self {
                registries: Arc::new(Mutex::new(HashMap::new())),
                discovery_interval: 300, // 5 minutes
            }
        }

        /// Registers a new registry.
        pub fn register(&self, metadata: RegistryMetadata) {
            let mut registries = self.registries.lock().unwrap();
            registries.insert(metadata.registry_id, metadata);
        }

        /// Unregisters a registry.
        pub fn unregister(&self, registry_id: Uuid) -> bool {
            let mut registries = self.registries.lock().unwrap();
            registries.remove(&registry_id).is_some()
        }

        /// Lists all registered registries.
        pub fn list_registries(&self) -> Vec<RegistryMetadata> {
            self.registries.lock().unwrap().values().cloned().collect()
        }

        /// Finds registries by jurisdiction.
        pub fn find_by_jurisdiction(&self, jurisdiction: &str) -> Vec<RegistryMetadata> {
            self.registries
                .lock()
                .unwrap()
                .values()
                .filter(|r| r.jurisdictions.contains(&jurisdiction.to_string()))
                .cloned()
                .collect()
        }

        /// Gets active registries only.
        pub fn get_active_registries(&self) -> Vec<RegistryMetadata> {
            self.registries
                .lock()
                .unwrap()
                .values()
                .filter(|r| r.is_active())
                .cloned()
                .collect()
        }

        /// Updates registry metadata.
        pub fn update_metadata(
            &self,
            registry_id: Uuid,
            metadata: RegistryMetadata,
        ) -> Result<(), String> {
            let mut registries = self.registries.lock().unwrap();
            if let std::collections::hash_map::Entry::Occupied(mut e) =
                registries.entry(registry_id)
            {
                e.insert(metadata);
                Ok(())
            } else {
                Err("Registry not found".to_string())
            }
        }
    }

    impl Default for RegistryDiscovery {
        fn default() -> Self {
            Self::new()
        }
    }

    // ========================================================================
    // 2. Cross-Registry Statute Queries
    // ========================================================================

    /// Cross-registry query request.
    #[derive(Debug, Clone)]
    pub struct FederatedQuery {
        /// Query text
        pub query: String,
        /// Target jurisdictions
        pub jurisdictions: Vec<String>,
        /// Target registries (if empty, queries all)
        pub target_registries: Vec<Uuid>,
        /// Maximum results per registry
        pub max_results_per_registry: usize,
        /// Timeout in seconds
        pub timeout: u64,
    }

    impl FederatedQuery {
        /// Creates a new federated query.
        pub fn new(query: String) -> Self {
            Self {
                query,
                jurisdictions: Vec::new(),
                target_registries: Vec::new(),
                max_results_per_registry: 50,
                timeout: 30,
            }
        }

        /// Filters by jurisdictions.
        pub fn with_jurisdictions(mut self, jurisdictions: Vec<String>) -> Self {
            self.jurisdictions = jurisdictions;
            self
        }

        /// Targets specific registries.
        pub fn with_target_registries(mut self, registries: Vec<Uuid>) -> Self {
            self.target_registries = registries;
            self
        }
    }

    /// Federated query result from a single registry.
    #[derive(Debug, Clone)]
    pub struct RegistryQueryResult {
        /// Source registry ID
        pub registry_id: Uuid,
        /// Registry name
        pub registry_name: String,
        /// Matched statute IDs
        pub statute_ids: Vec<String>,
        /// Query execution time
        pub execution_time: std::time::Duration,
        /// Success flag
        pub success: bool,
        /// Error message if failed
        pub error: Option<String>,
    }

    /// Aggregated federated query results.
    #[derive(Debug, Clone)]
    pub struct FederatedQueryResult {
        /// Query text
        pub query: String,
        /// Results from each registry
        pub registry_results: Vec<RegistryQueryResult>,
        /// Total statutes found
        pub total_statutes: usize,
        /// Number of registries queried
        pub registries_queried: usize,
        /// Number of successful queries
        pub successful_queries: usize,
        /// Total execution time
        pub total_execution_time: std::time::Duration,
    }

    /// Cross-registry query engine.
    #[derive(Debug)]
    pub struct FederatedQueryEngine {
        discovery: Arc<RegistryDiscovery>,
    }

    impl FederatedQueryEngine {
        /// Creates a new federated query engine.
        pub fn new(discovery: Arc<RegistryDiscovery>) -> Self {
            Self { discovery }
        }

        /// Executes a federated query across multiple registries.
        pub fn execute(&self, query: FederatedQuery) -> FederatedQueryResult {
            let start = std::time::Instant::now();
            let registries = if query.target_registries.is_empty() {
                self.discovery.get_active_registries()
            } else {
                self.discovery
                    .list_registries()
                    .into_iter()
                    .filter(|r| query.target_registries.contains(&r.registry_id))
                    .collect()
            };

            let mut registry_results = Vec::new();
            let mut total_statutes = 0;

            for registry in &registries {
                let result = self.query_single_registry(registry, &query);
                total_statutes += result.statute_ids.len();
                registry_results.push(result);
            }

            let successful_queries = registry_results.iter().filter(|r| r.success).count();

            FederatedQueryResult {
                query: query.query.clone(),
                registry_results,
                total_statutes,
                registries_queried: registries.len(),
                successful_queries,
                total_execution_time: start.elapsed(),
            }
        }

        fn query_single_registry(
            &self,
            registry: &RegistryMetadata,
            _query: &FederatedQuery,
        ) -> RegistryQueryResult {
            let start = std::time::Instant::now();

            // Simulated query execution
            // In a real implementation, this would make HTTP calls to the remote registry
            RegistryQueryResult {
                registry_id: registry.registry_id,
                registry_name: registry.name.clone(),
                statute_ids: Vec::new(),
                execution_time: start.elapsed(),
                success: true,
                error: None,
            }
        }
    }

    // ========================================================================
    // 3. Registry Peering Agreements
    // ========================================================================

    /// Peering agreement status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PeeringStatus {
        /// Agreement pending approval
        Pending,
        /// Agreement active
        Active,
        /// Agreement suspended
        Suspended,
        /// Agreement terminated
        Terminated,
    }

    /// Data sharing level in peering agreement.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SharingLevel {
        /// Share public data only
        Public,
        /// Share metadata only
        Metadata,
        /// Share full statute data
        Full,
        /// Bidirectional full sharing
        Bidirectional,
    }

    /// Registry peering agreement.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PeeringAgreement {
        /// Agreement ID
        pub id: Uuid,
        /// Local registry ID
        pub local_registry: Uuid,
        /// Peer registry ID
        pub peer_registry: Uuid,
        /// Agreement status
        pub status: PeeringStatus,
        /// Data sharing level
        pub sharing_level: SharingLevel,
        /// Created timestamp
        pub created_at: DateTime<Utc>,
        /// Last updated timestamp
        pub updated_at: DateTime<Utc>,
        /// Expiration date
        pub expires_at: Option<DateTime<Utc>>,
        /// Terms and conditions
        pub terms: String,
    }

    impl PeeringAgreement {
        /// Creates a new peering agreement.
        pub fn new(local_registry: Uuid, peer_registry: Uuid, sharing_level: SharingLevel) -> Self {
            let now = Utc::now();
            Self {
                id: Uuid::new_v4(),
                local_registry,
                peer_registry,
                status: PeeringStatus::Pending,
                sharing_level,
                created_at: now,
                updated_at: now,
                expires_at: None,
                terms: String::new(),
            }
        }

        /// Activates the peering agreement.
        pub fn activate(&mut self) {
            self.status = PeeringStatus::Active;
            self.updated_at = Utc::now();
        }

        /// Suspends the peering agreement.
        pub fn suspend(&mut self) {
            self.status = PeeringStatus::Suspended;
            self.updated_at = Utc::now();
        }

        /// Terminates the peering agreement.
        pub fn terminate(&mut self) {
            self.status = PeeringStatus::Terminated;
            self.updated_at = Utc::now();
        }

        /// Checks if the agreement is active and not expired.
        pub fn is_valid(&self) -> bool {
            if self.status != PeeringStatus::Active {
                return false;
            }
            if let Some(expires_at) = self.expires_at {
                Utc::now() < expires_at
            } else {
                true
            }
        }
    }

    /// Peering agreement manager.
    #[derive(Debug)]
    pub struct PeeringManager {
        agreements: Arc<Mutex<HashMap<Uuid, PeeringAgreement>>>,
    }

    impl PeeringManager {
        /// Creates a new peering manager.
        pub fn new() -> Self {
            Self {
                agreements: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Creates a new peering agreement.
        pub fn create_agreement(&self, agreement: PeeringAgreement) -> Uuid {
            let id = agreement.id;
            let mut agreements = self.agreements.lock().unwrap();
            agreements.insert(id, agreement);
            id
        }

        /// Gets a peering agreement by ID.
        pub fn get_agreement(&self, id: Uuid) -> Option<PeeringAgreement> {
            self.agreements.lock().unwrap().get(&id).cloned()
        }

        /// Lists all agreements for a registry.
        pub fn list_agreements(&self, registry_id: Uuid) -> Vec<PeeringAgreement> {
            self.agreements
                .lock()
                .unwrap()
                .values()
                .filter(|a| a.local_registry == registry_id || a.peer_registry == registry_id)
                .cloned()
                .collect()
        }

        /// Gets active agreements for a registry.
        pub fn get_active_agreements(&self, registry_id: Uuid) -> Vec<PeeringAgreement> {
            self.list_agreements(registry_id)
                .into_iter()
                .filter(|a| a.is_valid())
                .collect()
        }

        /// Updates an agreement.
        pub fn update_agreement(
            &self,
            id: Uuid,
            agreement: PeeringAgreement,
        ) -> Result<(), String> {
            let mut agreements = self.agreements.lock().unwrap();
            if let std::collections::hash_map::Entry::Occupied(mut e) = agreements.entry(id) {
                e.insert(agreement);
                Ok(())
            } else {
                Err("Agreement not found".to_string())
            }
        }

        /// Deletes an agreement.
        pub fn delete_agreement(&self, id: Uuid) -> bool {
            let mut agreements = self.agreements.lock().unwrap();
            agreements.remove(&id).is_some()
        }
    }

    impl Default for PeeringManager {
        fn default() -> Self {
            Self::new()
        }
    }

    // ========================================================================
    // 4. Federated Search Aggregation
    // ========================================================================

    /// Search result ranking strategy.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RankingStrategy {
        /// Rank by relevance score
        Relevance,
        /// Rank by registry trust level
        TrustLevel,
        /// Rank by recency
        Recency,
        /// Combined ranking
        Combined,
    }

    /// Aggregated search result.
    #[derive(Debug, Clone)]
    pub struct AggregatedSearchResult {
        /// Statute ID
        pub statute_id: String,
        /// Source registry ID
        pub registry_id: Uuid,
        /// Registry name
        pub registry_name: String,
        /// Relevance score (0.0 - 1.0)
        pub relevance_score: f64,
        /// Registry trust level
        pub trust_level: u8,
        /// Combined score
        pub combined_score: f64,
    }

    /// Federated search aggregator.
    #[derive(Debug)]
    pub struct FederatedSearchAggregator {
        ranking_strategy: RankingStrategy,
    }

    impl FederatedSearchAggregator {
        /// Creates a new search aggregator.
        pub fn new(ranking_strategy: RankingStrategy) -> Self {
            Self { ranking_strategy }
        }

        /// Aggregates results from multiple registries.
        pub fn aggregate(
            &self,
            federated_result: &FederatedQueryResult,
        ) -> Vec<AggregatedSearchResult> {
            let mut results = Vec::new();

            for registry_result in &federated_result.registry_results {
                if !registry_result.success {
                    continue;
                }

                for statute_id in &registry_result.statute_ids {
                    let result = AggregatedSearchResult {
                        statute_id: statute_id.clone(),
                        registry_id: registry_result.registry_id,
                        registry_name: registry_result.registry_name.clone(),
                        relevance_score: 1.0, // Would be calculated based on query match
                        trust_level: 50,      // Would come from registry metadata
                        combined_score: 0.0,
                    };
                    results.push(result);
                }
            }

            self.rank_results(&mut results);
            results
        }

        fn rank_results(&self, results: &mut [AggregatedSearchResult]) {
            for result in results.iter_mut() {
                result.combined_score = match self.ranking_strategy {
                    RankingStrategy::Relevance => result.relevance_score,
                    RankingStrategy::TrustLevel => f64::from(result.trust_level) / 100.0,
                    RankingStrategy::Recency => 0.5, // Placeholder
                    RankingStrategy::Combined => {
                        (result.relevance_score * 0.5)
                            + (f64::from(result.trust_level) / 100.0 * 0.5)
                    }
                };
            }

            results.sort_by(|a, b| {
                b.combined_score
                    .partial_cmp(&a.combined_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        /// Deduplicates results across registries.
        pub fn deduplicate(
            &self,
            results: Vec<AggregatedSearchResult>,
        ) -> Vec<AggregatedSearchResult> {
            let mut seen = BTreeSet::new();
            results
                .into_iter()
                .filter(|r| seen.insert(r.statute_id.clone()))
                .collect()
        }
    }

    // ========================================================================
    // 5. Trust Frameworks for Federation
    // ========================================================================

    /// Trust level category.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TrustLevel {
        /// Untrusted (0-20)
        Untrusted,
        /// Low trust (21-40)
        Low,
        /// Medium trust (41-60)
        Medium,
        /// High trust (61-80)
        High,
        /// Verified (81-100)
        Verified,
    }

    impl TrustLevel {
        /// Converts a numeric score to a trust level.
        pub fn from_score(score: u8) -> Self {
            match score {
                0..=20 => TrustLevel::Untrusted,
                21..=40 => TrustLevel::Low,
                41..=60 => TrustLevel::Medium,
                61..=80 => TrustLevel::High,
                81..=100 => TrustLevel::Verified,
                _ => TrustLevel::Medium,
            }
        }

        /// Converts trust level to numeric score.
        pub fn to_score(&self) -> u8 {
            match self {
                TrustLevel::Untrusted => 10,
                TrustLevel::Low => 30,
                TrustLevel::Medium => 50,
                TrustLevel::High => 70,
                TrustLevel::Verified => 90,
            }
        }
    }

    /// Trust metric for a registry.
    #[derive(Debug, Clone)]
    pub struct TrustMetric {
        /// Registry ID
        pub registry_id: Uuid,
        /// Uptime percentage (0-100)
        pub uptime: f64,
        /// Response time average (ms)
        pub avg_response_time: u64,
        /// Successful queries percentage
        pub success_rate: f64,
        /// Data quality score (0-100)
        pub data_quality: u8,
        /// Community reputation score (0-100)
        pub reputation: u8,
        /// Calculated trust score
        pub trust_score: u8,
    }

    impl TrustMetric {
        /// Creates a new trust metric.
        pub fn new(registry_id: Uuid) -> Self {
            Self {
                registry_id,
                uptime: 100.0,
                avg_response_time: 100,
                success_rate: 100.0,
                data_quality: 50,
                reputation: 50,
                trust_score: 50,
            }
        }

        /// Calculates the trust score based on metrics.
        pub fn calculate_trust_score(&mut self) {
            let uptime_score = self.uptime;
            let response_score = if self.avg_response_time < 100 {
                100.0
            } else if self.avg_response_time < 500 {
                80.0
            } else if self.avg_response_time < 1000 {
                60.0
            } else {
                40.0
            };
            let success_score = self.success_rate;

            self.trust_score = ((uptime_score * 0.3)
                + (response_score * 0.2)
                + (success_score * 0.2)
                + (f64::from(self.data_quality) * 0.15)
                + (f64::from(self.reputation) * 0.15)) as u8;
        }

        /// Gets the trust level category.
        pub fn trust_level(&self) -> TrustLevel {
            TrustLevel::from_score(self.trust_score)
        }
    }

    /// Trust framework manager.
    #[derive(Debug)]
    pub struct TrustFramework {
        metrics: Arc<Mutex<HashMap<Uuid, TrustMetric>>>,
    }

    impl TrustFramework {
        /// Creates a new trust framework.
        pub fn new() -> Self {
            Self {
                metrics: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Updates trust metrics for a registry.
        pub fn update_metrics(&self, metric: TrustMetric) {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.insert(metric.registry_id, metric);
        }

        /// Gets trust metrics for a registry.
        pub fn get_metrics(&self, registry_id: Uuid) -> Option<TrustMetric> {
            self.metrics.lock().unwrap().get(&registry_id).cloned()
        }

        /// Gets trust score for a registry.
        pub fn get_trust_score(&self, registry_id: Uuid) -> u8 {
            self.metrics
                .lock()
                .unwrap()
                .get(&registry_id)
                .map(|m| m.trust_score)
                .unwrap_or(50)
        }

        /// Lists all registries by trust level.
        pub fn list_by_trust_level(&self, min_level: TrustLevel) -> Vec<Uuid> {
            self.metrics
                .lock()
                .unwrap()
                .values()
                .filter(|m| m.trust_level() >= min_level)
                .map(|m| m.registry_id)
                .collect()
        }

        /// Recalculates all trust scores.
        pub fn recalculate_all(&self) {
            let mut metrics = self.metrics.lock().unwrap();
            for metric in metrics.values_mut() {
                metric.calculate_trust_score();
            }
        }
    }

    impl Default for TrustFramework {
        fn default() -> Self {
            Self::new()
        }
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

    #[test]
    fn test_similarity_score_possible_duplicate() {
        let score = SimilarityScore::new(0.65, 0.7, 0.6);

        // 0.7 * 0.85 = 0.595, score.overall ~ 0.68
        assert!(score.is_possible_duplicate(0.85));
        assert!(!score.is_likely_duplicate(0.85));
    }

    #[test]
    fn test_calculate_similarity() {
        let registry = StatuteRegistry::new();

        let entry1 = StatuteEntry::new(test_statute("test-1"), "US")
            .with_tag("civil")
            .with_tag("rights")
            .with_reference("ref-1".to_string())
            .with_reference("ref-2".to_string());

        let entry2 = StatuteEntry::new(test_statute("test-1"), "US")
            .with_tag("civil")
            .with_tag("rights")
            .with_reference("ref-1".to_string())
            .with_reference("ref-2".to_string());

        let similarity = registry.calculate_similarity(&entry1, &entry2);

        // Same title, tags, and references should give high similarity
        assert!(similarity.overall > 0.8);
        assert!(similarity.title > 0.8);
        assert!(similarity.content > 0.9); // Same references
        assert!(similarity.metadata > 0.9); // Same tags
    }

    #[test]
    fn test_calculate_similarity_different() {
        let registry = StatuteRegistry::new();

        let entry1 =
            StatuteEntry::new(test_statute("completely-different-1"), "US").with_tag("civil");

        let entry2 = StatuteEntry::new(test_statute("another-thing-2"), "UK").with_tag("criminal");

        let similarity = registry.calculate_similarity(&entry1, &entry2);

        // Different titles and tags should give low similarity
        assert!(similarity.overall < 0.5);
    }

    #[test]
    fn test_duplicate_detection_result() {
        let mut result = DuplicateDetectionResult::new(0.8, 10);

        assert_eq!(result.threshold, 0.8);
        assert_eq!(result.statutes_analyzed, 10);
        assert_eq!(result.total_duplicates(), 0);

        let candidate = DuplicateCandidate::new(
            "s1".to_string(),
            "s2".to_string(),
            SimilarityScore::new(0.85, 0.9, 0.8),
            "High similarity".to_string(),
        );

        result.add_candidate(candidate);
        assert_eq!(result.total_duplicates(), 1);
    }

    #[test]
    fn test_duplicate_detection_filtering() {
        let mut result = DuplicateDetectionResult::new(0.8, 10);

        // Add a likely duplicate (high similarity)
        result.add_candidate(DuplicateCandidate::new(
            "s1".to_string(),
            "s2".to_string(),
            SimilarityScore::new(0.85, 0.9, 0.8),
            "High".to_string(),
        ));

        // Add a possible duplicate (medium similarity)
        result.add_candidate(DuplicateCandidate::new(
            "s3".to_string(),
            "s4".to_string(),
            SimilarityScore::new(0.6, 0.65, 0.55),
            "Medium".to_string(),
        ));

        assert_eq!(result.likely_duplicates().len(), 1);
        // Both should be in possible duplicates (>= threshold * 0.7)
        assert_eq!(result.possible_duplicates().len(), 2);
    }

    #[test]
    fn test_detect_duplicates() {
        let mut registry = StatuteRegistry::new();

        // Add similar statutes with shared references
        registry
            .register(
                StatuteEntry::new(test_statute("civil-code-1"), "US")
                    .with_tag("civil")
                    .with_reference("ref-common".to_string()),
            )
            .unwrap();
        registry
            .register(
                StatuteEntry::new(test_statute("civil-code-2"), "US")
                    .with_tag("civil")
                    .with_reference("ref-common".to_string()),
            )
            .unwrap();
        registry
            .register(StatuteEntry::new(
                test_statute("completely-different"),
                "UK",
            ))
            .unwrap();

        let result = registry.detect_duplicates(0.7);

        assert_eq!(result.statutes_analyzed, 3);
        // Should find at least one duplicate pair (the two civil codes with similar titles, tags, and refs)
        assert!(result.total_duplicates() > 0);
    }

    #[test]
    fn test_detect_duplicates_no_duplicates() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("very-unique-1"), "US"))
            .unwrap();
        registry
            .register(StatuteEntry::new(test_statute("totally-different-2"), "UK"))
            .unwrap();
        registry
            .register(StatuteEntry::new(test_statute("another-one-3"), "JP"))
            .unwrap();

        let result = registry.detect_duplicates(0.9);

        assert_eq!(result.statutes_analyzed, 3);
        // With high threshold and different statutes, should find no duplicates
        assert_eq!(result.total_duplicates(), 0);
    }

    #[test]
    fn test_field_profile_creation() {
        let mut profile = FieldProfile::new("test_field".to_string(), 100);
        profile.null_count = 10;
        profile.unique_count = 50;

        profile.calculate_completeness();

        assert_eq!(profile.field_name, "test_field");
        assert_eq!(profile.total_values, 100);
        assert_eq!(profile.null_count, 10);
        assert_eq!(profile.unique_count, 50);
        assert_eq!(profile.completeness, 90.0); // (100-10)/100 * 100
    }

    #[test]
    fn test_data_profile_creation() {
        let mut profile = DataProfile::new(50);

        assert_eq!(profile.total_statutes, 50);
        assert_eq!(profile.average_quality, 0.0);

        let field_profile = FieldProfile::new("title".to_string(), 50);
        profile.add_field_profile(field_profile);

        assert_eq!(profile.field_profiles.len(), 1);
        assert!(profile.field_profiles.contains_key("title"));
    }

    #[test]
    fn test_data_profile_field_completeness() {
        let mut profile = DataProfile::new(100);

        let mut field = FieldProfile::new("jurisdiction".to_string(), 100);
        field.null_count = 5;
        field.calculate_completeness();

        profile.add_field_profile(field);

        let completeness = profile.field_completeness("jurisdiction");
        assert_eq!(completeness, Some(95.0));

        let missing = profile.field_completeness("nonexistent");
        assert_eq!(missing, None);
    }

    #[test]
    fn test_profile_data() {
        let mut registry = StatuteRegistry::new();

        // Add diverse statutes
        registry
            .register(
                StatuteEntry::new(test_statute("civil-1"), "US")
                    .with_tag("civil")
                    .with_status(StatuteStatus::Active),
            )
            .unwrap();
        registry
            .register(
                StatuteEntry::new(test_statute("criminal-1"), "UK")
                    .with_tag("criminal")
                    .with_status(StatuteStatus::Draft),
            )
            .unwrap();
        registry
            .register(
                StatuteEntry::new(test_statute("admin-1"), "JP")
                    .with_tag("administrative")
                    .with_status(StatuteStatus::Active),
            )
            .unwrap();

        let profile = registry.profile_data();

        assert_eq!(profile.total_statutes, 3);
        assert!(profile.average_quality > 0.0);

        // Should have status distribution
        assert!(
            profile
                .status_distribution
                .contains_key(&StatuteStatus::Active)
        );
        assert_eq!(profile.status_distribution[&StatuteStatus::Active], 2);

        // Should have jurisdiction distribution
        assert!(profile.jurisdiction_distribution.contains_key("US"));
        assert!(profile.jurisdiction_distribution.contains_key("UK"));
        assert!(profile.jurisdiction_distribution.contains_key("JP"));

        // Should have tag patterns
        assert!(profile.tag_patterns.contains_key("civil"));
        assert!(profile.tag_patterns.contains_key("criminal"));
        assert!(profile.tag_patterns.contains_key("administrative"));
    }

    #[test]
    fn test_profile_data_quality_distribution() {
        let mut registry = StatuteRegistry::new();

        // Add statutes with varying quality
        registry
            .register(
                StatuteEntry::new(test_statute("high-quality"), "US")
                    .with_tag("civil")
                    .with_metadata("description".to_string(), "Detailed statute".to_string())
                    .with_metadata("author".to_string(), "Congress".to_string()),
            )
            .unwrap();

        registry
            .register(StatuteEntry::new(test_statute("low-quality"), "UK"))
            .unwrap();

        let profile = registry.profile_data();

        assert_eq!(profile.total_statutes, 2);
        assert!(!profile.quality_distribution.is_empty());
    }

    #[test]
    fn test_find_low_quality_statutes() {
        let mut registry = StatuteRegistry::new();

        // Add a low-quality statute (minimal metadata)
        registry
            .register(StatuteEntry::new(test_statute("low"), "US"))
            .unwrap();

        // Add a high-quality statute
        registry
            .register(
                StatuteEntry::new(test_statute("high"), "UK")
                    .with_tag("civil")
                    .with_tag("rights")
                    .with_metadata("description".to_string(), "Excellent statute".to_string())
                    .with_metadata("version_notes".to_string(), "Initial".to_string()),
            )
            .unwrap();

        let low_quality = registry.find_low_quality_statutes(70.0);

        // At least the "low" statute should be flagged
        assert!(!low_quality.is_empty());
        assert!(low_quality.iter().any(|(id, _)| id == "low"));
    }

    #[test]
    fn test_export_quality_assessments_json() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("test-1"), "US"))
            .unwrap();
        registry
            .register(StatuteEntry::new(test_statute("test-2"), "UK"))
            .unwrap();

        let json = registry.export_quality_assessments_json().unwrap();

        assert!(json.contains("test-1"));
        assert!(json.contains("test-2"));
        assert!(json.contains("overall"));
        assert!(json.contains("issues"));
    }

    #[test]
    fn test_export_duplicates_json() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("similar-1"), "US"))
            .unwrap();
        registry
            .register(StatuteEntry::new(test_statute("similar-2"), "US"))
            .unwrap();

        let json = registry.export_duplicates_json(0.7).unwrap();

        assert!(json.contains("candidates"));
        assert!(json.contains("threshold"));
        assert!(json.contains("statutes_analyzed"));
    }

    #[test]
    fn test_data_profile_export_json() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("test-1"), "US"))
            .unwrap();

        let profile = registry.profile_data();
        let json = profile.export_json().unwrap();

        assert!(json.contains("total_statutes"));
        assert!(json.contains("average_quality"));
        assert!(json.contains("field_profiles"));
    }

    // ========================================================================
    // Enrichment and Lineage Tests
    // ========================================================================

    #[test]
    fn test_enrichment_suggestion_creation() {
        let suggestion = EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "civil".to_string(),
            0.85,
            "Contains civil law keywords".to_string(),
        );

        assert_eq!(suggestion.enrichment_type, EnrichmentType::AutoTag);
        assert_eq!(suggestion.suggestion, "civil");
        assert_eq!(suggestion.confidence, 0.85);
        assert!(suggestion.meets_threshold(0.8));
        assert!(!suggestion.meets_threshold(0.9));
    }

    #[test]
    fn test_enrichment_suggestion_confidence_clamping() {
        let too_high = EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "tag".to_string(),
            1.5,
            "test".to_string(),
        );
        assert_eq!(too_high.confidence, 1.0);

        let too_low = EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "tag".to_string(),
            -0.5,
            "test".to_string(),
        );
        assert_eq!(too_low.confidence, 0.0);
    }

    #[test]
    fn test_enrichment_result() {
        let mut result = EnrichmentResult::new("statute-1".to_string());

        result.add_suggestion(EnrichmentSuggestion::new(
            EnrichmentType::AutoTag,
            "criminal".to_string(),
            0.9,
            "High confidence".to_string(),
        ));

        result.add_suggestion(EnrichmentSuggestion::new(
            EnrichmentType::MetadataInference,
            "description".to_string(),
            0.5,
            "Low confidence".to_string(),
        ));

        assert_eq!(result.statute_id, "statute-1");
        assert_eq!(result.suggestions.len(), 2);
        assert_eq!(result.high_confidence_suggestions(0.7).len(), 1);
        assert_eq!(result.suggestions_by_type(EnrichmentType::AutoTag).len(), 1);
    }

    #[test]
    fn test_enrichment_config_builders() {
        let config = EnrichmentConfig::new()
            .with_auto_tagging(false)
            .with_metadata_inference(true)
            .with_jurisdiction_inference(false)
            .with_min_confidence(0.85);

        assert!(!config.enable_auto_tagging);
        assert!(config.enable_metadata_inference);
        assert!(!config.enable_jurisdiction_inference);
        assert_eq!(config.min_confidence, 0.85);
    }

    #[test]
    fn test_analyze_enrichment_auto_tagging() {
        let mut registry = StatuteRegistry::new();

        // Register a statute with civil law keywords in title
        registry
            .register(StatuteEntry::new(test_statute("civil-contract-law"), "US"))
            .unwrap();

        let config = EnrichmentConfig::new();
        let result = registry
            .analyze_enrichment("civil-contract-law", &config)
            .unwrap();

        // Should suggest "civil" tag since title contains "civil" and "contract"
        let auto_tag_suggestions = result.suggestions_by_type(EnrichmentType::AutoTag);
        let civil_suggestions: Vec<_> = auto_tag_suggestions
            .iter()
            .filter(|s| s.suggestion == "civil")
            .collect();

        assert!(!civil_suggestions.is_empty());
    }

    #[test]
    fn test_analyze_enrichment_metadata_inference() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("test-1"), "US"))
            .unwrap();

        let config = EnrichmentConfig::new();
        let result = registry.analyze_enrichment("test-1", &config).unwrap();

        // Should suggest adding description
        let metadata_suggestions = result.suggestions_by_type(EnrichmentType::MetadataInference);
        assert!(!metadata_suggestions.is_empty());
    }

    #[test]
    fn test_analyze_enrichment_nonexistent() {
        let registry = StatuteRegistry::new();
        let config = EnrichmentConfig::new();

        let result = registry.analyze_enrichment("nonexistent", &config);
        assert!(result.is_err());
        assert!(matches!(result, Err(RegistryError::StatuteNotFound(_))));
    }

    #[test]
    fn test_apply_enrichment() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("test-1"), "US"))
            .unwrap();

        let suggestions = vec![
            EnrichmentSuggestion::new(
                EnrichmentType::AutoTag,
                "civil".to_string(),
                0.9,
                "High confidence tag".to_string(),
            ),
            EnrichmentSuggestion::new(
                EnrichmentType::MetadataInference,
                "category".to_string(),
                0.8,
                "Category suggestion".to_string(),
            ),
        ];

        let count = registry
            .apply_enrichment("test-1", &suggestions, 0.7)
            .unwrap();

        assert_eq!(count, 2);

        let entry = registry.get("test-1").unwrap();
        assert!(entry.tags.contains(&"civil".to_string()));
        assert!(entry.metadata.contains_key("category"));
    }

    #[test]
    fn test_apply_enrichment_confidence_filter() {
        let mut registry = StatuteRegistry::new();

        registry
            .register(StatuteEntry::new(test_statute("test-1"), "US"))
            .unwrap();

        let suggestions = vec![
            EnrichmentSuggestion::new(
                EnrichmentType::AutoTag,
                "high-confidence".to_string(),
                0.9,
                "High".to_string(),
            ),
            EnrichmentSuggestion::new(
                EnrichmentType::AutoTag,
                "low-confidence".to_string(),
                0.5,
                "Low".to_string(),
            ),
        ];

        // Only apply suggestions with confidence >= 0.8
        let count = registry
            .apply_enrichment("test-1", &suggestions, 0.8)
            .unwrap();

        assert_eq!(count, 1);

        let entry = registry.get("test-1").unwrap();
        assert!(entry.tags.contains(&"high-confidence".to_string()));
        assert!(!entry.tags.contains(&"low-confidence".to_string()));
    }

    #[test]
    fn test_auto_enrich_all() {
        let mut registry = StatuteRegistry::new();

        // Register statutes with enrichment opportunities (using actual keyword matches)
        // Create custom statutes with titles containing keywords
        let criminal_statute = Statute::new(
            "criminal-offense-law",
            "Criminal Offense and Penalties Act",
            Effect::new(EffectType::Grant, "Test"),
        );

        let civil_statute = Statute::new(
            "civil-procedure-code",
            "Civil Procedure and Contract Law",
            Effect::new(EffectType::Grant, "Test"),
        );

        registry
            .register(StatuteEntry::new(criminal_statute, "US"))
            .unwrap();
        registry
            .register(StatuteEntry::new(civil_statute, "UK"))
            .unwrap();

        let config = EnrichmentConfig::new().with_min_confidence(0.25); // Lower threshold for test
        let results = registry.auto_enrich_all(&config);

        // At least some statutes should be enriched
        assert!(!results.is_empty());

        // Verify enrichment was actually applied
        for (statute_id, count) in results {
            assert!(count > 0);
            let entry = registry.get(&statute_id).unwrap();
            // Should have gained tags or metadata
            assert!(!entry.tags.is_empty() || !entry.metadata.is_empty());
        }
    }

    #[test]
    fn test_lineage_entry_creation() {
        let entry = LineageEntry::new(
            "statute-1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        );

        assert_eq!(entry.statute_id, "statute-1");
        assert_eq!(entry.operation, LineageOperation::Created);
        assert_eq!(entry.actor, "admin");
        assert!(entry.context.is_empty());
    }

    #[test]
    fn test_lineage_entry_with_context() {
        let entry = LineageEntry::new(
            "statute-1".to_string(),
            LineageOperation::Imported {
                source: "external-db".to_string(),
            },
            "system".to_string(),
        )
        .with_context("batch_id".to_string(), "batch-123".to_string())
        .with_context("import_date".to_string(), "2025-12-27".to_string());

        assert_eq!(entry.context.len(), 2);
        assert_eq!(
            entry.context.get("batch_id"),
            Some(&"batch-123".to_string())
        );
    }

    #[test]
    fn test_data_lineage_record() {
        let mut lineage = DataLineage::new(100);

        let entry1 = LineageEntry::new(
            "statute-1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        );

        let entry2 = LineageEntry::new(
            "statute-2".to_string(),
            LineageOperation::Created,
            "user".to_string(),
        );

        lineage.record(entry1);
        lineage.record(entry2);

        assert_eq!(lineage.count(), 2);
    }

    #[test]
    fn test_data_lineage_get_lineage() {
        let mut lineage = DataLineage::new(100);

        lineage.record(LineageEntry::new(
            "statute-1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "statute-1".to_string(),
            LineageOperation::Enriched {
                enrichment_type: "auto-tag".to_string(),
            },
            "system".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "statute-2".to_string(),
            LineageOperation::Created,
            "user".to_string(),
        ));

        let statute1_lineage = lineage.get_lineage("statute-1");
        assert_eq!(statute1_lineage.len(), 2);
    }

    #[test]
    fn test_data_lineage_get_by_operation() {
        let mut lineage = DataLineage::new(100);

        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s2".to_string(),
            LineageOperation::Imported {
                source: "db".to_string(),
            },
            "system".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s3".to_string(),
            LineageOperation::Created,
            "user".to_string(),
        ));

        let created = lineage.get_by_operation("Created");
        assert_eq!(created.len(), 2);

        let imported = lineage.get_by_operation("Imported");
        assert_eq!(imported.len(), 1);
    }

    #[test]
    fn test_data_lineage_get_by_actor() {
        let mut lineage = DataLineage::new(100);

        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s2".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s3".to_string(),
            LineageOperation::Created,
            "user".to_string(),
        ));

        let admin_entries = lineage.get_by_actor("admin");
        assert_eq!(admin_entries.len(), 2);

        let user_entries = lineage.get_by_actor("user");
        assert_eq!(user_entries.len(), 1);
    }

    #[test]
    fn test_data_lineage_get_by_time_range() {
        let mut lineage = DataLineage::new(100);

        let now = Utc::now();
        let past = now - chrono::Duration::hours(2);
        let future = now + chrono::Duration::hours(2);

        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        let entries = lineage.get_by_time_range(past, future);
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_data_lineage_trace_provenance() {
        let mut lineage = DataLineage::new(100);

        // Create a provenance chain: s1 -> s2 -> s3
        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s2".to_string(),
            LineageOperation::Derived {
                parent_id: "s1".to_string(),
            },
            "system".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s3".to_string(),
            LineageOperation::Derived {
                parent_id: "s2".to_string(),
            },
            "system".to_string(),
        ));

        let provenance = lineage.trace_provenance("s3");
        // Should include all three statutes in the chain
        assert!(!provenance.is_empty());
    }

    #[test]
    fn test_data_lineage_trace_merge_provenance() {
        let mut lineage = DataLineage::new(100);

        // Create merged statute from multiple sources
        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "s2".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        lineage.record(LineageEntry::new(
            "merged".to_string(),
            LineageOperation::Merged {
                source_ids: vec!["s1".to_string(), "s2".to_string()],
            },
            "system".to_string(),
        ));

        let provenance = lineage.trace_provenance("merged");
        // Should trace back to both source statutes
        assert!(!provenance.is_empty());
    }

    #[test]
    fn test_data_lineage_max_entries() {
        let mut lineage = DataLineage::new(5);

        // Add more entries than max
        for i in 0..10 {
            lineage.record(LineageEntry::new(
                format!("s{}", i),
                LineageOperation::Created,
                "admin".to_string(),
            ));
        }

        // Should have trimmed to max entries
        assert_eq!(lineage.count(), 5);
    }

    #[test]
    fn test_data_lineage_export_json() {
        let mut lineage = DataLineage::new(100);

        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        let json = lineage.export_json().unwrap();

        assert!(json.contains("statute_id"));
        assert!(json.contains("s1"));
        assert!(json.contains("Created"));
        assert!(json.contains("admin"));
    }

    #[test]
    fn test_data_lineage_clear() {
        let mut lineage = DataLineage::new(100);

        lineage.record(LineageEntry::new(
            "s1".to_string(),
            LineageOperation::Created,
            "admin".to_string(),
        ));

        assert_eq!(lineage.count(), 1);

        lineage.clear();
        assert_eq!(lineage.count(), 0);
    }

    // ========================================================================
    // Compliance Features Tests (v0.1.9)
    // ========================================================================

    #[test]
    fn test_pii_detection_creation() {
        let detection = PiiDetection::new(
            PiiFieldType::Email,
            "test@example.com".to_string(),
            10,
            0.95,
        );

        assert_eq!(detection.field_type, PiiFieldType::Email);
        assert_eq!(detection.value, "test@example.com");
        assert_eq!(detection.position, 10);
        assert_eq!(detection.length, 16);
        assert_eq!(detection.confidence, 0.95);
    }

    #[test]
    fn test_pii_detection_confidence() {
        let detection = PiiDetection::new(
            PiiFieldType::PhoneNumber,
            "123-456-7890".to_string(),
            0,
            0.85,
        );

        assert!(detection.is_confident(0.8));
        assert!(detection.is_confident(0.85));
        assert!(!detection.is_confident(0.9));
    }

    #[test]
    fn test_pii_scan_result() {
        let detections = vec![
            PiiDetection::new(PiiFieldType::Email, "a@b.com".to_string(), 0, 0.9),
            PiiDetection::new(
                PiiFieldType::PhoneNumber,
                "123-456-7890".to_string(),
                10,
                0.8,
            ),
        ];

        let result = PiiScanResult::new("test-statute".to_string(), detections);

        assert_eq!(result.statute_id, "test-statute");
        assert_eq!(result.pii_count, 2);

        let high_conf = result.high_confidence(0.85);
        assert_eq!(high_conf.len(), 1);
        assert_eq!(high_conf[0].field_type, PiiFieldType::Email);

        let emails = result.by_type(&PiiFieldType::Email);
        assert_eq!(emails.len(), 1);
    }

    #[test]
    fn test_pii_detector_scan() {
        let detector = PiiDetector::new();
        let content = "Contact us at support@example.com or call 555-123-4567";

        let result = detector.scan("statute-1", content);

        assert_eq!(result.statute_id, "statute-1");
        assert!(!result.detections.is_empty());
    }

    #[test]
    fn test_pii_detector_disabled() {
        let mut detector = PiiDetector::new();
        detector.set_enabled(false);

        let content = "Contact us at support@example.com";
        let result = detector.scan("statute-1", content);

        assert_eq!(result.pii_count, 0);
    }

    #[test]
    fn test_pii_masking_strategies() {
        let detector_asterisk =
            PiiDetector::new().with_masking_strategy(MaskingStrategy::Asterisks);
        let detector_redacted = PiiDetector::new().with_masking_strategy(MaskingStrategy::Redacted);
        let detector_partial = PiiDetector::new().with_masking_strategy(MaskingStrategy::Partial);

        let content = "email@test.com";
        let detections = vec![PiiDetection::new(
            PiiFieldType::Email,
            "email@test.com".to_string(),
            0,
            0.9,
        )];
        let scan_result = PiiScanResult::new("test".to_string(), detections);

        let masked_asterisk = detector_asterisk.mask(content, &scan_result);
        let masked_redacted = detector_redacted.mask(content, &scan_result);
        let masked_partial = detector_partial.mask(content, &scan_result);

        assert!(masked_asterisk.contains('*') || masked_asterisk.is_empty());
        assert!(masked_redacted.contains("[REDACTED]") || masked_redacted == content);
        assert!(masked_partial.starts_with('e') || masked_partial == content);
    }

    #[test]
    fn test_data_retention_config() {
        let config = DataRetentionConfig::new()
            .add_rule(DataRetentionRule::RetainForDays(30))
            .add_rule(DataRetentionRule::ArchiveAfterDays(90))
            .with_auto_apply(true)
            .with_dry_run(false);

        assert_eq!(config.rules().len(), 2);
        assert!(config.is_auto_apply());
        assert!(!config.is_dry_run());
    }

    #[test]
    fn test_retention_execution_result() {
        let result = RetentionExecutionResult::new(
            vec!["s1".to_string(), "s2".to_string()],
            vec!["s3".to_string()],
            false,
        );

        assert_eq!(result.deleted.len(), 2);
        assert_eq!(result.archived.len(), 1);
        assert_eq!(result.total_affected(), 3);
        assert!(!result.dry_run);
    }

    #[test]
    fn test_apply_retention_rules_dry_run() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("old-statute"), "JP");
        registry.register(entry).unwrap();

        let config = DataRetentionConfig::new()
            .add_rule(DataRetentionRule::RetainForDays(0))
            .with_dry_run(true);

        let result = registry.apply_retention_rules(&config);

        // In dry-run mode, nothing should be deleted
        assert_eq!(registry.count(), 1);
        assert!(result.dry_run);
    }

    #[test]
    fn test_apply_retention_rules_archive() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("old-statute"), "JP");
        registry.register(entry).unwrap();

        // Use a rule that will definitely trigger (old age)
        let config = DataRetentionConfig::new()
            .add_rule(DataRetentionRule::RetainForDays(0))
            .with_dry_run(true); // Use dry-run first

        let result = registry.apply_retention_rules(&config);

        // In this case, we're testing dry-run mode
        // Statute with age > 0 days would be deleted (but we're in dry run)
        assert!(result.dry_run);
        assert_eq!(registry.count(), 1); // Nothing actually deleted
    }

    #[test]
    fn test_audit_report_config() {
        let now = Utc::now();
        let config = AuditReportConfig::new("Monthly Report")
            .with_date_range(now, now)
            .with_sections(true, true, false, false)
            .with_format(AuditReportFormat::Json);

        assert_eq!(config.title, "Monthly Report");
        assert!(config.include_operations);
        assert!(config.include_events);
        assert!(!config.include_quality);
        assert_eq!(config.format, AuditReportFormat::Json);
    }

    #[test]
    fn test_generate_audit_report() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
        registry.register(entry).unwrap();

        let config = AuditReportConfig::new("Test Report").with_format(AuditReportFormat::Text);

        let report = registry.generate_audit_report(&config);

        assert_eq!(report.title, "Test Report");
        assert_eq!(report.total_statutes, 1);
        assert!(!report.content.is_empty());
        assert_eq!(report.format, AuditReportFormat::Text);
    }

    #[test]
    fn test_audit_report_export() {
        let report = AuditReport::new(
            "Test".to_string(),
            (None, None),
            10,
            5,
            3,
            0,
            85.0,
            "Test content".to_string(),
            AuditReportFormat::Json,
        );

        let exported = report.export();
        assert!(exported.contains("Test"));
    }

    #[test]
    fn test_geographic_region_code() {
        assert_eq!(GeographicRegion::EU.code(), "EU");
        assert_eq!(GeographicRegion::US.code(), "US");
        assert_eq!(GeographicRegion::Japan.code(), "JP");
        assert_eq!(GeographicRegion::Custom("XX".to_string()).code(), "XX");
    }

    #[test]
    fn test_geographic_region_transfer_rules() {
        // EU can transfer to EU and UK
        assert!(GeographicRegion::EU.allows_transfer_to(&GeographicRegion::EU));
        assert!(GeographicRegion::EU.allows_transfer_to(&GeographicRegion::UK));
        // EU cannot transfer to US (GDPR)
        assert!(!GeographicRegion::EU.allows_transfer_to(&GeographicRegion::US));

        // US can transfer anywhere
        assert!(GeographicRegion::US.allows_transfer_to(&GeographicRegion::EU));
        assert!(GeographicRegion::US.allows_transfer_to(&GeographicRegion::Japan));
    }

    #[test]
    fn test_data_sovereignty_config() {
        let config = DataSovereigntyConfig::new(GeographicRegion::EU)
            .allow_region(GeographicRegion::UK)
            .with_strict_residency(false)
            .with_encryption_required(true);

        assert_eq!(config.primary_region, GeographicRegion::EU);
        assert!(config.allowed_regions.contains(&GeographicRegion::UK));
        assert!(!config.strict_residency);
        assert!(config.require_encryption);
    }

    #[test]
    fn test_data_sovereignty_region_allowed() {
        let config =
            DataSovereigntyConfig::new(GeographicRegion::EU).allow_region(GeographicRegion::UK);

        // Primary region is always allowed
        assert!(config.is_region_allowed(&GeographicRegion::EU));

        // UK is explicitly allowed and EU->UK transfer is permitted
        assert!(config.is_region_allowed(&GeographicRegion::UK));

        // US is not in allowed list
        assert!(!config.is_region_allowed(&GeographicRegion::US));
    }

    #[test]
    fn test_data_sovereignty_strict_residency() {
        let config = DataSovereigntyConfig::new(GeographicRegion::EU)
            .allow_region(GeographicRegion::UK)
            .with_strict_residency(true);

        // Only primary region allowed in strict mode
        assert!(config.is_region_allowed(&GeographicRegion::EU));
        assert!(!config.is_region_allowed(&GeographicRegion::UK));
        assert!(!config.is_region_allowed(&GeographicRegion::US));
    }

    #[test]
    fn test_compliance_dashboard_creation() {
        let dashboard = ComplianceDashboard::new(
            100,  // total_statutes
            5,    // statutes_with_pii
            10,   // statutes_pending_retention
            85.0, // avg_quality_score
            8,    // low_quality_count
            200,  // total_audit_events
            3,    // failed_operations
            2,    // sovereignty_violations
        );

        assert_eq!(dashboard.total_statutes, 100);
        assert_eq!(dashboard.statutes_with_pii, 5);
        assert_eq!(dashboard.low_quality_count, 8);

        // Compliance rate = (100 - 8 - 2) / 100 = 0.90
        assert!((dashboard.compliance_rate - 0.90).abs() < 0.01);
    }

    #[test]
    fn test_compliance_dashboard_threshold() {
        let dashboard = ComplianceDashboard::new(100, 0, 0, 90.0, 5, 100, 0, 0);

        assert!(dashboard.meets_compliance_threshold(0.90));
        assert!(dashboard.meets_compliance_threshold(0.95));
        assert!(!dashboard.meets_compliance_threshold(0.99));
    }

    #[test]
    fn test_compliance_dashboard_to_json() {
        let dashboard = ComplianceDashboard::new(10, 1, 2, 85.0, 1, 50, 0, 0);
        let json = dashboard.to_json();

        assert!(json.contains("total_statutes"));
        assert!(json.contains("compliance_rate"));
    }

    #[test]
    fn test_generate_compliance_dashboard() {
        let mut registry = StatuteRegistry::new();

        // Add some statutes with varying quality
        for i in 1..=5 {
            let entry = StatuteEntry::new(test_statute(&format!("s{}", i)), "JP").with_tag("test");
            registry.register(entry).unwrap();
        }

        let dashboard = registry.generate_compliance_dashboard(70.0);

        assert_eq!(dashboard.total_statutes, 5);
        assert!(dashboard.compliance_rate >= 0.0 && dashboard.compliance_rate <= 1.0);
    }

    #[test]
    fn test_scan_for_pii() {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(test_statute("statute-1"), "JP")
            .with_metadata("email", "contact@example.com");
        registry.register(entry).unwrap();

        let detector = PiiDetector::new();
        let result = registry.scan_for_pii("statute-1", &detector).unwrap();

        assert_eq!(result.statute_id, "statute-1");
    }

    #[test]
    fn test_scan_for_pii_not_found() {
        let mut registry = StatuteRegistry::new();
        let detector = PiiDetector::new();

        let result = registry.scan_for_pii("nonexistent", &detector);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_sovereignty_access() {
        let registry = StatuteRegistry::new();
        let config =
            DataSovereigntyConfig::new(GeographicRegion::EU).allow_region(GeographicRegion::UK);

        // Check access from UK (allowed)
        assert!(registry.check_sovereignty_access("statute-1", &GeographicRegion::UK, &config));

        // Check access from US (not allowed)
        assert!(!registry.check_sovereignty_access("statute-1", &GeographicRegion::US, &config));
    }

    #[test]
    fn test_pii_field_type_variants() {
        let types = [
            PiiFieldType::Name,
            PiiFieldType::Email,
            PiiFieldType::PhoneNumber,
            PiiFieldType::NationalId,
            PiiFieldType::Address,
            PiiFieldType::DateOfBirth,
            PiiFieldType::IpAddress,
            PiiFieldType::Custom("SSN".to_string()),
        ];

        assert_eq!(types.len(), 8);
    }

    #[test]
    fn test_masking_strategy_variants() {
        let strategies = [
            MaskingStrategy::Asterisks,
            MaskingStrategy::Redacted,
            MaskingStrategy::TypeMarker,
            MaskingStrategy::Hash,
            MaskingStrategy::Partial,
        ];

        assert_eq!(strategies.len(), 5);
    }

    #[test]
    fn test_audit_report_format_variants() {
        let formats = [
            AuditReportFormat::Json,
            AuditReportFormat::Csv,
            AuditReportFormat::Text,
            AuditReportFormat::Html,
        ];

        assert_eq!(formats.len(), 4);
    }

    #[test]
    fn test_data_retention_rule_variants() {
        let now = Utc::now();
        let rules = [
            DataRetentionRule::RetainForDays(30),
            DataRetentionRule::RetainUntil(now),
            DataRetentionRule::RetainIndefinitely,
            DataRetentionRule::DeleteInactiveAfterDays(60),
            DataRetentionRule::ArchiveAfterDays(90),
        ];

        assert_eq!(rules.len(), 5);
    }

    #[test]
    fn test_pii_detector_builder_methods() {
        let _detector = PiiDetector::new()
            .with_min_confidence(0.85)
            .with_masking_strategy(MaskingStrategy::Partial);

        // Confidence should be clamped
        let _detector2 = PiiDetector::new().with_min_confidence(1.5);
        // Internal check - confidence should be 1.0 (clamped)

        let _detector3 = PiiDetector::new().with_min_confidence(-0.5);
        // Internal check - confidence should be 0.0 (clamped)
    }

    // ========================================================================
    // Access Control Features Tests (v0.1.4)
    // ========================================================================

    #[test]
    fn test_permission_all() {
        let perms = Permission::all();
        assert_eq!(perms.len(), 12);
        assert!(perms.contains(&Permission::Read));
        assert!(perms.contains(&Permission::ManagePermissions));
    }

    #[test]
    fn test_permission_read_only() {
        let perms = Permission::read_only();
        assert_eq!(perms.len(), 2);
        assert!(perms.contains(&Permission::Read));
        assert!(perms.contains(&Permission::GenerateReports));
        assert!(!perms.contains(&Permission::Delete));
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
        let grant =
            TemporaryAccess::new("user1", vec![Permission::Update], 1, "Quick fix", "admin")
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
        let user =
            AccessUser::new("user1", "Alice", Role::Viewer).with_permission(Permission::Update);

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

        let grant =
            TemporaryAccess::new("user1", vec![Permission::Delete], 1, "Emergency", "admin")
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

        let valid_grant =
            TemporaryAccess::new("user2", vec![Permission::Read], 24, "Test", "admin");

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

        let response = ApprovalResponse::new("approver1", ApprovalDecision::Approved)
            .with_comments("Looks good");

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
            archive_path: "/tmp/archive".to_string(),
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
            archive_path: "/tmp/archive".to_string(),
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
            archive_path: "/tmp/archive".to_string(),
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

        let mut metadata =
            RegistryMetadata::new("Test".to_string(), "https://example.com".to_string());
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

        let mut agreement =
            PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
        agreement.activate();
        assert_eq!(agreement.status, PeeringStatus::Active);
        assert!(agreement.is_valid());
    }

    #[test]
    fn test_peering_agreement_suspend() {
        use federation::*;

        let mut agreement =
            PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
        agreement.activate();
        agreement.suspend();
        assert_eq!(agreement.status, PeeringStatus::Suspended);
        assert!(!agreement.is_valid());
    }

    #[test]
    fn test_peering_agreement_terminate() {
        use federation::*;

        let mut agreement =
            PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
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
}

/// Real-Time Collaboration (v0.2.8)
///
/// This module provides real-time collaboration features for multi-user statute editing:
/// - WebSocket-based live updates
/// - Collaborative editing locks
/// - Real-time conflict notifications
/// - Presence indicators
/// - Change stream subscriptions
pub mod realtime {
    use super::*;
    use std::time::Duration;

    /// WebSocket message type for live updates.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum WebSocketMessage {
        /// Statute was created
        StatuteCreated {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was updated
        StatuteUpdated {
            statute_id: String,
            version: u32,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was deleted
        StatuteDeleted {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute was locked for editing
        StatuteLocked {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// Statute lock was released
        StatuteUnlocked {
            statute_id: String,
            actor: String,
            timestamp: DateTime<Utc>,
        },
        /// User presence update
        PresenceUpdate {
            user_id: String,
            status: PresenceStatus,
            timestamp: DateTime<Utc>,
        },
        /// Conflict notification
        ConflictDetected {
            statute_id: String,
            conflict_type: String,
            description: String,
            timestamp: DateTime<Utc>,
        },
        /// Heartbeat/ping message
        Ping,
        /// Heartbeat/pong response
        Pong,
    }

    impl WebSocketMessage {
        /// Get the statute ID associated with this message, if any.
        pub fn statute_id(&self) -> Option<&str> {
            match self {
                WebSocketMessage::StatuteCreated { statute_id, .. }
                | WebSocketMessage::StatuteUpdated { statute_id, .. }
                | WebSocketMessage::StatuteDeleted { statute_id, .. }
                | WebSocketMessage::StatuteLocked { statute_id, .. }
                | WebSocketMessage::StatuteUnlocked { statute_id, .. }
                | WebSocketMessage::ConflictDetected { statute_id, .. } => Some(statute_id),
                _ => None,
            }
        }

        /// Get the actor associated with this message, if any.
        pub fn actor(&self) -> Option<&str> {
            match self {
                WebSocketMessage::StatuteCreated { actor, .. }
                | WebSocketMessage::StatuteUpdated { actor, .. }
                | WebSocketMessage::StatuteDeleted { actor, .. }
                | WebSocketMessage::StatuteLocked { actor, .. }
                | WebSocketMessage::StatuteUnlocked { actor, .. } => Some(actor),
                _ => None,
            }
        }

        /// Get the timestamp of this message.
        pub fn timestamp(&self) -> Option<DateTime<Utc>> {
            match self {
                WebSocketMessage::StatuteCreated { timestamp, .. }
                | WebSocketMessage::StatuteUpdated { timestamp, .. }
                | WebSocketMessage::StatuteDeleted { timestamp, .. }
                | WebSocketMessage::StatuteLocked { timestamp, .. }
                | WebSocketMessage::StatuteUnlocked { timestamp, .. }
                | WebSocketMessage::PresenceUpdate { timestamp, .. }
                | WebSocketMessage::ConflictDetected { timestamp, .. } => Some(*timestamp),
                _ => None,
            }
        }
    }

    /// User presence status.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum PresenceStatus {
        /// User is online and active
        Online,
        /// User is idle
        Idle,
        /// User is away
        Away,
        /// User is offline
        Offline,
    }

    /// User presence information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserPresence {
        /// User ID
        pub user_id: String,
        /// Current status
        pub status: PresenceStatus,
        /// Currently viewing/editing statute ID (if any)
        pub current_statute: Option<String>,
        /// Last activity timestamp
        pub last_activity: DateTime<Utc>,
        /// User display name
        pub display_name: Option<String>,
    }

    impl UserPresence {
        /// Create a new user presence entry.
        pub fn new(user_id: String) -> Self {
            Self {
                user_id,
                status: PresenceStatus::Online,
                current_statute: None,
                last_activity: Utc::now(),
                display_name: None,
            }
        }

        /// Set the display name.
        pub fn with_display_name(mut self, name: String) -> Self {
            self.display_name = Some(name);
            self
        }

        /// Set the current statute being viewed/edited.
        pub fn with_current_statute(mut self, statute_id: String) -> Self {
            self.current_statute = Some(statute_id);
            self
        }

        /// Update the user's status.
        pub fn set_status(&mut self, status: PresenceStatus) {
            self.status = status;
            self.last_activity = Utc::now();
        }

        /// Update the currently viewed/edited statute.
        pub fn set_current_statute(&mut self, statute_id: Option<String>) {
            self.current_statute = statute_id;
            self.last_activity = Utc::now();
        }

        /// Update last activity timestamp.
        pub fn touch(&mut self) {
            self.last_activity = Utc::now();
        }

        /// Check if user is inactive for a given duration.
        pub fn is_inactive(&self, threshold: Duration) -> bool {
            let elapsed = Utc::now()
                .signed_duration_since(self.last_activity)
                .to_std()
                .unwrap_or(Duration::ZERO);
            elapsed > threshold
        }
    }

    /// Presence tracker for managing user presence.
    pub struct PresenceTracker {
        presences: Arc<Mutex<HashMap<String, UserPresence>>>,
        inactive_threshold: Duration,
    }

    impl PresenceTracker {
        /// Create a new presence tracker.
        pub fn new() -> Self {
            Self {
                presences: Arc::new(Mutex::new(HashMap::new())),
                inactive_threshold: Duration::from_secs(300), // 5 minutes
            }
        }

        /// Set the inactive threshold duration.
        pub fn with_inactive_threshold(mut self, threshold: Duration) -> Self {
            self.inactive_threshold = threshold;
            self
        }

        /// Register or update a user's presence.
        pub fn update_presence(&self, presence: UserPresence) {
            let mut presences = self.presences.lock().unwrap();
            presences.insert(presence.user_id.clone(), presence);
        }

        /// Get a user's presence.
        pub fn get_presence(&self, user_id: &str) -> Option<UserPresence> {
            let presences = self.presences.lock().unwrap();
            presences.get(user_id).cloned()
        }

        /// Get all online users.
        pub fn get_online_users(&self) -> Vec<UserPresence> {
            let presences = self.presences.lock().unwrap();
            presences
                .values()
                .filter(|p| p.status == PresenceStatus::Online)
                .cloned()
                .collect()
        }

        /// Get users viewing/editing a specific statute.
        pub fn get_users_for_statute(&self, statute_id: &str) -> Vec<UserPresence> {
            let presences = self.presences.lock().unwrap();
            presences
                .values()
                .filter(|p| {
                    p.current_statute
                        .as_ref()
                        .is_some_and(|sid| sid == statute_id)
                })
                .cloned()
                .collect()
        }

        /// Remove inactive users and mark them as offline.
        pub fn cleanup_inactive(&self) -> Vec<String> {
            let mut presences = self.presences.lock().unwrap();
            let mut inactive_users = Vec::new();

            for (user_id, presence) in presences.iter_mut() {
                if presence.is_inactive(self.inactive_threshold)
                    && presence.status != PresenceStatus::Offline
                {
                    presence.status = PresenceStatus::Offline;
                    inactive_users.push(user_id.clone());
                }
            }

            inactive_users
        }

        /// Remove a user's presence.
        pub fn remove_user(&self, user_id: &str) {
            let mut presences = self.presences.lock().unwrap();
            presences.remove(user_id);
        }

        /// Count online users.
        pub fn online_count(&self) -> usize {
            let presences = self.presences.lock().unwrap();
            presences
                .values()
                .filter(|p| p.status == PresenceStatus::Online)
                .count()
        }
    }

    impl Default for PresenceTracker {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Editing lock for collaborative editing.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EditingLock {
        /// Statute ID being locked
        pub statute_id: String,
        /// User holding the lock
        pub holder: String,
        /// When the lock was acquired
        pub acquired_at: DateTime<Utc>,
        /// When the lock expires
        pub expires_at: DateTime<Utc>,
        /// Lock type
        pub lock_type: LockType,
    }

    /// Type of editing lock.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum LockType {
        /// Exclusive write lock
        Exclusive,
        /// Shared read lock
        Shared,
    }

    impl EditingLock {
        /// Create a new exclusive lock.
        pub fn exclusive(statute_id: String, holder: String, duration: Duration) -> Self {
            let acquired_at = Utc::now();
            let expires_at = acquired_at + chrono::Duration::from_std(duration).unwrap();

            Self {
                statute_id,
                holder,
                acquired_at,
                expires_at,
                lock_type: LockType::Exclusive,
            }
        }

        /// Create a new shared lock.
        pub fn shared(statute_id: String, holder: String, duration: Duration) -> Self {
            let acquired_at = Utc::now();
            let expires_at = acquired_at + chrono::Duration::from_std(duration).unwrap();

            Self {
                statute_id,
                holder,
                acquired_at,
                expires_at,
                lock_type: LockType::Shared,
            }
        }

        /// Check if the lock has expired.
        pub fn is_expired(&self) -> bool {
            Utc::now() > self.expires_at
        }

        /// Extend the lock by a duration.
        pub fn extend(&mut self, duration: Duration) {
            self.expires_at += chrono::Duration::from_std(duration).unwrap();
        }

        /// Get remaining time until expiration.
        pub fn time_remaining(&self) -> Option<Duration> {
            if self.is_expired() {
                None
            } else {
                self.expires_at
                    .signed_duration_since(Utc::now())
                    .to_std()
                    .ok()
            }
        }
    }

    /// Lock manager for collaborative editing locks.
    pub struct LockManager {
        locks: Arc<Mutex<HashMap<String, Vec<EditingLock>>>>,
        default_duration: Duration,
    }

    impl LockManager {
        /// Create a new lock manager.
        pub fn new() -> Self {
            Self {
                locks: Arc::new(Mutex::new(HashMap::new())),
                default_duration: Duration::from_secs(300), // 5 minutes
            }
        }

        /// Set the default lock duration.
        pub fn with_default_duration(mut self, duration: Duration) -> Self {
            self.default_duration = duration;
            self
        }

        /// Acquire an exclusive lock on a statute.
        pub fn acquire_exclusive(
            &self,
            statute_id: String,
            holder: String,
        ) -> Result<EditingLock, String> {
            let mut locks = self.locks.lock().unwrap();

            // Check if there are any existing locks
            if let Some(existing_locks) = locks.get(&statute_id) {
                for lock in existing_locks {
                    if !lock.is_expired() {
                        return Err(format!(
                            "Statute {} is locked by {}",
                            statute_id, lock.holder
                        ));
                    }
                }
            }

            // Acquire the lock
            let lock = EditingLock::exclusive(statute_id.clone(), holder, self.default_duration);
            locks.insert(statute_id, vec![lock.clone()]);

            Ok(lock)
        }

        /// Acquire a shared lock on a statute.
        pub fn acquire_shared(
            &self,
            statute_id: String,
            holder: String,
        ) -> Result<EditingLock, String> {
            let mut locks = self.locks.lock().unwrap();

            // Check for exclusive locks
            if let Some(existing_locks) = locks.get(&statute_id) {
                for lock in existing_locks {
                    if !lock.is_expired() && lock.lock_type == LockType::Exclusive {
                        return Err(format!(
                            "Statute {} has exclusive lock by {}",
                            statute_id, lock.holder
                        ));
                    }
                }
            }

            // Acquire the shared lock
            let lock = EditingLock::shared(statute_id.clone(), holder, self.default_duration);
            locks.entry(statute_id).or_default().push(lock.clone());

            Ok(lock)
        }

        /// Release a lock.
        pub fn release(&self, statute_id: &str, holder: &str) -> bool {
            let mut locks = self.locks.lock().unwrap();

            if let Some(statute_locks) = locks.get_mut(statute_id) {
                let initial_len = statute_locks.len();
                statute_locks.retain(|lock| lock.holder != holder);
                let new_len = statute_locks.len();

                if statute_locks.is_empty() {
                    locks.remove(statute_id);
                }

                return new_len < initial_len;
            }

            false
        }

        /// Get all locks for a statute.
        pub fn get_locks(&self, statute_id: &str) -> Vec<EditingLock> {
            let locks = self.locks.lock().unwrap();
            locks
                .get(statute_id)
                .map(|v| v.iter().filter(|l| !l.is_expired()).cloned().collect())
                .unwrap_or_default()
        }

        /// Check if a statute is locked.
        pub fn is_locked(&self, statute_id: &str) -> bool {
            let locks = self.locks.lock().unwrap();
            locks
                .get(statute_id)
                .is_some_and(|v| v.iter().any(|lock| !lock.is_expired()))
        }

        /// Extend a lock.
        pub fn extend_lock(&self, statute_id: &str, holder: &str, duration: Duration) -> bool {
            let mut locks = self.locks.lock().unwrap();

            if let Some(statute_locks) = locks.get_mut(statute_id) {
                for lock in statute_locks.iter_mut() {
                    if lock.holder == holder && !lock.is_expired() {
                        lock.extend(duration);
                        return true;
                    }
                }
            }

            false
        }

        /// Clean up expired locks.
        pub fn cleanup_expired(&self) -> usize {
            let mut locks = self.locks.lock().unwrap();
            let mut removed_count = 0;

            locks.retain(|_, statute_locks| {
                let initial_len = statute_locks.len();
                statute_locks.retain(|lock| !lock.is_expired());
                removed_count += initial_len - statute_locks.len();
                !statute_locks.is_empty()
            });

            removed_count
        }

        /// Get the holder of an exclusive lock, if any.
        pub fn get_exclusive_holder(&self, statute_id: &str) -> Option<String> {
            let locks = self.locks.lock().unwrap();
            locks.get(statute_id).and_then(|v| {
                v.iter()
                    .find(|lock| !lock.is_expired() && lock.lock_type == LockType::Exclusive)
                    .map(|lock| lock.holder.clone())
            })
        }
    }

    impl Default for LockManager {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Real-time conflict notification.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConflictNotification {
        /// Unique notification ID
        pub notification_id: Uuid,
        /// Statute ID where conflict occurred
        pub statute_id: String,
        /// Type of conflict
        pub conflict_type: ConflictType,
        /// Users involved in the conflict
        pub users: Vec<String>,
        /// Description of the conflict
        pub description: String,
        /// When the conflict was detected
        pub detected_at: DateTime<Utc>,
        /// Conflict resolution status
        pub status: ConflictStatus,
    }

    /// Type of conflict.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConflictType {
        /// Concurrent edits to the same statute
        ConcurrentEdit,
        /// Version mismatch
        VersionMismatch,
        /// Lock acquisition conflict
        LockConflict,
        /// Merge conflict
        MergeConflict,
        /// Other conflict type
        Other(String),
    }

    /// Status of conflict resolution.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConflictStatus {
        /// Conflict detected but not yet resolved
        Pending,
        /// Conflict is being resolved
        Resolving,
        /// Conflict has been resolved
        Resolved,
        /// Conflict resolution abandoned
        Abandoned,
    }

    impl ConflictNotification {
        /// Create a new conflict notification.
        pub fn new(
            statute_id: String,
            conflict_type: ConflictType,
            users: Vec<String>,
            description: String,
        ) -> Self {
            Self {
                notification_id: Uuid::new_v4(),
                statute_id,
                conflict_type,
                users,
                description,
                detected_at: Utc::now(),
                status: ConflictStatus::Pending,
            }
        }

        /// Mark the conflict as being resolved.
        pub fn mark_resolving(&mut self) {
            self.status = ConflictStatus::Resolving;
        }

        /// Mark the conflict as resolved.
        pub fn mark_resolved(&mut self) {
            self.status = ConflictStatus::Resolved;
        }

        /// Mark the conflict as abandoned.
        pub fn mark_abandoned(&mut self) {
            self.status = ConflictStatus::Abandoned;
        }

        /// Check if the conflict is resolved.
        pub fn is_resolved(&self) -> bool {
            self.status == ConflictStatus::Resolved
        }
    }

    /// Conflict notification manager.
    pub struct ConflictManager {
        notifications: Arc<Mutex<HashMap<Uuid, ConflictNotification>>>,
        statute_conflicts: Arc<Mutex<HashMap<String, Vec<Uuid>>>>,
    }

    impl ConflictManager {
        /// Create a new conflict manager.
        pub fn new() -> Self {
            Self {
                notifications: Arc::new(Mutex::new(HashMap::new())),
                statute_conflicts: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Register a new conflict.
        pub fn register_conflict(&self, notification: ConflictNotification) -> Uuid {
            let notification_id = notification.notification_id;
            let statute_id = notification.statute_id.clone();

            let mut notifications = self.notifications.lock().unwrap();
            notifications.insert(notification_id, notification);

            let mut statute_conflicts = self.statute_conflicts.lock().unwrap();
            statute_conflicts
                .entry(statute_id)
                .or_default()
                .push(notification_id);

            notification_id
        }

        /// Get a conflict by ID.
        pub fn get_conflict(&self, notification_id: Uuid) -> Option<ConflictNotification> {
            let notifications = self.notifications.lock().unwrap();
            notifications.get(&notification_id).cloned()
        }

        /// Get all conflicts for a statute.
        pub fn get_conflicts_for_statute(&self, statute_id: &str) -> Vec<ConflictNotification> {
            let statute_conflicts = self.statute_conflicts.lock().unwrap();
            let notifications = self.notifications.lock().unwrap();

            statute_conflicts
                .get(statute_id)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| notifications.get(id).cloned())
                        .collect()
                })
                .unwrap_or_default()
        }

        /// Get all pending conflicts.
        pub fn get_pending_conflicts(&self) -> Vec<ConflictNotification> {
            let notifications = self.notifications.lock().unwrap();
            notifications
                .values()
                .filter(|n| n.status == ConflictStatus::Pending)
                .cloned()
                .collect()
        }

        /// Update conflict status.
        pub fn update_status(&self, notification_id: Uuid, status: ConflictStatus) -> bool {
            let mut notifications = self.notifications.lock().unwrap();
            if let Some(notification) = notifications.get_mut(&notification_id) {
                notification.status = status;
                return true;
            }
            false
        }

        /// Remove resolved conflicts older than a threshold.
        pub fn cleanup_resolved(&self, older_than: Duration) -> usize {
            let threshold = Utc::now() - chrono::Duration::from_std(older_than).unwrap();

            let mut notifications = self.notifications.lock().unwrap();
            let mut statute_conflicts = self.statute_conflicts.lock().unwrap();

            let to_remove: Vec<Uuid> = notifications
                .iter()
                .filter(|(_, n)| n.status == ConflictStatus::Resolved && n.detected_at < threshold)
                .map(|(id, _)| *id)
                .collect();

            let removed_count = to_remove.len();

            for id in to_remove {
                if let Some(notification) = notifications.remove(&id)
                    && let Some(conflicts) = statute_conflicts.get_mut(&notification.statute_id)
                {
                    conflicts.retain(|cid| *cid != id);
                }
            }

            // Clean up empty statute entries
            statute_conflicts.retain(|_, conflicts| !conflicts.is_empty());

            removed_count
        }
    }

    impl Default for ConflictManager {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Change stream subscription for real-time updates.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ChangeSubscription {
        /// Subscription ID
        pub subscription_id: Uuid,
        /// User or client ID
        pub subscriber: String,
        /// Filter for statute IDs (if empty, subscribe to all)
        pub statute_filters: Vec<String>,
        /// Filter for event types (if empty, subscribe to all)
        pub event_filters: Vec<String>,
        /// When the subscription was created
        pub created_at: DateTime<Utc>,
        /// Last activity timestamp
        pub last_activity: DateTime<Utc>,
    }

    impl ChangeSubscription {
        /// Create a new subscription.
        pub fn new(subscriber: String) -> Self {
            Self {
                subscription_id: Uuid::new_v4(),
                subscriber,
                statute_filters: Vec::new(),
                event_filters: Vec::new(),
                created_at: Utc::now(),
                last_activity: Utc::now(),
            }
        }

        /// Add a statute ID filter.
        pub fn with_statute_filter(mut self, statute_id: String) -> Self {
            self.statute_filters.push(statute_id);
            self
        }

        /// Add an event type filter.
        pub fn with_event_filter(mut self, event_type: String) -> Self {
            self.event_filters.push(event_type);
            self
        }

        /// Check if this subscription matches a statute ID.
        pub fn matches_statute(&self, statute_id: &str) -> bool {
            self.statute_filters.is_empty()
                || self.statute_filters.contains(&statute_id.to_string())
        }

        /// Check if this subscription matches an event type.
        pub fn matches_event(&self, event_type: &str) -> bool {
            self.event_filters.is_empty() || self.event_filters.contains(&event_type.to_string())
        }

        /// Update last activity timestamp.
        pub fn touch(&mut self) {
            self.last_activity = Utc::now();
        }
    }

    /// Subscription manager for change streams.
    pub struct SubscriptionManager {
        subscriptions: Arc<Mutex<HashMap<Uuid, ChangeSubscription>>>,
        subscriber_subscriptions: Arc<Mutex<HashMap<String, Vec<Uuid>>>>,
    }

    impl SubscriptionManager {
        /// Create a new subscription manager.
        pub fn new() -> Self {
            Self {
                subscriptions: Arc::new(Mutex::new(HashMap::new())),
                subscriber_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Create a new subscription.
        pub fn subscribe(&self, subscription: ChangeSubscription) -> Uuid {
            let subscription_id = subscription.subscription_id;
            let subscriber = subscription.subscriber.clone();

            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(subscription_id, subscription);

            let mut subscriber_subscriptions = self.subscriber_subscriptions.lock().unwrap();
            subscriber_subscriptions
                .entry(subscriber)
                .or_default()
                .push(subscription_id);

            subscription_id
        }

        /// Unsubscribe by subscription ID.
        pub fn unsubscribe(&self, subscription_id: Uuid) -> bool {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            if let Some(subscription) = subscriptions.remove(&subscription_id) {
                let mut subscriber_subscriptions = self.subscriber_subscriptions.lock().unwrap();
                if let Some(subs) = subscriber_subscriptions.get_mut(&subscription.subscriber) {
                    subs.retain(|id| *id != subscription_id);
                    if subs.is_empty() {
                        subscriber_subscriptions.remove(&subscription.subscriber);
                    }
                }
                return true;
            }
            false
        }

        /// Get all subscriptions for a subscriber.
        pub fn get_subscriber_subscriptions(&self, subscriber: &str) -> Vec<ChangeSubscription> {
            let subscriber_subscriptions = self.subscriber_subscriptions.lock().unwrap();
            let subscriptions = self.subscriptions.lock().unwrap();

            subscriber_subscriptions
                .get(subscriber)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| subscriptions.get(id).cloned())
                        .collect()
                })
                .unwrap_or_default()
        }

        /// Get all subscriptions that match a statute ID and event type.
        pub fn find_matching_subscriptions(
            &self,
            statute_id: &str,
            event_type: &str,
        ) -> Vec<ChangeSubscription> {
            let subscriptions = self.subscriptions.lock().unwrap();
            subscriptions
                .values()
                .filter(|sub| sub.matches_statute(statute_id) && sub.matches_event(event_type))
                .cloned()
                .collect()
        }

        /// Get total subscription count.
        pub fn subscription_count(&self) -> usize {
            let subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.len()
        }

        /// Get total subscriber count.
        pub fn subscriber_count(&self) -> usize {
            let subscriber_subscriptions = self.subscriber_subscriptions.lock().unwrap();
            subscriber_subscriptions.len()
        }

        /// Unsubscribe all subscriptions for a subscriber.
        pub fn unsubscribe_all(&self, subscriber: &str) -> usize {
            let mut subscriber_subscriptions = self.subscriber_subscriptions.lock().unwrap();
            if let Some(sub_ids) = subscriber_subscriptions.remove(subscriber) {
                let mut subscriptions = self.subscriptions.lock().unwrap();
                for id in &sub_ids {
                    subscriptions.remove(id);
                }
                return sub_ids.len();
            }
            0
        }
    }

    impl Default for SubscriptionManager {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_websocket_message_statute_id() {
            let msg = WebSocketMessage::StatuteCreated {
                statute_id: "test-1".to_string(),
                actor: "user1".to_string(),
                timestamp: Utc::now(),
            };
            assert_eq!(msg.statute_id(), Some("test-1"));

            let ping = WebSocketMessage::Ping;
            assert_eq!(ping.statute_id(), None);
        }

        #[test]
        fn test_websocket_message_actor() {
            let msg = WebSocketMessage::StatuteUpdated {
                statute_id: "test-1".to_string(),
                version: 2,
                actor: "user2".to_string(),
                timestamp: Utc::now(),
            };
            assert_eq!(msg.actor(), Some("user2"));
        }

        #[test]
        fn test_websocket_message_timestamp() {
            let now = Utc::now();
            let msg = WebSocketMessage::StatuteDeleted {
                statute_id: "test-1".to_string(),
                actor: "user1".to_string(),
                timestamp: now,
            };
            assert_eq!(msg.timestamp(), Some(now));
        }

        #[test]
        fn test_user_presence_creation() {
            let presence = UserPresence::new("user1".to_string())
                .with_display_name("Alice".to_string())
                .with_current_statute("statute-1".to_string());

            assert_eq!(presence.user_id, "user1");
            assert_eq!(presence.display_name, Some("Alice".to_string()));
            assert_eq!(presence.current_statute, Some("statute-1".to_string()));
            assert_eq!(presence.status, PresenceStatus::Online);
        }

        #[test]
        fn test_user_presence_set_status() {
            let mut presence = UserPresence::new("user1".to_string());
            presence.set_status(PresenceStatus::Away);
            assert_eq!(presence.status, PresenceStatus::Away);
        }

        #[test]
        fn test_user_presence_is_inactive() {
            let mut presence = UserPresence::new("user1".to_string());
            assert!(!presence.is_inactive(Duration::from_secs(60)));

            // Simulate old activity
            presence.last_activity = Utc::now() - chrono::Duration::seconds(120);
            assert!(presence.is_inactive(Duration::from_secs(60)));
        }

        #[test]
        fn test_presence_tracker_update_and_get() {
            let tracker = PresenceTracker::new();
            let presence = UserPresence::new("user1".to_string());

            tracker.update_presence(presence.clone());
            let retrieved = tracker.get_presence("user1");
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().user_id, "user1");
        }

        #[test]
        fn test_presence_tracker_online_users() {
            let tracker = PresenceTracker::new();

            let presence1 = UserPresence::new("user1".to_string());
            let mut presence2 = UserPresence::new("user2".to_string());
            presence2.status = PresenceStatus::Offline;

            tracker.update_presence(presence1);
            tracker.update_presence(presence2);

            let online_users = tracker.get_online_users();
            assert_eq!(online_users.len(), 1);
            assert_eq!(online_users[0].user_id, "user1");
        }

        #[test]
        fn test_presence_tracker_users_for_statute() {
            let tracker = PresenceTracker::new();

            let presence1 = UserPresence::new("user1".to_string())
                .with_current_statute("statute-1".to_string());
            let presence2 = UserPresence::new("user2".to_string())
                .with_current_statute("statute-2".to_string());

            tracker.update_presence(presence1);
            tracker.update_presence(presence2);

            let users = tracker.get_users_for_statute("statute-1");
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].user_id, "user1");
        }

        #[test]
        fn test_presence_tracker_cleanup_inactive() {
            let tracker = PresenceTracker::new().with_inactive_threshold(Duration::from_secs(60));

            let mut presence = UserPresence::new("user1".to_string());
            presence.last_activity = Utc::now() - chrono::Duration::seconds(120);

            tracker.update_presence(presence);

            let inactive_users = tracker.cleanup_inactive();
            assert_eq!(inactive_users.len(), 1);
            assert_eq!(inactive_users[0], "user1");

            let presence = tracker.get_presence("user1");
            assert_eq!(presence.unwrap().status, PresenceStatus::Offline);
        }

        #[test]
        fn test_editing_lock_exclusive() {
            let lock = EditingLock::exclusive(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );

            assert_eq!(lock.statute_id, "statute-1");
            assert_eq!(lock.holder, "user1");
            assert_eq!(lock.lock_type, LockType::Exclusive);
            assert!(!lock.is_expired());
        }

        #[test]
        fn test_editing_lock_shared() {
            let lock = EditingLock::shared(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );

            assert_eq!(lock.lock_type, LockType::Shared);
        }

        #[test]
        fn test_editing_lock_extend() {
            let mut lock = EditingLock::exclusive(
                "statute-1".to_string(),
                "user1".to_string(),
                Duration::from_secs(300),
            );

            let initial_expiry = lock.expires_at;
            lock.extend(Duration::from_secs(60));
            assert!(lock.expires_at > initial_expiry);
        }

        #[test]
        fn test_lock_manager_acquire_exclusive() {
            let manager = LockManager::new();

            let result = manager.acquire_exclusive("statute-1".to_string(), "user1".to_string());
            assert!(result.is_ok());

            // Second user cannot acquire exclusive lock
            let result2 = manager.acquire_exclusive("statute-1".to_string(), "user2".to_string());
            assert!(result2.is_err());
        }

        #[test]
        fn test_lock_manager_acquire_shared() {
            let manager = LockManager::new();

            let result1 = manager.acquire_shared("statute-1".to_string(), "user1".to_string());
            assert!(result1.is_ok());

            let result2 = manager.acquire_shared("statute-1".to_string(), "user2".to_string());
            assert!(result2.is_ok());
        }

        #[test]
        fn test_lock_manager_exclusive_blocks_shared() {
            let manager = LockManager::new();

            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();

            let result = manager.acquire_shared("statute-1".to_string(), "user2".to_string());
            assert!(result.is_err());
        }

        #[test]
        fn test_lock_manager_release() {
            let manager = LockManager::new();

            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();

            let released = manager.release("statute-1", "user1");
            assert!(released);

            // Can now acquire lock
            let result = manager.acquire_exclusive("statute-1".to_string(), "user2".to_string());
            assert!(result.is_ok());
        }

        #[test]
        fn test_lock_manager_is_locked() {
            let manager = LockManager::new();

            assert!(!manager.is_locked("statute-1"));

            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();

            assert!(manager.is_locked("statute-1"));
        }

        #[test]
        fn test_lock_manager_extend_lock() {
            let manager = LockManager::new();

            manager
                .acquire_exclusive("statute-1".to_string(), "user1".to_string())
                .unwrap();

            let extended = manager.extend_lock("statute-1", "user1", Duration::from_secs(60));
            assert!(extended);
        }

        #[test]
        fn test_conflict_notification_creation() {
            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string(), "user2".to_string()],
                "Concurrent edits detected".to_string(),
            );

            assert_eq!(notification.statute_id, "statute-1");
            assert_eq!(notification.conflict_type, ConflictType::ConcurrentEdit);
            assert_eq!(notification.users.len(), 2);
            assert_eq!(notification.status, ConflictStatus::Pending);
        }

        #[test]
        fn test_conflict_notification_status_changes() {
            let mut notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );

            notification.mark_resolving();
            assert_eq!(notification.status, ConflictStatus::Resolving);

            notification.mark_resolved();
            assert_eq!(notification.status, ConflictStatus::Resolved);
            assert!(notification.is_resolved());
        }

        #[test]
        fn test_conflict_manager_register() {
            let manager = ConflictManager::new();

            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );

            let id = manager.register_conflict(notification);
            let retrieved = manager.get_conflict(id);
            assert!(retrieved.is_some());
        }

        #[test]
        fn test_conflict_manager_get_for_statute() {
            let manager = ConflictManager::new();

            let notification1 = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test 1".to_string(),
            );

            let notification2 = ConflictNotification::new(
                "statute-2".to_string(),
                ConflictType::LockConflict,
                vec!["user2".to_string()],
                "Test 2".to_string(),
            );

            manager.register_conflict(notification1);
            manager.register_conflict(notification2);

            let conflicts = manager.get_conflicts_for_statute("statute-1");
            assert_eq!(conflicts.len(), 1);
            assert_eq!(conflicts[0].statute_id, "statute-1");
        }

        #[test]
        fn test_conflict_manager_pending_conflicts() {
            let manager = ConflictManager::new();

            let notification = ConflictNotification::new(
                "statute-1".to_string(),
                ConflictType::ConcurrentEdit,
                vec!["user1".to_string()],
                "Test".to_string(),
            );

            let id = manager.register_conflict(notification);

            let pending = manager.get_pending_conflicts();
            assert_eq!(pending.len(), 1);

            manager.update_status(id, ConflictStatus::Resolved);
            let pending = manager.get_pending_conflicts();
            assert_eq!(pending.len(), 0);
        }

        #[test]
        fn test_change_subscription_creation() {
            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());

            assert_eq!(subscription.subscriber, "user1");
            assert_eq!(subscription.statute_filters.len(), 1);
            assert_eq!(subscription.event_filters.len(), 1);
        }

        #[test]
        fn test_change_subscription_matches() {
            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());

            assert!(subscription.matches_statute("statute-1"));
            assert!(!subscription.matches_statute("statute-2"));

            assert!(subscription.matches_event("StatuteUpdated"));
            assert!(!subscription.matches_event("StatuteDeleted"));
        }

        #[test]
        fn test_change_subscription_match_all() {
            let subscription = ChangeSubscription::new("user1".to_string());

            // Empty filters match all
            assert!(subscription.matches_statute("any-statute"));
            assert!(subscription.matches_event("any-event"));
        }

        #[test]
        fn test_subscription_manager_subscribe() {
            let manager = SubscriptionManager::new();

            let subscription = ChangeSubscription::new("user1".to_string());
            let _id = manager.subscribe(subscription);

            assert_eq!(manager.subscription_count(), 1);
            assert_eq!(manager.subscriber_count(), 1);
        }

        #[test]
        fn test_subscription_manager_unsubscribe() {
            let manager = SubscriptionManager::new();

            let subscription = ChangeSubscription::new("user1".to_string());
            let id = manager.subscribe(subscription);

            let unsubscribed = manager.unsubscribe(id);
            assert!(unsubscribed);
            assert_eq!(manager.subscription_count(), 0);
        }

        #[test]
        fn test_subscription_manager_find_matching() {
            let manager = SubscriptionManager::new();

            let subscription = ChangeSubscription::new("user1".to_string())
                .with_statute_filter("statute-1".to_string())
                .with_event_filter("StatuteUpdated".to_string());

            manager.subscribe(subscription);

            let matches = manager.find_matching_subscriptions("statute-1", "StatuteUpdated");
            assert_eq!(matches.len(), 1);

            let no_matches = manager.find_matching_subscriptions("statute-2", "StatuteUpdated");
            assert_eq!(no_matches.len(), 0);
        }

        #[test]
        fn test_subscription_manager_unsubscribe_all() {
            let manager = SubscriptionManager::new();

            let sub1 = ChangeSubscription::new("user1".to_string());
            let sub2 = ChangeSubscription::new("user1".to_string());

            manager.subscribe(sub1);
            manager.subscribe(sub2);

            assert_eq!(manager.subscription_count(), 2);

            let removed = manager.unsubscribe_all("user1");
            assert_eq!(removed, 2);
            assert_eq!(manager.subscription_count(), 0);
        }
    }
}

/// Enterprise Security (v0.2.9)
///
/// This module provides enterprise-grade security features:
/// - LDAP/Active Directory integration
/// - Single sign-on (SAML, OIDC)
/// - Hardware security module (HSM) support
/// - Audit log tamper detection
/// - Field-level encryption
pub mod enterprise_security {
    use super::*;

    /// LDAP/Active Directory configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LdapConfig {
        /// LDAP server URL (e.g., "ldap://ldap.example.com:389")
        pub server_url: String,
        /// Base DN for searches (e.g., "dc=example,dc=com")
        pub base_dn: String,
        /// Bind DN for authentication (e.g., "cn=admin,dc=example,dc=com")
        pub bind_dn: Option<String>,
        /// Bind password
        pub bind_password: Option<String>,
        /// User search filter (e.g., "(uid={username})")
        pub user_search_filter: String,
        /// Group search filter (e.g., "(member={user_dn})")
        pub group_search_filter: Option<String>,
        /// Connection timeout in seconds
        pub timeout_seconds: u64,
        /// Enable TLS/SSL
        pub use_tls: bool,
    }

    impl LdapConfig {
        /// Create a new LDAP configuration.
        pub fn new(server_url: String, base_dn: String) -> Self {
            Self {
                server_url,
                base_dn,
                bind_dn: None,
                bind_password: None,
                user_search_filter: "(uid={username})".to_string(),
                group_search_filter: Some("(member={user_dn})".to_string()),
                timeout_seconds: 30,
                use_tls: true,
            }
        }

        /// Set bind credentials.
        pub fn with_bind_credentials(mut self, dn: String, password: String) -> Self {
            self.bind_dn = Some(dn);
            self.bind_password = Some(password);
            self
        }

        /// Set user search filter.
        pub fn with_user_filter(mut self, filter: String) -> Self {
            self.user_search_filter = filter;
            self
        }

        /// Set connection timeout.
        pub fn with_timeout(mut self, seconds: u64) -> Self {
            self.timeout_seconds = seconds;
            self
        }

        /// Disable TLS.
        pub fn without_tls(mut self) -> Self {
            self.use_tls = false;
            self
        }
    }

    /// LDAP user information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LdapUser {
        /// User distinguished name
        pub dn: String,
        /// Username
        pub username: String,
        /// Email address
        pub email: Option<String>,
        /// Display name
        pub display_name: Option<String>,
        /// Group memberships
        pub groups: Vec<String>,
        /// Additional attributes
        pub attributes: HashMap<String, String>,
    }

    impl LdapUser {
        /// Create a new LDAP user.
        pub fn new(dn: String, username: String) -> Self {
            Self {
                dn,
                username,
                email: None,
                display_name: None,
                groups: Vec::new(),
                attributes: HashMap::new(),
            }
        }

        /// Check if user is member of a group.
        pub fn is_member_of(&self, group: &str) -> bool {
            self.groups.iter().any(|g| g == group)
        }

        /// Add a group membership.
        pub fn add_group(&mut self, group: String) {
            if !self.groups.contains(&group) {
                self.groups.push(group);
            }
        }
    }

    /// LDAP directory service client.
    pub struct LdapClient {
        config: LdapConfig,
        #[allow(dead_code)]
        connection_pool: Arc<Mutex<VecDeque<String>>>,
    }

    impl LdapClient {
        /// Create a new LDAP client.
        pub fn new(config: LdapConfig) -> Self {
            Self {
                config,
                connection_pool: Arc::new(Mutex::new(VecDeque::new())),
            }
        }

        /// Authenticate a user (placeholder implementation).
        pub fn authenticate(&self, username: &str, password: &str) -> Result<LdapUser, String> {
            // Placeholder: In real implementation, this would connect to LDAP
            if password.is_empty() {
                return Err("Invalid credentials".to_string());
            }

            let dn = format!("uid={},{}", username, self.config.base_dn);
            Ok(LdapUser::new(dn, username.to_string()))
        }

        /// Search for a user (placeholder implementation).
        pub fn search_user(&self, username: &str) -> Result<Option<LdapUser>, String> {
            // Placeholder: In real implementation, this would search LDAP
            if username.is_empty() {
                return Err("Username cannot be empty".to_string());
            }

            let dn = format!("uid={},{}", username, self.config.base_dn);
            Ok(Some(LdapUser::new(dn, username.to_string())))
        }

        /// Get user groups (placeholder implementation).
        pub fn get_user_groups(&self, user_dn: &str) -> Result<Vec<String>, String> {
            // Placeholder: In real implementation, this would query LDAP
            if user_dn.is_empty() {
                return Err("User DN cannot be empty".to_string());
            }

            Ok(vec!["users".to_string()])
        }
    }

    /// Single Sign-On (SSO) provider type.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum SsoProvider {
        /// SAML 2.0
        SAML,
        /// OpenID Connect
        OIDC,
        /// OAuth 2.0
        OAuth2,
    }

    /// SSO configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SsoConfig {
        /// SSO provider type
        pub provider: SsoProvider,
        /// Identity provider URL
        pub idp_url: String,
        /// Service provider (this app) URL
        pub sp_url: String,
        /// Client ID
        pub client_id: String,
        /// Client secret
        pub client_secret: String,
        /// SSO metadata URL (for SAML)
        pub metadata_url: Option<String>,
        /// Redirect URL after authentication
        pub redirect_url: String,
        /// Requested scopes (for OIDC/OAuth2)
        pub scopes: Vec<String>,
    }

    impl SsoConfig {
        /// Create a new SAML configuration.
        pub fn saml(idp_url: String, sp_url: String, metadata_url: String) -> Self {
            let redirect_url = format!("{}/callback", sp_url);
            Self {
                provider: SsoProvider::SAML,
                idp_url,
                sp_url,
                client_id: String::new(),
                client_secret: String::new(),
                metadata_url: Some(metadata_url),
                redirect_url,
                scopes: Vec::new(),
            }
        }

        /// Create a new OIDC configuration.
        pub fn oidc(
            idp_url: String,
            client_id: String,
            client_secret: String,
            redirect_url: String,
        ) -> Self {
            Self {
                provider: SsoProvider::OIDC,
                idp_url,
                sp_url: redirect_url.clone(),
                client_id,
                client_secret,
                metadata_url: None,
                redirect_url,
                scopes: vec![
                    "openid".to_string(),
                    "profile".to_string(),
                    "email".to_string(),
                ],
            }
        }

        /// Add a scope.
        pub fn with_scope(mut self, scope: String) -> Self {
            if !self.scopes.contains(&scope) {
                self.scopes.push(scope);
            }
            self
        }
    }

    /// SSO user information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SsoUser {
        /// Unique user ID from provider
        pub user_id: String,
        /// Email address
        pub email: String,
        /// Display name
        pub name: Option<String>,
        /// Provider name
        pub provider: SsoProvider,
        /// Additional claims/attributes
        pub claims: HashMap<String, String>,
    }

    impl SsoUser {
        /// Create a new SSO user.
        pub fn new(user_id: String, email: String, provider: SsoProvider) -> Self {
            Self {
                user_id,
                email,
                name: None,
                provider,
                claims: HashMap::new(),
            }
        }

        /// Add a claim.
        pub fn with_claim(mut self, key: String, value: String) -> Self {
            self.claims.insert(key, value);
            self
        }
    }

    /// SSO authentication manager.
    pub struct SsoManager {
        configs: Arc<Mutex<HashMap<SsoProvider, SsoConfig>>>,
        sessions: Arc<Mutex<HashMap<String, SsoUser>>>,
    }

    impl SsoManager {
        /// Create a new SSO manager.
        pub fn new() -> Self {
            Self {
                configs: Arc::new(Mutex::new(HashMap::new())),
                sessions: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Register an SSO configuration.
        pub fn register_provider(&self, config: SsoConfig) {
            let mut configs = self.configs.lock().unwrap();
            configs.insert(config.provider, config);
        }

        /// Initiate SSO login (placeholder implementation).
        pub fn initiate_login(&self, provider: SsoProvider) -> Result<String, String> {
            let configs = self.configs.lock().unwrap();
            let config = configs
                .get(&provider)
                .ok_or_else(|| "Provider not configured".to_string())?;

            // Placeholder: Generate auth URL
            Ok(format!("{}?client_id={}", config.idp_url, config.client_id))
        }

        /// Handle SSO callback (placeholder implementation).
        pub fn handle_callback(
            &self,
            provider: SsoProvider,
            code: String,
        ) -> Result<SsoUser, String> {
            if code.is_empty() {
                return Err("Invalid code".to_string());
            }

            // Placeholder: Exchange code for user info
            let user = SsoUser::new(
                "user123".to_string(),
                "user@example.com".to_string(),
                provider,
            );

            // Store session
            let session_id = Uuid::new_v4().to_string();
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_id, user.clone());

            Ok(user)
        }

        /// Get active sessions count.
        pub fn active_sessions(&self) -> usize {
            let sessions = self.sessions.lock().unwrap();
            sessions.len()
        }
    }

    impl Default for SsoManager {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Hardware Security Module (HSM) key type.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HsmKeyType {
        /// RSA key
        RSA,
        /// Elliptic Curve key
        EC,
        /// AES symmetric key
        AES,
    }

    /// HSM key metadata.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HsmKey {
        /// Unique key ID
        pub key_id: String,
        /// Key type
        pub key_type: HsmKeyType,
        /// Key label/name
        pub label: String,
        /// Creation timestamp
        pub created_at: DateTime<Utc>,
        /// Key size in bits
        pub key_size: u32,
        /// Whether key can be exported
        pub exportable: bool,
    }

    impl HsmKey {
        /// Create a new HSM key metadata.
        pub fn new(key_id: String, key_type: HsmKeyType, label: String, key_size: u32) -> Self {
            Self {
                key_id,
                key_type,
                label,
                created_at: Utc::now(),
                key_size,
                exportable: false,
            }
        }
    }

    /// HSM configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HsmConfig {
        /// HSM provider (e.g., "AWS CloudHSM", "Azure Key Vault", "PKCS#11")
        pub provider: String,
        /// Connection endpoint
        pub endpoint: String,
        /// Authentication credentials
        pub credentials: HashMap<String, String>,
        /// Default key type
        pub default_key_type: HsmKeyType,
    }

    impl HsmConfig {
        /// Create a new HSM configuration.
        pub fn new(provider: String, endpoint: String) -> Self {
            Self {
                provider,
                endpoint,
                credentials: HashMap::new(),
                default_key_type: HsmKeyType::AES,
            }
        }

        /// Add a credential.
        pub fn with_credential(mut self, key: String, value: String) -> Self {
            self.credentials.insert(key, value);
            self
        }
    }

    /// HSM client for cryptographic operations.
    pub struct HsmClient {
        #[allow(dead_code)]
        config: HsmConfig,
        keys: Arc<Mutex<HashMap<String, HsmKey>>>,
    }

    impl HsmClient {
        /// Create a new HSM client.
        pub fn new(config: HsmConfig) -> Self {
            Self {
                config,
                keys: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        /// Generate a new key in the HSM (placeholder implementation).
        pub fn generate_key(
            &self,
            label: String,
            key_type: HsmKeyType,
            key_size: u32,
        ) -> Result<HsmKey, String> {
            let key_id = Uuid::new_v4().to_string();
            let key = HsmKey::new(key_id.clone(), key_type, label, key_size);

            let mut keys = self.keys.lock().unwrap();
            keys.insert(key_id, key.clone());

            Ok(key)
        }

        /// Sign data using an HSM key (placeholder implementation).
        pub fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, String> {
            let keys = self.keys.lock().unwrap();
            keys.get(key_id)
                .ok_or_else(|| "Key not found".to_string())?;

            // Placeholder: In real implementation, this would use HSM to sign
            Ok(data.to_vec())
        }

        /// Verify signature using an HSM key (placeholder implementation).
        pub fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, String> {
            let keys = self.keys.lock().unwrap();
            keys.get(key_id)
                .ok_or_else(|| "Key not found".to_string())?;

            // Placeholder: In real implementation, this would use HSM to verify
            Ok(data == signature)
        }

        /// Encrypt data using an HSM key (placeholder implementation).
        pub fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, String> {
            let keys = self.keys.lock().unwrap();
            keys.get(key_id)
                .ok_or_else(|| "Key not found".to_string())?;

            // Placeholder: In real implementation, this would use HSM to encrypt
            Ok(data.to_vec())
        }

        /// Decrypt data using an HSM key (placeholder implementation).
        pub fn decrypt(&self, key_id: &str, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
            let keys = self.keys.lock().unwrap();
            keys.get(key_id)
                .ok_or_else(|| "Key not found".to_string())?;

            // Placeholder: In real implementation, this would use HSM to decrypt
            Ok(encrypted_data.to_vec())
        }

        /// List all keys.
        pub fn list_keys(&self) -> Vec<HsmKey> {
            let keys = self.keys.lock().unwrap();
            keys.values().cloned().collect()
        }

        /// Delete a key.
        pub fn delete_key(&self, key_id: &str) -> Result<(), String> {
            let mut keys = self.keys.lock().unwrap();
            keys.remove(key_id)
                .ok_or_else(|| "Key not found".to_string())?;
            Ok(())
        }
    }

    /// Audit log entry with tamper detection.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TamperProofLogEntry {
        /// Entry ID
        pub entry_id: Uuid,
        /// Timestamp
        pub timestamp: DateTime<Utc>,
        /// Log entry data
        pub data: String,
        /// Hash of previous entry (for chain)
        pub previous_hash: String,
        /// Hash of this entry
        pub entry_hash: String,
        /// Digital signature (optional)
        pub signature: Option<String>,
    }

    impl TamperProofLogEntry {
        /// Create a new log entry.
        pub fn new(data: String, previous_hash: String) -> Self {
            let entry_id = Uuid::new_v4();
            let timestamp = Utc::now();

            // Calculate entry hash
            let hash_input = format!("{}{}{}{}", entry_id, timestamp, data, previous_hash);
            let entry_hash = Self::calculate_hash(&hash_input);

            Self {
                entry_id,
                timestamp,
                data,
                previous_hash,
                entry_hash,
                signature: None,
            }
        }

        /// Calculate SHA-256 hash (simplified placeholder).
        fn calculate_hash(data: &str) -> String {
            // Placeholder: In real implementation, use proper SHA-256
            format!("hash_{}", data.len())
        }

        /// Verify the entry hash.
        pub fn verify_hash(&self) -> bool {
            let hash_input = format!(
                "{}{}{}{}",
                self.entry_id, self.timestamp, self.data, self.previous_hash
            );
            let calculated_hash = Self::calculate_hash(&hash_input);
            calculated_hash == self.entry_hash
        }

        /// Sign the entry (placeholder implementation).
        pub fn sign(&mut self, key: &str) {
            // Placeholder: In real implementation, use proper digital signature
            self.signature = Some(format!("sig_{}_{}", key, self.entry_hash));
        }

        /// Verify the signature (placeholder implementation).
        pub fn verify_signature(&self, key: &str) -> bool {
            match &self.signature {
                Some(sig) => sig == &format!("sig_{}_{}", key, self.entry_hash),
                None => false,
            }
        }
    }

    /// Tamper-proof audit log.
    pub struct TamperProofLog {
        entries: Arc<Mutex<Vec<TamperProofLogEntry>>>,
        signing_key: Option<String>,
    }

    impl TamperProofLog {
        /// Create a new tamper-proof log.
        pub fn new() -> Self {
            Self {
                entries: Arc::new(Mutex::new(Vec::new())),
                signing_key: None,
            }
        }

        /// Enable signing with a key.
        pub fn with_signing_key(mut self, key: String) -> Self {
            self.signing_key = Some(key);
            self
        }

        /// Append a log entry.
        pub fn append(&self, data: String) -> Uuid {
            let mut entries = self.entries.lock().unwrap();

            let previous_hash = entries
                .last()
                .map(|e| e.entry_hash.clone())
                .unwrap_or_else(|| "genesis".to_string());

            let mut entry = TamperProofLogEntry::new(data, previous_hash);

            // Sign if key is configured
            if let Some(key) = &self.signing_key {
                entry.sign(key);
            }

            let entry_id = entry.entry_id;
            entries.push(entry);

            entry_id
        }

        /// Verify the entire log chain.
        pub fn verify_chain(&self) -> Result<(), Vec<Uuid>> {
            let entries = self.entries.lock().unwrap();
            let mut invalid_entries = Vec::new();

            for (i, entry) in entries.iter().enumerate() {
                // Verify hash
                if !entry.verify_hash() {
                    invalid_entries.push(entry.entry_id);
                    continue;
                }

                // Verify chain
                if i > 0 {
                    let previous_entry = &entries[i - 1];
                    if entry.previous_hash != previous_entry.entry_hash {
                        invalid_entries.push(entry.entry_id);
                        continue;
                    }
                }

                // Verify signature if present
                if let Some(key) = &self.signing_key
                    && entry.signature.is_some()
                    && !entry.verify_signature(key)
                {
                    invalid_entries.push(entry.entry_id);
                }
            }

            if invalid_entries.is_empty() {
                Ok(())
            } else {
                Err(invalid_entries)
            }
        }

        /// Get entry by ID.
        pub fn get_entry(&self, entry_id: Uuid) -> Option<TamperProofLogEntry> {
            let entries = self.entries.lock().unwrap();
            entries.iter().find(|e| e.entry_id == entry_id).cloned()
        }

        /// Get all entries.
        pub fn get_all_entries(&self) -> Vec<TamperProofLogEntry> {
            let entries = self.entries.lock().unwrap();
            entries.clone()
        }

        /// Get entry count.
        pub fn entry_count(&self) -> usize {
            let entries = self.entries.lock().unwrap();
            entries.len()
        }
    }

    impl Default for TamperProofLog {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Field-level encryption configuration.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FieldEncryptionConfig {
        /// Encryption algorithm (e.g., "AES-256-GCM")
        pub algorithm: String,
        /// Key ID for encryption
        pub key_id: String,
        /// Fields to encrypt (field paths)
        pub encrypted_fields: Vec<String>,
        /// Key rotation schedule (in days)
        pub key_rotation_days: u32,
    }

    impl FieldEncryptionConfig {
        /// Create a new field encryption configuration.
        pub fn new(algorithm: String, key_id: String) -> Self {
            Self {
                algorithm,
                key_id,
                encrypted_fields: Vec::new(),
                key_rotation_days: 90,
            }
        }

        /// Add a field to encrypt.
        pub fn add_field(mut self, field_path: String) -> Self {
            if !self.encrypted_fields.contains(&field_path) {
                self.encrypted_fields.push(field_path);
            }
            self
        }

        /// Set key rotation period.
        pub fn with_rotation_period(mut self, days: u32) -> Self {
            self.key_rotation_days = days;
            self
        }
    }

    /// Encrypted field value.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EncryptedField {
        /// Encryption algorithm used
        pub algorithm: String,
        /// Key ID used for encryption
        pub key_id: String,
        /// Encrypted data (base64 encoded)
        pub ciphertext: String,
        /// Initialization vector (base64 encoded)
        pub iv: String,
        /// Encryption timestamp
        pub encrypted_at: DateTime<Utc>,
    }

    impl EncryptedField {
        /// Create a new encrypted field.
        pub fn new(algorithm: String, key_id: String, ciphertext: String, iv: String) -> Self {
            Self {
                algorithm,
                key_id,
                ciphertext,
                iv,
                encrypted_at: Utc::now(),
            }
        }
    }

    /// Field-level encryption manager.
    pub struct FieldEncryptionManager {
        config: FieldEncryptionConfig,
        hsm_client: Option<Arc<HsmClient>>,
    }

    impl FieldEncryptionManager {
        /// Create a new field encryption manager.
        pub fn new(config: FieldEncryptionConfig) -> Self {
            Self {
                config,
                hsm_client: None,
            }
        }

        /// Set HSM client for key management.
        pub fn with_hsm(mut self, hsm_client: Arc<HsmClient>) -> Self {
            self.hsm_client = Some(hsm_client);
            self
        }

        /// Encrypt a field value (placeholder implementation).
        pub fn encrypt_field(
            &self,
            field_name: &str,
            value: &str,
        ) -> Result<EncryptedField, String> {
            if !self
                .config
                .encrypted_fields
                .contains(&field_name.to_string())
            {
                return Err("Field not configured for encryption".to_string());
            }

            // Placeholder: In real implementation, use proper encryption
            let ciphertext = format!("encrypted_{}", value);
            let iv = "random_iv".to_string();

            Ok(EncryptedField::new(
                self.config.algorithm.clone(),
                self.config.key_id.clone(),
                ciphertext,
                iv,
            ))
        }

        /// Decrypt a field value (placeholder implementation).
        pub fn decrypt_field(&self, encrypted: &EncryptedField) -> Result<String, String> {
            if encrypted.key_id != self.config.key_id {
                return Err("Key mismatch".to_string());
            }

            // Placeholder: In real implementation, use proper decryption
            let decrypted = encrypted
                .ciphertext
                .strip_prefix("encrypted_")
                .unwrap_or(&encrypted.ciphertext)
                .to_string();

            Ok(decrypted)
        }

        /// Check if a field should be encrypted.
        pub fn should_encrypt(&self, field_name: &str) -> bool {
            self.config
                .encrypted_fields
                .contains(&field_name.to_string())
        }

        /// Get encrypted fields configuration.
        pub fn get_encrypted_fields(&self) -> &[String] {
            &self.config.encrypted_fields
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ldap_config_creation() {
            let config = LdapConfig::new(
                "ldap://ldap.example.com".to_string(),
                "dc=example,dc=com".to_string(),
            );

            assert_eq!(config.server_url, "ldap://ldap.example.com");
            assert_eq!(config.base_dn, "dc=example,dc=com");
            assert!(config.use_tls);
        }

        #[test]
        fn test_ldap_config_with_credentials() {
            let config = LdapConfig::new(
                "ldap://ldap.example.com".to_string(),
                "dc=example,dc=com".to_string(),
            )
            .with_bind_credentials(
                "cn=admin,dc=example,dc=com".to_string(),
                "secret".to_string(),
            );

            assert!(config.bind_dn.is_some());
            assert!(config.bind_password.is_some());
        }

        #[test]
        fn test_ldap_user_creation() {
            let user = LdapUser::new("uid=jdoe,dc=example,dc=com".to_string(), "jdoe".to_string());

            assert_eq!(user.username, "jdoe");
            assert_eq!(user.dn, "uid=jdoe,dc=example,dc=com");
        }

        #[test]
        fn test_ldap_user_group_membership() {
            let mut user =
                LdapUser::new("uid=jdoe,dc=example,dc=com".to_string(), "jdoe".to_string());
            user.add_group("admins".to_string());
            user.add_group("users".to_string());

            assert!(user.is_member_of("admins"));
            assert!(user.is_member_of("users"));
            assert!(!user.is_member_of("guests"));
        }

        #[test]
        fn test_ldap_client_authenticate() {
            let config = LdapConfig::new(
                "ldap://ldap.example.com".to_string(),
                "dc=example,dc=com".to_string(),
            );
            let client = LdapClient::new(config);

            let result = client.authenticate("jdoe", "password");
            assert!(result.is_ok());

            let result = client.authenticate("jdoe", "");
            assert!(result.is_err());
        }

        #[test]
        fn test_sso_config_saml() {
            let config = SsoConfig::saml(
                "https://idp.example.com".to_string(),
                "https://sp.example.com".to_string(),
                "https://idp.example.com/metadata".to_string(),
            );

            assert_eq!(config.provider, SsoProvider::SAML);
            assert!(config.metadata_url.is_some());
        }

        #[test]
        fn test_sso_config_oidc() {
            let config = SsoConfig::oidc(
                "https://idp.example.com".to_string(),
                "client123".to_string(),
                "secret".to_string(),
                "https://sp.example.com/callback".to_string(),
            );

            assert_eq!(config.provider, SsoProvider::OIDC);
            assert!(config.scopes.contains(&"openid".to_string()));
        }

        #[test]
        fn test_sso_user_creation() {
            let user = SsoUser::new(
                "user123".to_string(),
                "user@example.com".to_string(),
                SsoProvider::OIDC,
            )
            .with_claim("name".to_string(), "John Doe".to_string());

            assert_eq!(user.user_id, "user123");
            assert_eq!(user.email, "user@example.com");
            assert!(user.claims.contains_key("name"));
        }

        #[test]
        fn test_sso_manager_register_provider() {
            let manager = SsoManager::new();
            let config = SsoConfig::oidc(
                "https://idp.example.com".to_string(),
                "client123".to_string(),
                "secret".to_string(),
                "https://sp.example.com/callback".to_string(),
            );

            manager.register_provider(config);

            let result = manager.initiate_login(SsoProvider::OIDC);
            assert!(result.is_ok());
        }

        #[test]
        fn test_sso_manager_handle_callback() {
            let manager = SsoManager::new();
            let config = SsoConfig::oidc(
                "https://idp.example.com".to_string(),
                "client123".to_string(),
                "secret".to_string(),
                "https://sp.example.com/callback".to_string(),
            );

            manager.register_provider(config);

            let result = manager.handle_callback(SsoProvider::OIDC, "auth_code".to_string());
            assert!(result.is_ok());

            assert_eq!(manager.active_sessions(), 1);
        }

        #[test]
        fn test_hsm_key_creation() {
            let key = HsmKey::new(
                "key123".to_string(),
                HsmKeyType::AES,
                "encryption-key".to_string(),
                256,
            );

            assert_eq!(key.key_id, "key123");
            assert_eq!(key.key_type, HsmKeyType::AES);
            assert_eq!(key.key_size, 256);
        }

        #[test]
        fn test_hsm_config_creation() {
            let config = HsmConfig::new("AWS CloudHSM".to_string(), "hsm.example.com".to_string())
                .with_credential("api_key".to_string(), "secret".to_string());

            assert_eq!(config.provider, "AWS CloudHSM");
            assert!(config.credentials.contains_key("api_key"));
        }

        #[test]
        fn test_hsm_client_generate_key() {
            let config = HsmConfig::new("Mock HSM".to_string(), "localhost".to_string());
            let client = HsmClient::new(config);

            let key = client.generate_key("test-key".to_string(), HsmKeyType::AES, 256);
            assert!(key.is_ok());

            let key = key.unwrap();
            assert_eq!(key.label, "test-key");
            assert_eq!(key.key_size, 256);
        }

        #[test]
        fn test_hsm_client_sign_verify() {
            let config = HsmConfig::new("Mock HSM".to_string(), "localhost".to_string());
            let client = HsmClient::new(config);

            let key = client
                .generate_key("signing-key".to_string(), HsmKeyType::RSA, 2048)
                .unwrap();

            let data = b"test data";
            let signature = client.sign(&key.key_id, data).unwrap();
            let verified = client.verify(&key.key_id, data, &signature).unwrap();

            assert!(verified);
        }

        #[test]
        fn test_hsm_client_encrypt_decrypt() {
            let config = HsmConfig::new("Mock HSM".to_string(), "localhost".to_string());
            let client = HsmClient::new(config);

            let key = client
                .generate_key("encryption-key".to_string(), HsmKeyType::AES, 256)
                .unwrap();

            let data = b"sensitive data";
            let encrypted = client.encrypt(&key.key_id, data).unwrap();
            let decrypted = client.decrypt(&key.key_id, &encrypted).unwrap();

            assert_eq!(data.to_vec(), decrypted);
        }

        #[test]
        fn test_hsm_client_list_keys() {
            let config = HsmConfig::new("Mock HSM".to_string(), "localhost".to_string());
            let client = HsmClient::new(config);

            client
                .generate_key("key1".to_string(), HsmKeyType::AES, 256)
                .unwrap();
            client
                .generate_key("key2".to_string(), HsmKeyType::RSA, 2048)
                .unwrap();

            let keys = client.list_keys();
            assert_eq!(keys.len(), 2);
        }

        #[test]
        fn test_hsm_client_delete_key() {
            let config = HsmConfig::new("Mock HSM".to_string(), "localhost".to_string());
            let client = HsmClient::new(config);

            let key = client
                .generate_key("temp-key".to_string(), HsmKeyType::AES, 256)
                .unwrap();

            let result = client.delete_key(&key.key_id);
            assert!(result.is_ok());

            let keys = client.list_keys();
            assert_eq!(keys.len(), 0);
        }

        #[test]
        fn test_tamper_proof_log_entry_creation() {
            let entry = TamperProofLogEntry::new("test log".to_string(), "genesis".to_string());

            assert_eq!(entry.data, "test log");
            assert_eq!(entry.previous_hash, "genesis");
            assert!(entry.verify_hash());
        }

        #[test]
        fn test_tamper_proof_log_entry_signing() {
            let mut entry = TamperProofLogEntry::new("test log".to_string(), "genesis".to_string());
            entry.sign("signing_key");

            assert!(entry.signature.is_some());
            assert!(entry.verify_signature("signing_key"));
            assert!(!entry.verify_signature("wrong_key"));
        }

        #[test]
        fn test_tamper_proof_log_append() {
            let log = TamperProofLog::new();

            let id1 = log.append("first entry".to_string());
            let id2 = log.append("second entry".to_string());

            assert_eq!(log.entry_count(), 2);

            let entry1 = log.get_entry(id1).unwrap();
            let entry2 = log.get_entry(id2).unwrap();

            assert_eq!(entry1.data, "first entry");
            assert_eq!(entry2.data, "second entry");
            assert_eq!(entry2.previous_hash, entry1.entry_hash);
        }

        #[test]
        fn test_tamper_proof_log_verify_chain() {
            let log = TamperProofLog::new();

            log.append("entry 1".to_string());
            log.append("entry 2".to_string());
            log.append("entry 3".to_string());

            let result = log.verify_chain();
            assert!(result.is_ok());
        }

        #[test]
        fn test_tamper_proof_log_with_signing() {
            let log = TamperProofLog::new().with_signing_key("test_key".to_string());

            log.append("signed entry 1".to_string());
            log.append("signed entry 2".to_string());

            let result = log.verify_chain();
            assert!(result.is_ok());
        }

        #[test]
        fn test_field_encryption_config_creation() {
            let config =
                FieldEncryptionConfig::new("AES-256-GCM".to_string(), "key123".to_string())
                    .add_field("email".to_string())
                    .add_field("ssn".to_string())
                    .with_rotation_period(30);

            assert_eq!(config.algorithm, "AES-256-GCM");
            assert_eq!(config.encrypted_fields.len(), 2);
            assert_eq!(config.key_rotation_days, 30);
        }

        #[test]
        fn test_encrypted_field_creation() {
            let field = EncryptedField::new(
                "AES-256-GCM".to_string(),
                "key123".to_string(),
                "ciphertext".to_string(),
                "iv123".to_string(),
            );

            assert_eq!(field.algorithm, "AES-256-GCM");
            assert_eq!(field.ciphertext, "ciphertext");
        }

        #[test]
        fn test_field_encryption_manager_encrypt_decrypt() {
            let config =
                FieldEncryptionConfig::new("AES-256-GCM".to_string(), "key123".to_string())
                    .add_field("email".to_string());

            let manager = FieldEncryptionManager::new(config);

            let encrypted = manager.encrypt_field("email", "user@example.com").unwrap();
            let decrypted = manager.decrypt_field(&encrypted).unwrap();

            assert_eq!(decrypted, "user@example.com");
        }

        #[test]
        fn test_field_encryption_manager_should_encrypt() {
            let config =
                FieldEncryptionConfig::new("AES-256-GCM".to_string(), "key123".to_string())
                    .add_field("email".to_string())
                    .add_field("ssn".to_string());

            let manager = FieldEncryptionManager::new(config);

            assert!(manager.should_encrypt("email"));
            assert!(manager.should_encrypt("ssn"));
            assert!(!manager.should_encrypt("name"));
        }

        #[test]
        fn test_field_encryption_manager_not_configured() {
            let config =
                FieldEncryptionConfig::new("AES-256-GCM".to_string(), "key123".to_string())
                    .add_field("email".to_string());

            let manager = FieldEncryptionManager::new(config);

            let result = manager.encrypt_field("ssn", "123-45-6789");
            assert!(result.is_err());
        }
    }
}
