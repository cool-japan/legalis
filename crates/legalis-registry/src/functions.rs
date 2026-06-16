//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#![allow(dead_code)]

use legalis_core::Statute;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use super::types::RegistryError;
use super::types_3::RegistryEvent;
use super::types_4::StatuteRegistry;
use super::types_6::{StatuteEntry, StatuteStatus};

/// Result type for registry operations.
pub type RegistryResult<T> = Result<T, RegistryError>;
/// Webhook callback function type.
pub type WebhookCallback = Arc<dyn Fn(&RegistryEvent) + Send + Sync>;
#[cfg(feature = "async")]
pub mod async_api {
    //! Async variants of registry operations.
    //!
    //! This module provides async versions of the main registry methods,
    //! allowing integration with async runtimes like tokio.
    use super::*;
    use crate::types::ActivityAnalytics;
    use crate::types_3::TagAnalytics;
    use crate::types_5::{Pagination, RegistryBackup, WebhookEventFilter};
    use crate::types_6::{AggregationResult, RelationshipAnalytics};
    use crate::types_7::{SearchQuery, TemporalAnalytics};
    use crate::types_8::PagedResult;
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
#[cfg(all(feature = "async", feature = "async-stream"))]
pub mod streaming {
    //! Streaming support for large result sets.
    //!
    //! This module provides Stream implementations for efficiently
    //! iterating over large collections of statutes.
    use super::*;
    use crate::types_7::SearchQuery;
    use crate::types_8::StatuteSummary;
    use async_stream::stream;
    use futures::Stream;
    /// Creates a stream of all statutes.
    pub fn stream_all(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteEntry>> {
        stream! {
            let registry = registry.read(). await; let statutes : Vec < StatuteEntry > =
            registry.list().into_iter().cloned().collect(); drop(registry); for chunk in
            statutes.chunks(chunk_size) { yield chunk.to_vec(); }
        }
    }
    /// Creates a stream of statutes matching a query.
    pub fn stream_search(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        query: SearchQuery,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteEntry>> {
        stream! {
            let registry = registry.read(). await; let results : Vec < StatuteEntry > =
            registry.search(& query).iter().map(|& e | e.clone()).collect();
            drop(registry); for chunk in results.chunks(chunk_size) { yield chunk
            .to_vec(); }
        }
    }
    /// Creates a stream of statute summaries.
    pub fn stream_summaries(
        registry: std::sync::Arc<tokio::sync::RwLock<StatuteRegistry>>,
        chunk_size: usize,
    ) -> impl Stream<Item = Vec<StatuteSummary>> {
        stream! {
            let registry = registry.read(). await; let summaries : Vec < StatuteSummary >
            = registry.list_summaries().into_iter().collect(); drop(registry); for chunk
            in summaries.chunks(chunk_size) { yield chunk.to_vec(); }
        }
    }
}
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
#[cfg(feature = "akoma-ntoso")]
pub mod akoma_ntoso {
    //! Import/export support for Akoma Ntoso format.
    //!
    //! Akoma Ntoso is an XML standard for parliamentary,
    //! legislative and judiciary documents.
    use super::*;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;
    use serde::{Deserialize, Serialize};
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
        let effect = legalis_core::Effect::new(
            legalis_core::EffectType::Custom,
            "Imported from Akoma Ntoso XML",
        );
        let statute = Statute::new(&statute_id, &title, effect);
        StatuteEntry::new(statute, jurisdiction)
    }
}
