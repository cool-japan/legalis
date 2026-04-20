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
    pub async fn set_status(&self, statute_id: &str, status: StatuteStatus) -> RegistryResult<()> {
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
    pub async fn batch_register(&self, entries: Vec<StatuteEntry>) -> Vec<RegistryResult<Uuid>> {
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
