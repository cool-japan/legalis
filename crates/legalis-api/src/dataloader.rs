//! DataLoader implementation for N+1 query optimization in GraphQL.
//!
//! This module provides DataLoader implementations that batch and cache
//! database/registry queries to prevent N+1 query problems in GraphQL resolvers.

use async_graphql::dataloader::Loader;
use legalis_core::Statute;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// DataLoader for batching statute lookups by ID.
///
/// This loader batches multiple statute ID lookups into a single operation,
/// reducing the number of reads from the statute registry and preventing
/// N+1 query problems in GraphQL resolvers.
///
/// # Example
///
/// ```rust,ignore
/// use legalis_api::dataloader::StatuteLoader;
/// use async_graphql::dataloader::DataLoader;
///
/// let loader = DataLoader::new(
///     StatuteLoader::new(statutes),
///     tokio::spawn
/// );
///
/// // Later in a GraphQL resolver:
/// let statute = loader.load_one(statute_id).await?;
/// ```
#[derive(Clone)]
pub struct StatuteLoader {
    statutes: Arc<RwLock<Vec<Statute>>>,
}

impl StatuteLoader {
    /// Create a new StatuteLoader with the given statute collection.
    pub fn new(statutes: Arc<RwLock<Vec<Statute>>>) -> Self {
        Self { statutes }
    }
}

#[async_trait::async_trait]
impl Loader<String> for StatuteLoader {
    type Value = Statute;
    type Error = Arc<String>;

    /// Load multiple statutes by their IDs in a single batch operation.
    ///
    /// This method is called by the DataLoader when it's time to execute
    /// a batch of loads. It receives all the keys (statute IDs) that have
    /// been requested since the last batch execution.
    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Statute>, Arc<String>> {
        let statutes = self.statutes.read().await;

        let mut result = HashMap::new();

        // Batch load all requested statutes in one pass
        for statute in statutes.iter() {
            if keys.contains(&statute.id) {
                result.insert(statute.id.clone(), statute.clone());
            }
        }

        Ok(result)
    }
}

/// DataLoader for batching jurisdiction-based statute lookups.
///
/// This loader batches multiple jurisdiction lookups into a single operation,
/// useful when resolving nested GraphQL queries that filter by jurisdiction.
#[derive(Clone)]
pub struct JurisdictionLoader {
    statutes: Arc<RwLock<Vec<Statute>>>,
}

impl JurisdictionLoader {
    /// Create a new JurisdictionLoader with the given statute collection.
    pub fn new(statutes: Arc<RwLock<Vec<Statute>>>) -> Self {
        Self { statutes }
    }
}

#[async_trait::async_trait]
impl Loader<String> for JurisdictionLoader {
    type Value = Vec<Statute>;
    type Error = Arc<String>;

    /// Load statutes for multiple jurisdictions in a single batch operation.
    ///
    /// Returns a map from jurisdiction ID to the list of statutes in that jurisdiction.
    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Vec<Statute>>, Arc<String>> {
        let statutes = self.statutes.read().await;

        let mut result: HashMap<String, Vec<Statute>> = HashMap::new();

        // Initialize empty vectors for all requested jurisdictions
        for key in keys {
            result.insert(key.clone(), Vec::new());
        }

        // Batch load all statutes and group by jurisdiction
        for statute in statutes.iter() {
            if let Some(jurisdiction) = &statute.jurisdiction {
                if keys.contains(jurisdiction) {
                    if let Some(statutes_vec) = result.get_mut(jurisdiction) {
                        statutes_vec.push(statute.clone());
                    }
                }
            }
        }

        Ok(result)
    }
}

/// DataLoader for batching version lookups (statutes by ID and version).
///
/// This loader is useful for queries that need to fetch specific versions
/// of statutes, such as historical comparisons or version diffs.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StatuteVersionKey {
    pub id: String,
    pub version: u32,
}

#[derive(Clone)]
pub struct VersionLoader {
    statutes: Arc<RwLock<Vec<Statute>>>,
}

impl VersionLoader {
    /// Create a new VersionLoader with the given statute collection.
    pub fn new(statutes: Arc<RwLock<Vec<Statute>>>) -> Self {
        Self { statutes }
    }
}

#[async_trait::async_trait]
impl Loader<StatuteVersionKey> for VersionLoader {
    type Value = Statute;
    type Error = Arc<String>;

    /// Load specific statute versions in a single batch operation.
    async fn load(&self, keys: &[StatuteVersionKey]) -> Result<HashMap<StatuteVersionKey, Statute>, Arc<String>> {
        let statutes = self.statutes.read().await;

        let mut result = HashMap::new();

        // Batch load all requested statute versions
        for statute in statutes.iter() {
            for key in keys {
                if statute.id == key.id && statute.version == key.version {
                    result.insert(key.clone(), statute.clone());
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn create_test_statute(id: &str, jurisdiction: Option<&str>, version: u32) -> Statute {
        use legalis_core::TemporalValidity;
        use std::collections::HashMap;

        Statute {
            id: id.to_string(),
            title: format!("Test Statute {}", id),
            version,
            jurisdiction: jurisdiction.map(|s| s.to_string()),
            effect: Effect {
                effect_type: EffectType::Obligation,
                description: "Test effect".to_string(),
                parameters: HashMap::new(),
            },
            preconditions: vec![],
            discretion_logic: None,
            exceptions: vec![],
            applies_to: vec![],
            derives_from: vec![],
            temporal_validity: TemporalValidity::default(),
        }
    }

    #[tokio::test]
    async fn test_statute_loader_batch() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
            create_test_statute("statute-2", Some("US"), 1),
            create_test_statute("statute-3", Some("EU"), 1),
        ]));

        let loader = StatuteLoader::new(statutes);

        // Request multiple statutes - they should be loaded in a single batch
        let keys = vec!["statute-1".to_string(), "statute-3".to_string()];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result.contains_key("statute-1"));
        assert!(result.contains_key("statute-3"));
        assert_eq!(result.get("statute-1").unwrap().id, "statute-1");
        assert_eq!(result.get("statute-3").unwrap().id, "statute-3");
    }

    #[tokio::test]
    async fn test_statute_loader_missing() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
        ]));

        let loader = StatuteLoader::new(statutes);

        // Request a statute that doesn't exist
        let keys = vec!["nonexistent".to_string()];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_jurisdiction_loader() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
            create_test_statute("statute-2", Some("US"), 1),
            create_test_statute("statute-3", Some("EU"), 1),
            create_test_statute("statute-4", Some("EU"), 1),
            create_test_statute("statute-5", Some("UK"), 1),
        ]));

        let loader = JurisdictionLoader::new(statutes);

        // Request statutes from multiple jurisdictions
        let keys = vec!["US".to_string(), "EU".to_string()];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result.get("US").unwrap().len(), 2);
        assert_eq!(result.get("EU").unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_jurisdiction_loader_empty() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
        ]));

        let loader = JurisdictionLoader::new(statutes);

        // Request a jurisdiction with no statutes
        let keys = vec!["NONEXISTENT".to_string()];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result.get("NONEXISTENT").unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_version_loader() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
            create_test_statute("statute-1", Some("US"), 2),
            create_test_statute("statute-1", Some("US"), 3),
            create_test_statute("statute-2", Some("US"), 1),
        ]));

        let loader = VersionLoader::new(statutes);

        // Request specific versions
        let keys = vec![
            StatuteVersionKey { id: "statute-1".to_string(), version: 1 },
            StatuteVersionKey { id: "statute-1".to_string(), version: 3 },
        ];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&keys[0]).unwrap().version, 1);
        assert_eq!(result.get(&keys[1]).unwrap().version, 3);
    }

    #[tokio::test]
    async fn test_version_loader_missing_version() {
        let statutes = Arc::new(RwLock::new(vec![
            create_test_statute("statute-1", Some("US"), 1),
        ]));

        let loader = VersionLoader::new(statutes);

        // Request a version that doesn't exist
        let keys = vec![
            StatuteVersionKey { id: "statute-1".to_string(), version: 99 },
        ];
        let result = loader.load(&keys).await.unwrap();

        assert_eq!(result.len(), 0);
    }
}
