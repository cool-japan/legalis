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
    async fn find_by_jurisdiction(&self, jurisdiction: &str) -> RegistryResult<Vec<StatuteEntry>>;

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

    async fn find_by_jurisdiction(&self, jurisdiction: &str) -> RegistryResult<Vec<StatuteEntry>> {
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

    async fn find_by_jurisdiction(&self, jurisdiction: &str) -> RegistryResult<Vec<StatuteEntry>> {
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
