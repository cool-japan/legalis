//! SQLite-based storage backend for audit trails.
//!
//! This backend is built on [`oxisql_sqlite_compat::SqliteConnection`], the
//! COOLJAPAN Pure-Rust SQLite-compatible engine (Limbo-backed via OxiSQL).  It
//! contains no C/C++ dependency — `libsqlite3` is never linked.
//!
//! # Sync ↔ async bridge
//!
//! The [`super::AuditStorage`] trait is synchronous, whereas OxiSQL is
//! asynchronous.  Each [`SqliteStorage`] therefore owns a dedicated
//! current-thread Tokio runtime and drives every database operation through
//! `runtime.block_on(...)`.  This keeps the backend self-contained: it works in
//! plain synchronous test functions and on worker threads alike, without
//! requiring an ambient runtime (unlike `tokio::runtime::Handle::current()`).
//!
//! # Parameter placeholders
//!
//! OxiSQL uses `$1`, `$2`, … positional placeholders (the `oxisql-sqlite-compat`
//! layer rewrites them to `?` for Limbo internally).  All SQL in this module
//! uses the `$N` form.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use oxisql_core::{Connection, ToSqlValue, Value};
use oxisql_sqlite_compat::SqliteConnection;
use std::path::Path;
use tokio::runtime::Runtime;
use uuid::Uuid;

/// SQLite-based audit storage backed by OxiSQL (Pure Rust).
pub struct SqliteStorage {
    conn: SqliteConnection,
    runtime: Runtime,
}

/// Builds a fresh current-thread Tokio runtime used to drive the async OxiSQL
/// connection from the synchronous [`super::AuditStorage`] trait.
fn build_runtime() -> AuditResult<Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| AuditError::StorageError(format!("Failed to build Tokio runtime: {e}")))
}

/// Extracts a column as a [`String`], mapping a missing/wrong-typed column to a
/// typed [`AuditError`].
fn text_col(row: &oxisql_core::Row, idx: usize, name: &str) -> AuditResult<String> {
    match row.get_by_index(idx) {
        Some(Value::Text(s)) => Ok(s.clone()),
        Some(other) => Err(AuditError::StorageError(format!(
            "column {name} (index {idx}): expected text, got {}",
            other.type_name()
        ))),
        None => Err(AuditError::StorageError(format!(
            "column {name} (index {idx}) missing from result row"
        ))),
    }
}

/// Extracts a column as an optional [`String`] (NULL → `None`).
fn opt_text_col(row: &oxisql_core::Row, idx: usize, name: &str) -> AuditResult<Option<String>> {
    match row.get_by_index(idx) {
        Some(Value::Null) | None => Ok(None),
        Some(Value::Text(s)) => Ok(Some(s.clone())),
        Some(other) => Err(AuditError::StorageError(format!(
            "column {name} (index {idx}): expected text or null, got {}",
            other.type_name()
        ))),
    }
}

/// Extracts a column as an `i64`.
fn int_col(row: &oxisql_core::Row, idx: usize, name: &str) -> AuditResult<i64> {
    match row.get_by_index(idx) {
        Some(Value::I64(n)) => Ok(*n),
        Some(other) => Err(AuditError::StorageError(format!(
            "column {name} (index {idx}): expected integer, got {}",
            other.type_name()
        ))),
        None => Err(AuditError::StorageError(format!(
            "column {name} (index {idx}) missing from result row"
        ))),
    }
}

/// Parses a [`Uuid`] from a string column, surfacing parse failures as typed errors.
fn parse_uuid(s: &str, name: &str) -> AuditResult<Uuid> {
    Uuid::parse_str(s)
        .map_err(|e| AuditError::StorageError(format!("invalid UUID in column {name}: {e}")))
}

/// Reconstructs an [`AuditdRecord`](AuditRecord) from a result [`Row`](oxisql_core::Row).
///
/// Column order matches the `SELECT` projection used throughout this module:
/// `id, timestamp, event_type, actor, statute_id, subject_id, context, result,
/// previous_hash, record_hash`.
fn row_to_record(row: &oxisql_core::Row) -> AuditResult<AuditRecord> {
    let id_str = text_col(row, 0, "id")?;
    let timestamp = int_col(row, 1, "timestamp")?;
    let event_type_str = text_col(row, 2, "event_type")?;
    let actor_str = text_col(row, 3, "actor")?;
    let statute_id = text_col(row, 4, "statute_id")?;
    let subject_id_str = text_col(row, 5, "subject_id")?;
    let context_str = text_col(row, 6, "context")?;
    let result_str = text_col(row, 7, "result")?;
    let previous_hash = opt_text_col(row, 8, "previous_hash")?;
    let record_hash = text_col(row, 9, "record_hash")?;

    Ok(AuditRecord {
        id: parse_uuid(&id_str, "id")?,
        timestamp: DateTime::from_timestamp(timestamp, 0).ok_or_else(|| {
            AuditError::StorageError(format!("timestamp out of range: {timestamp}"))
        })?,
        event_type: serde_json::from_str(&event_type_str)?,
        actor: serde_json::from_str(&actor_str)?,
        statute_id,
        subject_id: parse_uuid(&subject_id_str, "subject_id")?,
        context: serde_json::from_str(&context_str)?,
        result: serde_json::from_str(&result_str)?,
        previous_hash,
        record_hash,
    })
}

impl SqliteStorage {
    /// Creates a new SQLite storage backend.
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file
    ///
    /// # Example
    /// ```no_run
    /// use legalis_audit::storage::sqlite::SqliteStorage;
    ///
    /// let storage = SqliteStorage::new("audit.db").expect("open audit db");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> AuditResult<Self> {
        let runtime = build_runtime()?;
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let conn = runtime.block_on(SqliteConnection::open(&path_str))?;
        let storage = Self { conn, runtime };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Creates an in-memory SQLite database (useful for testing).
    pub fn in_memory() -> AuditResult<Self> {
        let runtime = build_runtime()?;
        let conn = runtime.block_on(SqliteConnection::open_memory())?;
        let storage = Self { conn, runtime };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Initializes the database schema.
    fn initialize_schema(&self) -> AuditResult<()> {
        self.runtime.block_on(async {
            self.conn
                .execute(
                    "CREATE TABLE IF NOT EXISTS audit_records (
                        id TEXT PRIMARY KEY,
                        timestamp INTEGER NOT NULL,
                        event_type TEXT NOT NULL,
                        actor TEXT NOT NULL,
                        statute_id TEXT NOT NULL,
                        subject_id TEXT NOT NULL,
                        context TEXT NOT NULL,
                        result TEXT NOT NULL,
                        previous_hash TEXT,
                        record_hash TEXT NOT NULL
                    )",
                    &[],
                )
                .await?;

            self.conn
                .execute(
                    "CREATE INDEX IF NOT EXISTS idx_statute_id ON audit_records(statute_id)",
                    &[],
                )
                .await?;

            self.conn
                .execute(
                    "CREATE INDEX IF NOT EXISTS idx_subject_id ON audit_records(subject_id)",
                    &[],
                )
                .await?;

            self.conn
                .execute(
                    "CREATE INDEX IF NOT EXISTS idx_timestamp ON audit_records(timestamp)",
                    &[],
                )
                .await?;

            self.conn
                .execute(
                    "CREATE TABLE IF NOT EXISTS metadata (
                        key TEXT PRIMARY KEY,
                        value TEXT NOT NULL
                    )",
                    &[],
                )
                .await?;

            Ok::<(), AuditError>(())
        })
    }

    /// Runs the shared `SELECT` projection with the supplied positional params
    /// and decodes every row into an [`AuditRecord`].
    fn query_records(
        &self,
        sql: &str,
        params: &[&dyn ToSqlValue],
    ) -> AuditResult<Vec<AuditRecord>> {
        let rows = self.runtime.block_on(self.conn.query(sql, params))?;
        rows.iter().map(row_to_record).collect()
    }
}

impl super::AuditStorage for SqliteStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let timestamp = record.timestamp.timestamp();
        let event_type = serde_json::to_string(&record.event_type)?;
        let actor = serde_json::to_string(&record.actor)?;
        let context = serde_json::to_string(&record.context)?;
        let result = serde_json::to_string(&record.result)?;
        let id = record.id.to_string();
        let subject_id = record.subject_id.to_string();

        self.runtime.block_on(async {
            self.conn
                .execute(
                    "INSERT INTO audit_records (
                        id, timestamp, event_type, actor, statute_id, subject_id,
                        context, result, previous_hash, record_hash
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                    &[
                        &id,
                        &timestamp,
                        &event_type,
                        &actor,
                        &record.statute_id,
                        &subject_id,
                        &context,
                        &result,
                        &record.previous_hash,
                        &record.record_hash,
                    ],
                )
                .await
        })?;

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let id_str = id.to_string();
        let rows = self.runtime.block_on(self.conn.query(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE id = $1",
            &[&id_str],
        ))?;

        match rows.first() {
            Some(row) => row_to_record(row),
            None => Err(AuditError::RecordNotFound(id)),
        }
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        self.query_records(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             ORDER BY timestamp ASC",
            &[],
        )
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        self.query_records(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE statute_id = $1
             ORDER BY timestamp ASC",
            &[&statute_id],
        )
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let subject_id_str = subject_id.to_string();
        self.query_records(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE subject_id = $1
             ORDER BY timestamp ASC",
            &[&subject_id_str],
        )
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let start_ts = start.timestamp();
        let end_ts = end.timestamp();
        self.query_records(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE timestamp >= $1 AND timestamp <= $2
             ORDER BY timestamp ASC",
            &[&start_ts, &end_ts],
        )
    }

    fn count(&self) -> AuditResult<usize> {
        let rows = self
            .runtime
            .block_on(self.conn.query("SELECT COUNT(*) FROM audit_records", &[]))?;
        let count = match rows.first() {
            Some(row) => int_col(row, 0, "count")?,
            None => 0,
        };
        Ok(count.max(0) as usize)
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        let rows = self.runtime.block_on(
            self.conn
                .query("SELECT value FROM metadata WHERE key = 'last_hash'", &[]),
        )?;
        match rows.first() {
            Some(row) => Ok(Some(text_col(row, 0, "value")?)),
            None => Ok(None),
        }
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        // NOTE: Limbo 0.0.22 (the engine behind oxisql-sqlite-compat) does not
        // support `INSERT OR REPLACE` / `ON CONFLICT`.  Because `metadata` holds
        // a single `last_hash` row, an explicit DELETE-then-INSERT is exactly
        // equivalent to an upsert and stays within the supported SQL surface.
        self.runtime.block_on(async {
            self.conn
                .execute("DELETE FROM metadata WHERE key = 'last_hash'", &[])
                .await?;

            if let Some(h) = hash {
                self.conn
                    .execute(
                        "INSERT INTO metadata (key, value) VALUES ('last_hash', $1)",
                        &[&h],
                    )
                    .await?;
            }

            Ok::<(), AuditError>(())
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::AuditStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    #[test]
    fn test_sqlite_storage() {
        let mut storage = SqliteStorage::in_memory().expect("create in-memory storage");

        let record = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        );

        let id = record.id;
        storage.store(record).expect("store record");

        let retrieved = storage.get(id).expect("get record");
        assert_eq!(retrieved.id, id);
        assert_eq!(storage.count().expect("count"), 1);
    }

    #[test]
    fn test_sqlite_query_by_statute() {
        let mut storage = SqliteStorage::in_memory().expect("create in-memory storage");

        for i in 0..3 {
            let record = AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "test".to_string(),
                },
                format!("statute-{}", i),
                Uuid::new_v4(),
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "test".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            );
            storage.store(record).expect("store record");
        }

        let results = storage
            .get_by_statute("statute-1")
            .expect("query by statute");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].statute_id, "statute-1");
    }

    #[test]
    fn test_sqlite_last_hash() {
        let mut storage = SqliteStorage::in_memory().expect("create in-memory storage");

        assert_eq!(storage.get_last_hash().expect("get last hash"), None);

        storage
            .set_last_hash(Some("test-hash".to_string()))
            .expect("set last hash");
        assert_eq!(
            storage.get_last_hash().expect("get last hash"),
            Some("test-hash".to_string())
        );

        storage.set_last_hash(None).expect("clear last hash");
        assert_eq!(storage.get_last_hash().expect("get last hash"), None);
    }
}
