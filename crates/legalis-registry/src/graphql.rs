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
