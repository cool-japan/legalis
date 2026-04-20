//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::functions::RegistryResult;
use super::types::{RegistryError, Validator};
use super::types_4::StatuteRegistry;
use super::types_6::{StatuteEntry, StatuteStatus};
use super::types_8::ValidationError;

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
/// Result type for validation operations.
pub type ValidationResult<T> = Result<T, ValidationError>;
/// A validation rule for statute entries.
pub trait ValidationRule: Send + Sync {
    /// Validates a statute entry.
    fn validate(&self, entry: &StatuteEntry) -> ValidationResult<()>;
    /// Returns a description of this validation rule.
    fn description(&self) -> String;
}
fn default_true() -> bool {
    true
}
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
            if self.validate {
                let validator = Validator::with_defaults();
                if let Err(errors) = validator.validate(&entry) {
                    return Err(RegistryError::InvalidOperation(format!(
                        "Validation failed: {:?}",
                        errors
                    )));
                }
            }
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
