//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Utc};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use indexmap::IndexMap;
use legalis_core::{Condition, EffectType, Statute};
use lru::LruCache;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::functions::RegistryResult;
use super::types::{
    ActivityAnalytics, CachedAnalytics, EventStore, RegistryError, StatuteArchive, WebhookManager,
};
use super::types_3::{BackupMetadata, RegistryEvent, RetentionPolicy, TagAnalytics};
use super::types_5::{
    IncrementalBackup, Pagination, RankingConfig, RegistryBackup, RetentionRule, WebhookEventFilter,
};
use super::types_6::{
    AggregationResult, DependencyGraph, RegistrySnapshot, RegistryStatistics,
    RelationshipAnalytics, SearchResult, StatuteEntry, StatuteStatus,
};
use super::types_7::{RetentionResult, SearchQuery, TemporalAnalytics};
use super::types_8::{ArchivedStatute, PagedResult, StatuteSummary};

/// The central statute registry.
pub struct StatuteRegistry {
    /// Statutes by ID (latest version)
    pub(super) statutes: IndexMap<String, StatuteEntry>,
    /// Version history: statute_id -> version -> entry
    pub(super) versions: HashMap<String, HashMap<u32, StatuteEntry>>,
    /// Index by tag
    pub(super) tag_index: HashMap<String, HashSet<String>>,
    /// Index by jurisdiction
    pub(super) jurisdiction_index: HashMap<String, HashSet<String>>,
    /// LRU cache for frequently accessed statutes
    pub(crate) cache: LruCache<String, StatuteEntry>,
    /// Fuzzy matcher for statute IDs
    pub(crate) fuzzy_matcher: SkimMatcherV2,
    /// Event store for change tracking
    pub(super) event_store: EventStore,
    /// Webhook manager for notifications
    pub(super) webhook_manager: WebhookManager,
    /// Archive for deleted/superseded statutes
    pub(super) archive: StatuteArchive,
    /// Retention policy for auto-archiving
    pub(super) retention_policy: RetentionPolicy,
    /// Analytics cache with TTL support
    pub(crate) analytics_cache: CachedAnalytics,
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
        self.versions
            .entry(statute_id.clone())
            .or_default()
            .insert(entry.version, entry.clone());
        self.statutes.insert(statute_id.clone(), entry);
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
        self.cache.pop(statute_id);
        self.versions
            .entry(statute_id.to_string())
            .or_default()
            .insert(new_version, new_entry.clone());
        self.statutes.insert(statute_id.to_string(), new_entry);
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
        if existing.etag != expected_etag {
            return Err(RegistryError::ConcurrentModification {
                expected: expected_etag.to_string(),
                actual: existing.etag.clone(),
            });
        }
        self.update(statute_id, statute)
    }
    /// Gets a statute by ID (latest version).
    pub fn get(&mut self, statute_id: &str) -> Option<StatuteEntry> {
        if let Some(cached) = self.cache.get(statute_id) {
            return Some(cached.clone());
        }
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
        self.cache.pop(statute_id);
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
        let current_etag = self
            .statutes
            .get(statute_id)
            .map(|e| e.etag.clone())
            .ok_or_else(|| RegistryError::StatuteNotFound(statute_id.to_string()))?;
        if current_etag != expected_etag {
            return Err(RegistryError::ConcurrentModification {
                expected: expected_etag.to_string(),
                actual: current_etag,
            });
        }
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
        matches.sort_by_key(|b| std::cmp::Reverse(b.0));
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
                if !query.tags.is_empty() && !query.tags.iter().any(|t| entry.tags.contains(t)) {
                    return false;
                }
                if let Some(jurisdiction) = &query.jurisdiction
                    && &entry.jurisdiction != jurisdiction
                {
                    return false;
                }
                if let Some(status) = &query.status
                    && &entry.status != status
                {
                    return false;
                }
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
        self.cache.pop(statute_id);
        for tag in &entry.tags {
            if let Some(ids) = self.tag_index.get_mut(tag) {
                ids.remove(statute_id);
                if ids.is_empty() {
                    self.tag_index.remove(tag);
                }
            }
        }
        if let Some(ids) = self.jurisdiction_index.get_mut(&entry.jurisdiction) {
            ids.remove(statute_id);
            if ids.is_empty() {
                self.jurisdiction_index.remove(&entry.jurisdiction);
            }
        }
        self.versions.remove(statute_id);
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
        let mut to_archive: Vec<(String, String)> = Vec::new();
        for (statute_id, entry) in &self.statutes {
            for rule in &self.retention_policy.rules {
                let should_archive = match rule {
                    RetentionRule::ExpiredStatutes { reason: _ } => {
                        if let Some(expiry) = entry.expiry_date {
                            expiry < now
                        } else {
                            false
                        }
                    }
                    RetentionRule::OlderThanDays { days, reason: _ } => {
                        if let Some(effective) = entry.effective_date {
                            let cutoff = now - chrono::Duration::days(*days);
                            effective < cutoff
                        } else {
                            false
                        }
                    }
                    RetentionRule::ByStatus { status, reason: _ } => entry.status == *status,
                    RetentionRule::SupersededStatutes { reason: _ } => !entry.supersedes.is_empty(),
                    RetentionRule::InactiveForDays { days, reason: _ } => {
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
                    break;
                }
            }
        }
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
        self.build_dependency_graph(statute_id, &mut graph.dependencies, &mut HashSet::new());
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
        let title_lower = entry.statute.title.to_lowercase();
        if title_lower == query {
            score += config.title_weight * config.exact_match_boost;
        } else if title_lower.contains(query) {
            score += config.title_weight;
        }
        let id_lower = entry.statute.id.to_lowercase();
        if id_lower == query {
            score += config.id_weight * config.exact_match_boost;
        } else if id_lower.contains(query) {
            score += config.id_weight;
        }
        for tag in &entry.tags {
            let tag_lower = tag.to_lowercase();
            if tag_lower == query {
                score += config.tag_weight * config.exact_match_boost;
            } else if tag_lower.contains(query) {
                score += config.tag_weight;
            }
        }
        let jurisdiction_lower = entry.jurisdiction.to_lowercase();
        if jurisdiction_lower == query {
            score += config.jurisdiction_weight * config.exact_match_boost;
        } else if jurisdiction_lower.contains(query) {
            score += config.jurisdiction_weight;
        }
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
                let fuzzy_score = self
                    .fuzzy_matcher
                    .fuzzy_match(&entry.statute.id, query)
                    .unwrap_or(0) as f64;
                let text_score =
                    self.calculate_relevance_score(entry, &query.to_lowercase(), &config);
                let normalized_fuzzy = (fuzzy_score / 100.0).min(1.0);
                let combined_score = (normalized_fuzzy * 0.4 + text_score * 0.6).min(1.0);
                if combined_score > 0.1 {
                    Some(SearchResult::new(entry, combined_score))
                } else {
                    None
                }
            })
            .collect();
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
        self.statutes.clear();
        self.versions.clear();
        self.tag_index.clear();
        self.jurisdiction_index.clear();
        self.cache.clear();
        self.event_store.clear();
        self.versions = backup.versions;
        for entry in backup.statutes {
            let statute_id = entry.statute.id.clone();
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
            self.statutes.insert(statute_id, entry);
        }
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
            if self.statutes.contains_key(&statute_id) {
                continue;
            }
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
            self.statutes.insert(statute_id.clone(), entry);
            merged_ids.push(statute_id);
        }
        for (statute_id, versions) in backup.versions {
            let entry = self.versions.entry(statute_id).or_default();
            for (version, version_entry) in versions {
                entry.insert(version, version_entry);
            }
        }
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
        incremental.delta_events = self
            .event_store
            .all_events()
            .iter()
            .filter(|e| self.event_timestamp(e) > base_time)
            .cloned()
            .cloned()
            .collect();
        incremental.changed_statutes = self
            .statutes
            .values()
            .filter(|entry| entry.modified_at > base_time)
            .cloned()
            .collect();
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
        for statute_id in &incremental.deleted_statute_ids {
            if self.statutes.contains_key(statute_id) {
                self.delete(statute_id)?;
            }
        }
        for entry in incremental.changed_statutes {
            let statute_id = entry.statute.id.clone();
            if self.statutes.contains_key(&statute_id) {
                self.update(&statute_id, entry.statute)?;
            } else {
                self.register(entry)?;
            }
        }
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
        if !entry.tags.contains(&tag) {
            entry.tags.push(tag.clone());
            entry.modified_at = Utc::now();
            entry.update_etag();
            self.tag_index
                .entry(tag.clone())
                .or_default()
                .insert(statute_id.to_string());
            self.cache.pop(statute_id);
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
        if let Some(pos) = entry.tags.iter().position(|t| t == tag) {
            entry.tags.remove(pos);
            entry.modified_at = Utc::now();
            entry.update_etag();
            if let Some(statute_ids) = self.tag_index.get_mut(tag) {
                statute_ids.remove(statute_id);
                if statute_ids.is_empty() {
                    self.tag_index.remove(tag);
                }
            }
            self.cache.pop(statute_id);
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
        self.cache.pop(statute_id);
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
            self.cache.pop(statute_id);
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
    /// Computes temporal analytics for the registry.
    ///
    /// Analyzes registration patterns, update frequency, and version velocity.
    /// Results are cached for performance.
    pub fn temporal_analytics(&mut self) -> TemporalAnalytics {
        if let Some(cached) = self.analytics_cache.get_temporal() {
            return cached.clone();
        }
        let analytics = self.compute_temporal_analytics();
        self.analytics_cache.set_temporal(analytics.clone());
        analytics
    }
    /// Computes temporal analytics without using cache.
    fn compute_temporal_analytics(&self) -> TemporalAnalytics {
        let mut registrations_per_day: HashMap<String, usize> = HashMap::new();
        let mut updates_per_day: HashMap<String, usize> = HashMap::new();
        let mut version_counts: HashMap<String, usize> = HashMap::new();
        for entry in self.statutes.values() {
            let date = entry.created_at.format("%Y-%m-%d").to_string();
            *registrations_per_day.entry(date).or_insert(0) += 1;
        }
        for entry in self.statutes.values() {
            if entry.modified_at != entry.created_at {
                let date = entry.modified_at.format("%Y-%m-%d").to_string();
                *updates_per_day.entry(date).or_insert(0) += 1;
            }
        }
        for (statute_id, versions) in &self.versions {
            version_counts.insert(statute_id.clone(), versions.len());
        }
        let avg_versions = if self.statutes.is_empty() {
            0.0
        } else {
            version_counts.values().sum::<usize>() as f64 / self.statutes.len() as f64
        };
        let mut most_versioned: Vec<(String, usize)> = version_counts.into_iter().collect();
        most_versioned.sort_by_key(|b| std::cmp::Reverse(b.1));
        most_versioned.truncate(10);
        let days_count = registrations_per_day.len().max(1);
        let growth_rate = self.statutes.len() as f64 / days_count as f64;
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
        if let Some(cached) = self.analytics_cache.get_relationship() {
            return cached.clone();
        }
        let analytics = self.compute_relationship_analytics();
        self.analytics_cache.set_relationship(analytics.clone());
        analytics
    }
    /// Computes relationship analytics without using cache.
    fn compute_relationship_analytics(&self) -> RelationshipAnalytics {
        let mut reference_counts: HashMap<String, usize> = HashMap::new();
        let mut dependency_counts: HashMap<String, usize> = HashMap::new();
        let mut supersession_chains: HashMap<String, Vec<String>> = HashMap::new();
        let mut has_relationships: HashSet<String> = HashSet::new();
        for entry in self.statutes.values() {
            for reference in &entry.references {
                *reference_counts.entry(reference.clone()).or_insert(0) += 1;
                has_relationships.insert(entry.statute.id.clone());
                has_relationships.insert(reference.clone());
            }
            dependency_counts.insert(entry.statute.id.clone(), entry.references.len());
        }
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
        let mut most_referenced: Vec<(String, usize)> = reference_counts.into_iter().collect();
        most_referenced.sort_by_key(|b| std::cmp::Reverse(b.1));
        most_referenced.truncate(10);
        let mut most_dependencies: Vec<(String, usize)> = dependency_counts.into_iter().collect();
        most_dependencies.sort_by_key(|b| std::cmp::Reverse(b.1));
        most_dependencies.truncate(10);
        let orphaned_statutes: Vec<String> = self
            .statutes
            .keys()
            .filter(|id| !has_relationships.contains(*id))
            .cloned()
            .collect();
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
        if let Some(cached) = self.analytics_cache.get_tag() {
            return cached.clone();
        }
        let analytics = self.compute_tag_analytics();
        self.analytics_cache.set_tag(analytics.clone());
        analytics
    }
    /// Computes tag analytics without using cache.
    fn compute_tag_analytics(&self) -> TagAnalytics {
        let mut tag_frequency: HashMap<String, usize> = HashMap::new();
        let mut tag_cooccurrence: HashMap<String, HashMap<String, usize>> = HashMap::new();
        for entry in self.statutes.values() {
            for tag in &entry.tags {
                *tag_frequency.entry(tag.clone()).or_insert(0) += 1;
            }
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
        let mut most_used_tags: Vec<(String, usize)> =
            tag_frequency.iter().map(|(t, &c)| (t.clone(), c)).collect();
        most_used_tags.sort_by_key(|b| std::cmp::Reverse(b.1));
        let top_most_used = most_used_tags.iter().take(10).cloned().collect();
        most_used_tags.sort_by_key(|a| a.1);
        let least_used_tags = most_used_tags.iter().take(10).cloned().collect();
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
        if let Some(cached) = self.analytics_cache.get_activity() {
            return cached.clone();
        }
        let analytics = self.compute_activity_analytics();
        self.analytics_cache.set_activity(analytics.clone());
        analytics
    }
    /// Computes activity analytics without using cache.
    fn compute_activity_analytics(&self) -> ActivityAnalytics {
        let mut modification_counts: HashMap<String, usize> = HashMap::new();
        let mut status_change_counts: HashMap<String, usize> = HashMap::new();
        for (statute_id, versions) in &self.versions {
            modification_counts.insert(statute_id.clone(), versions.len());
        }
        for event in self.event_store.all_events() {
            if let RegistryEvent::StatusChanged { statute_id, .. } = event {
                *status_change_counts.entry(statute_id.clone()).or_insert(0) += 1;
            }
        }
        let mut most_modified: Vec<(String, usize)> = modification_counts.into_iter().collect();
        most_modified.sort_by_key(|b| std::cmp::Reverse(b.1));
        most_modified.truncate(10);
        let mut recently_modified: Vec<(String, DateTime<Utc>)> = self
            .statutes
            .iter()
            .map(|(id, entry)| (id.clone(), entry.modified_at))
            .collect();
        recently_modified.sort_by_key(|b| std::cmp::Reverse(b.1));
        recently_modified.truncate(20);
        let mut least_modified: Vec<(String, DateTime<Utc>)> = self
            .statutes
            .iter()
            .map(|(id, entry)| (id.clone(), entry.modified_at))
            .collect();
        least_modified.sort_by_key(|a| a.1);
        least_modified.truncate(20);
        let mut frequent_status_changes: Vec<(String, usize)> =
            status_change_counts.into_iter().collect();
        frequent_status_changes.sort_by_key(|b| std::cmp::Reverse(b.1));
        frequent_status_changes.truncate(10);
        let total_modifications: usize = self.versions.values().map(|v| v.len()).sum();
        let avg_mod_frequency = if !recently_modified.is_empty() && total_modifications > 0 {
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
        wtr.write_record(["Key", "Count", "Percentage"])?;
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
