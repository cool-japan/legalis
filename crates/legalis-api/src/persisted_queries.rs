//! Persisted queries for GraphQL.
//!
//! Persisted queries allow clients to send a hash of a GraphQL query instead of
//! the full query text, reducing bandwidth and improving security by limiting
//! which queries can be executed.

use async_graphql::{
    ServerError, Variables,
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::ExecutableDocument,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A persisted query entry containing the query text and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedQuery {
    /// The full GraphQL query text
    pub query: String,
    /// Optional name for the query
    pub name: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// When the query was registered
    pub registered_at: chrono::DateTime<chrono::Utc>,
}

/// Storage for persisted queries.
#[derive(Clone)]
pub struct PersistedQueryStore {
    queries: Arc<RwLock<HashMap<String, PersistedQuery>>>,
}

impl PersistedQueryStore {
    /// Create a new empty persisted query store.
    pub fn new() -> Self {
        Self {
            queries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new persisted query with the given hash.
    pub async fn register(&self, hash: String, query: String, name: Option<String>) {
        let mut queries = self.queries.write().await;
        queries.insert(
            hash,
            PersistedQuery {
                query,
                name,
                description: None,
                registered_at: chrono::Utc::now(),
            },
        );
    }

    /// Get a persisted query by its hash.
    pub async fn get(&self, hash: &str) -> Option<PersistedQuery> {
        let queries = self.queries.read().await;
        queries.get(hash).cloned()
    }

    /// Check if a query hash exists in the store.
    pub async fn contains(&self, hash: &str) -> bool {
        let queries = self.queries.read().await;
        queries.contains_key(hash)
    }

    /// Remove a persisted query by its hash.
    pub async fn remove(&self, hash: &str) -> bool {
        let mut queries = self.queries.write().await;
        queries.remove(hash).is_some()
    }

    /// Get all registered query hashes.
    pub async fn list_hashes(&self) -> Vec<String> {
        let queries = self.queries.read().await;
        queries.keys().cloned().collect()
    }

    /// Get the total number of registered queries.
    pub async fn count(&self) -> usize {
        let queries = self.queries.read().await;
        queries.len()
    }

    /// Clear all persisted queries.
    pub async fn clear(&self) {
        let mut queries = self.queries.write().await;
        queries.clear();
    }
}

impl Default for PersistedQueryStore {
    fn default() -> Self {
        Self::new()
    }
}

/// GraphQL extension for persisted queries support.
///
/// This extension intercepts query execution and:
/// 1. If a query hash is provided, looks up the query from the store
/// 2. If the query is not found, returns an error
/// 3. Allows automatic registration of new queries (if enabled)
pub struct PersistedQueryExtension {
    store: PersistedQueryStore,
    allow_auto_persist: bool,
}

impl PersistedQueryExtension {
    /// Create a new persisted query extension.
    pub fn new(store: PersistedQueryStore, allow_auto_persist: bool) -> Self {
        Self {
            store,
            allow_auto_persist,
        }
    }
}

impl ExtensionFactory for PersistedQueryExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(PersistedQueryExtensionImpl {
            store: self.store.clone(),
            allow_auto_persist: self.allow_auto_persist,
        })
    }
}

struct PersistedQueryExtensionImpl {
    store: PersistedQueryStore,
    allow_auto_persist: bool,
}

#[async_trait::async_trait]
impl Extension for PersistedQueryExtensionImpl {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> async_graphql::ServerResult<ExecutableDocument> {
        // Check if this is a persisted query hash (starts with "sha256:")
        if let Some(hash) = query.strip_prefix("sha256:") {
            // Look up the query from the store
            if let Some(persisted) = self.store.get(hash).await {
                // Execute the persisted query
                return next.run(ctx, &persisted.query, variables).await;
            } else {
                // Query not found in store
                return Err(ServerError::new(
                    format!("Persisted query not found: {}", hash),
                    None,
                ));
            }
        }

        // For automatic persisting, hash the query and store it
        if self.allow_auto_persist && !query.is_empty() {
            let hash = sha256_hash(query);
            if !self.store.contains(&hash).await {
                self.store
                    .register(hash.clone(), query.to_string(), None)
                    .await;
            }
        }

        // Normal query execution
        next.run(ctx, query, variables).await
    }
}

/// Compute SHA-256 hash of a string.
fn sha256_hash(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Automatic persisted query (APQ) protocol support.
///
/// This implements the Apollo APQ protocol where clients can:
/// 1. Send query hash only
/// 2. If not found, server returns PersistedQueryNotFound error
/// 3. Client re-sends with both hash and query
/// 4. Server stores the query for future use
#[derive(Clone)]
pub struct ApqExtension {
    store: PersistedQueryStore,
}

impl ApqExtension {
    pub fn new(store: PersistedQueryStore) -> Self {
        Self { store }
    }
}

impl ExtensionFactory for ApqExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(ApqExtensionImpl {
            store: self.store.clone(),
        })
    }
}

struct ApqExtensionImpl {
    store: PersistedQueryStore,
}

#[async_trait::async_trait]
impl Extension for ApqExtensionImpl {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> async_graphql::ServerResult<ExecutableDocument> {
        // APQ protocol: check for hash in extensions
        if let Ok(extensions) = ctx.data::<serde_json::Value>()
            && let Some(apq) = extensions.get("persistedQuery")
            && let Some(hash) = apq.get("sha256Hash").and_then(|h| h.as_str())
        {
            // If query is empty, this is a persisted query request
            if query.is_empty() {
                if let Some(persisted) = self.store.get(hash).await {
                    return next.run(ctx, &persisted.query, variables).await;
                } else {
                    return Err(ServerError::new("PersistedQueryNotFound", None));
                }
            } else {
                // Client provided both hash and query - store it
                self.store
                    .register(hash.to_string(), query.to_string(), None)
                    .await;
            }
        }

        next.run(ctx, query, variables).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_persisted_query_store_registration() {
        let store = PersistedQueryStore::new();

        assert_eq!(store.count().await, 0);

        store
            .register(
                "hash1".to_string(),
                "query { users }".to_string(),
                Some("GetUsers".to_string()),
            )
            .await;

        assert_eq!(store.count().await, 1);
        assert!(store.contains("hash1").await);
    }

    #[tokio::test]
    async fn test_persisted_query_store_retrieval() {
        let store = PersistedQueryStore::new();

        store
            .register(
                "hash1".to_string(),
                "query { users }".to_string(),
                Some("GetUsers".to_string()),
            )
            .await;

        let query = store.get("hash1").await;
        assert!(query.is_some());
        assert_eq!(query.unwrap().query, "query { users }");
    }

    #[tokio::test]
    async fn test_persisted_query_store_removal() {
        let store = PersistedQueryStore::new();

        store
            .register("hash1".to_string(), "query { users }".to_string(), None)
            .await;

        assert!(store.remove("hash1").await);
        assert!(!store.contains("hash1").await);
        assert_eq!(store.count().await, 0);
    }

    #[tokio::test]
    async fn test_persisted_query_store_list() {
        let store = PersistedQueryStore::new();

        store
            .register("hash1".to_string(), "query1".to_string(), None)
            .await;
        store
            .register("hash2".to_string(), "query2".to_string(), None)
            .await;

        let hashes = store.list_hashes().await;
        assert_eq!(hashes.len(), 2);
        assert!(hashes.contains(&"hash1".to_string()));
        assert!(hashes.contains(&"hash2".to_string()));
    }

    #[tokio::test]
    async fn test_persisted_query_store_clear() {
        let store = PersistedQueryStore::new();

        store
            .register("hash1".to_string(), "query1".to_string(), None)
            .await;
        store
            .register("hash2".to_string(), "query2".to_string(), None)
            .await;

        assert_eq!(store.count().await, 2);

        store.clear().await;
        assert_eq!(store.count().await, 0);
    }

    #[test]
    fn test_sha256_hash() {
        let hash1 = sha256_hash("test query");
        let hash2 = sha256_hash("test query");
        let hash3 = sha256_hash("different query");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64 hex characters
    }
}
