//! PostgreSQL-based storage backend for audit trails.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

/// PostgreSQL-based audit storage.
pub struct PostgresStorage {
    client: Arc<Mutex<Client>>,
}

impl PostgresStorage {
    /// Creates a new PostgreSQL storage backend.
    ///
    /// # Arguments
    /// * `connection_string` - PostgreSQL connection string (e.g., "host=localhost user=postgres")
    ///
    /// # Example
    /// ```no_run
    /// use legalis_audit::storage::postgres::PostgresStorage;
    ///
    /// # async fn example() {
    /// let storage = PostgresStorage::new("host=localhost user=postgres dbname=audit").await.unwrap();
    /// # }
    /// ```
    pub async fn new(connection_string: &str) -> AuditResult<Self> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls)
            .await
            .map_err(|e| {
                AuditError::StorageError(format!("Failed to connect to PostgreSQL: {}", e))
            })?;

        // Spawn connection handler
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        let storage = Self {
            client: Arc::new(Mutex::new(client)),
        };

        storage.initialize_schema().await?;
        Ok(storage)
    }

    /// Initializes the database schema.
    async fn initialize_schema(&self) -> AuditResult<()> {
        let client = self.client.lock().await;

        client
            .execute(
                "CREATE TABLE IF NOT EXISTS audit_records (
                    id UUID PRIMARY KEY,
                    timestamp TIMESTAMPTZ NOT NULL,
                    event_type JSONB NOT NULL,
                    actor JSONB NOT NULL,
                    statute_id TEXT NOT NULL,
                    subject_id UUID NOT NULL,
                    context JSONB NOT NULL,
                    result JSONB NOT NULL,
                    previous_hash TEXT,
                    record_hash TEXT NOT NULL
                )",
                &[],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to create table: {}", e)))?;

        client
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_statute_id ON audit_records(statute_id)",
                &[],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to create index: {}", e)))?;

        client
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_subject_id ON audit_records(subject_id)",
                &[],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to create index: {}", e)))?;

        client
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_timestamp ON audit_records(timestamp)",
                &[],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to create index: {}", e)))?;

        client
            .execute(
                "CREATE TABLE IF NOT EXISTS metadata (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                )",
                &[],
            )
            .await
            .map_err(|e| {
                AuditError::StorageError(format!("Failed to create metadata table: {}", e))
            })?;

        Ok(())
    }

    /// Stores a record (async version).
    pub async fn store_async(&self, record: AuditRecord) -> AuditResult<()> {
        let client = self.client.lock().await;

        let event_type = serde_json::to_value(&record.event_type)?;
        let actor = serde_json::to_value(&record.actor)?;
        let context = serde_json::to_value(&record.context)?;
        let result_value = serde_json::to_value(&record.result)?;

        client
            .execute(
                "INSERT INTO audit_records (
                    id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[
                    &record.id,
                    &record.timestamp,
                    &event_type,
                    &actor,
                    &record.statute_id,
                    &record.subject_id,
                    &context,
                    &result_value,
                    &record.previous_hash,
                    &record.record_hash,
                ],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to insert record: {}", e)))?;

        Ok(())
    }

    /// Retrieves a record by ID (async version).
    pub async fn get_async(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let client = self.client.lock().await;

        let row = client
            .query_one(
                "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                 FROM audit_records
                 WHERE id = $1",
                &[&id],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Record not found: {}", e)))?;

        let event_type: serde_json::Value = row.get(2);
        let actor: serde_json::Value = row.get(3);
        let context: serde_json::Value = row.get(6);
        let result_value: serde_json::Value = row.get(7);

        Ok(AuditRecord {
            id: row.get(0),
            timestamp: row.get(1),
            event_type: serde_json::from_value(event_type)?,
            actor: serde_json::from_value(actor)?,
            statute_id: row.get(4),
            subject_id: row.get(5),
            context: serde_json::from_value(context)?,
            result: serde_json::from_value(result_value)?,
            previous_hash: row.get(8),
            record_hash: row.get(9),
        })
    }

    /// Retrieves all records (async version).
    pub async fn get_all_async(&self) -> AuditResult<Vec<AuditRecord>> {
        let client = self.client.lock().await;

        let rows = client
            .query(
                "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                 FROM audit_records
                 ORDER BY timestamp ASC",
                &[],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to query records: {}", e)))?;

        let mut records = Vec::new();
        for row in rows {
            let event_type: serde_json::Value = row.get(2);
            let actor: serde_json::Value = row.get(3);
            let context: serde_json::Value = row.get(6);
            let result_value: serde_json::Value = row.get(7);

            records.push(AuditRecord {
                id: row.get(0),
                timestamp: row.get(1),
                event_type: serde_json::from_value(event_type)?,
                actor: serde_json::from_value(actor)?,
                statute_id: row.get(4),
                subject_id: row.get(5),
                context: serde_json::from_value(context)?,
                result: serde_json::from_value(result_value)?,
                previous_hash: row.get(8),
                record_hash: row.get(9),
            });
        }

        Ok(records)
    }

    /// Retrieves records by statute ID (async version).
    pub async fn get_by_statute_async(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let client = self.client.lock().await;

        let rows = client
            .query(
                "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                 FROM audit_records
                 WHERE statute_id = $1
                 ORDER BY timestamp ASC",
                &[&statute_id],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to query records: {}", e)))?;

        let mut records = Vec::new();
        for row in rows {
            let event_type: serde_json::Value = row.get(2);
            let actor: serde_json::Value = row.get(3);
            let context: serde_json::Value = row.get(6);
            let result_value: serde_json::Value = row.get(7);

            records.push(AuditRecord {
                id: row.get(0),
                timestamp: row.get(1),
                event_type: serde_json::from_value(event_type)?,
                actor: serde_json::from_value(actor)?,
                statute_id: row.get(4),
                subject_id: row.get(5),
                context: serde_json::from_value(context)?,
                result: serde_json::from_value(result_value)?,
                previous_hash: row.get(8),
                record_hash: row.get(9),
            });
        }

        Ok(records)
    }

    /// Retrieves records by subject ID (async version).
    pub async fn get_by_subject_async(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let client = self.client.lock().await;

        let rows = client
            .query(
                "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                 FROM audit_records
                 WHERE subject_id = $1
                 ORDER BY timestamp ASC",
                &[&subject_id],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to query records: {}", e)))?;

        let mut records = Vec::new();
        for row in rows {
            let event_type: serde_json::Value = row.get(2);
            let actor: serde_json::Value = row.get(3);
            let context: serde_json::Value = row.get(6);
            let result_value: serde_json::Value = row.get(7);

            records.push(AuditRecord {
                id: row.get(0),
                timestamp: row.get(1),
                event_type: serde_json::from_value(event_type)?,
                actor: serde_json::from_value(actor)?,
                statute_id: row.get(4),
                subject_id: row.get(5),
                context: serde_json::from_value(context)?,
                result: serde_json::from_value(result_value)?,
                previous_hash: row.get(8),
                record_hash: row.get(9),
            });
        }

        Ok(records)
    }

    /// Retrieves records within a time range (async version).
    pub async fn get_by_time_range_async(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let client = self.client.lock().await;

        let rows = client
            .query(
                "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                 FROM audit_records
                 WHERE timestamp >= $1 AND timestamp <= $2
                 ORDER BY timestamp ASC",
                &[&start, &end],
            )
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to query records: {}", e)))?;

        let mut records = Vec::new();
        for row in rows {
            let event_type: serde_json::Value = row.get(2);
            let actor: serde_json::Value = row.get(3);
            let context: serde_json::Value = row.get(6);
            let result_value: serde_json::Value = row.get(7);

            records.push(AuditRecord {
                id: row.get(0),
                timestamp: row.get(1),
                event_type: serde_json::from_value(event_type)?,
                actor: serde_json::from_value(actor)?,
                statute_id: row.get(4),
                subject_id: row.get(5),
                context: serde_json::from_value(context)?,
                result: serde_json::from_value(result_value)?,
                previous_hash: row.get(8),
                record_hash: row.get(9),
            });
        }

        Ok(records)
    }

    /// Returns the total number of records (async version).
    pub async fn count_async(&self) -> AuditResult<usize> {
        let client = self.client.lock().await;

        let row = client
            .query_one("SELECT COUNT(*) FROM audit_records", &[])
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to count records: {}", e)))?;

        let count: i64 = row.get(0);
        Ok(count as usize)
    }

    /// Gets the hash of the last record in the chain (async version).
    pub async fn get_last_hash_async(&self) -> AuditResult<Option<String>> {
        let client = self.client.lock().await;

        let row = client
            .query_opt("SELECT value FROM metadata WHERE key = 'last_hash'", &[])
            .await
            .map_err(|e| AuditError::StorageError(format!("Failed to get last hash: {}", e)))?;

        Ok(row.map(|r| r.get(0)))
    }

    /// Updates the last hash (async version).
    pub async fn set_last_hash_async(&self, hash: Option<String>) -> AuditResult<()> {
        let client = self.client.lock().await;

        if let Some(h) = hash {
            client
                .execute(
                    "INSERT INTO metadata (key, value) VALUES ('last_hash', $1)
                     ON CONFLICT (key) DO UPDATE SET value = $1",
                    &[&h],
                )
                .await
                .map_err(|e| AuditError::StorageError(format!("Failed to set last hash: {}", e)))?;
        } else {
            client
                .execute("DELETE FROM metadata WHERE key = 'last_hash'", &[])
                .await
                .map_err(|e| {
                    AuditError::StorageError(format!("Failed to delete last hash: {}", e))
                })?;
        }

        Ok(())
    }
}

// Implement the synchronous AuditStorage trait by blocking on async operations
impl super::AuditStorage for PostgresStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.store_async(record))
        })
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_async(id))
        })
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_all_async())
        })
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_by_statute_async(statute_id))
        })
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_by_subject_async(subject_id))
        })
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_by_time_range_async(start, end))
        })
    }

    fn count(&self) -> AuditResult<usize> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.count_async())
        })
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.get_last_hash_async())
        })
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.set_last_hash_async(hash))
        })
    }
}
