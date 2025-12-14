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

/// Lazy loading configuration.
#[derive(Debug, Clone, Copy)]
pub struct LazyLoadConfig {
    /// Load statute content on demand
    pub lazy_content: bool,
    /// Load version history on demand
    pub lazy_versions: bool,
    /// Load events on demand
    pub lazy_events: bool,
}

impl Default for LazyLoadConfig {
    fn default() -> Self {
        Self {
            lazy_content: false,
            lazy_versions: false,
            lazy_events: false,
        }
    }
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

    /// Returns whether this statute is currently active.
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.status == StatuteStatus::Active
            && self.effective_date.is_none_or(|d| d <= now)
            && self.expiry_date.is_none_or(|d| d > now)
    }
}

/// Status of a statute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
                | RegistryEvent::MetadataUpdated { statute_id: id, .. } => id == statute_id,
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
                    | RegistryEvent::MetadataUpdated { timestamp, .. } => timestamp,
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
                .map_or(true, |filter| filter.matches(event))
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
        }
    }
}

impl StatuteRegistry {
    /// Creates a new empty registry.
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
}
