//! SQLite-based storage backend for audit trails.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// SQLite-based audit storage.
pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
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
    /// let storage = SqliteStorage::new("audit.db").unwrap();
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> AuditResult<Self> {
        let conn = Connection::open(path)?;
        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Creates an in-memory SQLite database (useful for testing).
    pub fn in_memory() -> AuditResult<Self> {
        let conn = Connection::open_in_memory()?;
        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Initializes the database schema.
    fn initialize_schema(&self) -> AuditResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        conn.execute(
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
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_statute_id ON audit_records(statute_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_subject_id ON audit_records(subject_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON audit_records(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }
}

impl super::AuditStorage for SqliteStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let timestamp = record.timestamp.timestamp();
        let event_type = serde_json::to_string(&record.event_type)?;
        let actor = serde_json::to_string(&record.actor)?;
        let context = serde_json::to_string(&record.context)?;
        let result = serde_json::to_string(&record.result)?;

        conn.execute(
            "INSERT INTO audit_records (
                id, timestamp, event_type, actor, statute_id, subject_id,
                context, result, previous_hash, record_hash
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                record.id.to_string(),
                timestamp,
                event_type,
                actor,
                record.statute_id,
                record.subject_id.to_string(),
                context,
                result,
                record.previous_hash,
                record.record_hash,
            ],
        )?;

        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE id = ?1",
        )?;

        let record = stmt
            .query_row(params![id.to_string()], |row| {
                let timestamp: i64 = row.get(1)?;
                let event_type_str: String = row.get(2)?;
                let actor_str: String = row.get(3)?;
                let context_str: String = row.get(6)?;
                let result_str: String = row.get(7)?;

                Ok(AuditRecord {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap(),
                    event_type: serde_json::from_str(&event_type_str).unwrap(),
                    actor: serde_json::from_str(&actor_str).unwrap(),
                    statute_id: row.get(4)?,
                    subject_id: Uuid::parse_str(&row.get::<_, String>(5)?).unwrap(),
                    context: serde_json::from_str(&context_str).unwrap(),
                    result: serde_json::from_str(&result_str).unwrap(),
                    previous_hash: row.get(8)?,
                    record_hash: row.get(9)?,
                })
            })
            .optional()?;

        record.ok_or_else(|| AuditError::RecordNotFound(id))
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map([], |row| {
                let timestamp: i64 = row.get(1)?;
                let event_type_str: String = row.get(2)?;
                let actor_str: String = row.get(3)?;
                let context_str: String = row.get(6)?;
                let result_str: String = row.get(7)?;

                Ok(AuditRecord {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap(),
                    event_type: serde_json::from_str(&event_type_str).unwrap(),
                    actor: serde_json::from_str(&actor_str).unwrap(),
                    statute_id: row.get(4)?,
                    subject_id: Uuid::parse_str(&row.get::<_, String>(5)?).unwrap(),
                    context: serde_json::from_str(&context_str).unwrap(),
                    result: serde_json::from_str(&result_str).unwrap(),
                    previous_hash: row.get(8)?,
                    record_hash: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE statute_id = ?1
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map(params![statute_id], |row| {
                let timestamp: i64 = row.get(1)?;
                let event_type_str: String = row.get(2)?;
                let actor_str: String = row.get(3)?;
                let context_str: String = row.get(6)?;
                let result_str: String = row.get(7)?;

                Ok(AuditRecord {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap(),
                    event_type: serde_json::from_str(&event_type_str).unwrap(),
                    actor: serde_json::from_str(&actor_str).unwrap(),
                    statute_id: row.get(4)?,
                    subject_id: Uuid::parse_str(&row.get::<_, String>(5)?).unwrap(),
                    context: serde_json::from_str(&context_str).unwrap(),
                    result: serde_json::from_str(&result_str).unwrap(),
                    previous_hash: row.get(8)?,
                    record_hash: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE subject_id = ?1
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map(params![subject_id.to_string()], |row| {
                let timestamp: i64 = row.get(1)?;
                let event_type_str: String = row.get(2)?;
                let actor_str: String = row.get(3)?;
                let context_str: String = row.get(6)?;
                let result_str: String = row.get(7)?;

                Ok(AuditRecord {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap(),
                    event_type: serde_json::from_str(&event_type_str).unwrap(),
                    actor: serde_json::from_str(&actor_str).unwrap(),
                    statute_id: row.get(4)?,
                    subject_id: Uuid::parse_str(&row.get::<_, String>(5)?).unwrap(),
                    context: serde_json::from_str(&context_str).unwrap(),
                    result: serde_json::from_str(&result_str).unwrap(),
                    previous_hash: row.get(8)?,
                    record_hash: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let mut stmt = conn.prepare(
            "SELECT id, timestamp, event_type, actor, statute_id, subject_id,
                    context, result, previous_hash, record_hash
             FROM audit_records
             WHERE timestamp >= ?1 AND timestamp <= ?2
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map(params![start.timestamp(), end.timestamp()], |row| {
                let timestamp: i64 = row.get(1)?;
                let event_type_str: String = row.get(2)?;
                let actor_str: String = row.get(3)?;
                let context_str: String = row.get(6)?;
                let result_str: String = row.get(7)?;

                Ok(AuditRecord {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    timestamp: DateTime::from_timestamp(timestamp, 0).unwrap(),
                    event_type: serde_json::from_str(&event_type_str).unwrap(),
                    actor: serde_json::from_str(&actor_str).unwrap(),
                    statute_id: row.get(4)?,
                    subject_id: Uuid::parse_str(&row.get::<_, String>(5)?).unwrap(),
                    context: serde_json::from_str(&context_str).unwrap(),
                    result: serde_json::from_str(&result_str).unwrap(),
                    previous_hash: row.get(8)?,
                    record_hash: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }

    fn count(&self) -> AuditResult<usize> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM audit_records", [], |row| row.get(0))?;

        Ok(count as usize)
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        let hash = conn
            .query_row(
                "SELECT value FROM metadata WHERE key = 'last_hash'",
                [],
                |row| row.get(0),
            )
            .optional()?;

        Ok(hash)
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock connection: {}", e)))?;

        if let Some(h) = hash {
            conn.execute(
                "INSERT OR REPLACE INTO metadata (key, value) VALUES ('last_hash', ?1)",
                params![h],
            )?;
        } else {
            conn.execute("DELETE FROM metadata WHERE key = 'last_hash'", [])?;
        }

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
        let mut storage = SqliteStorage::in_memory().unwrap();

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
        storage.store(record).unwrap();

        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);
        assert_eq!(storage.count().unwrap(), 1);
    }

    #[test]
    fn test_sqlite_query_by_statute() {
        let mut storage = SqliteStorage::in_memory().unwrap();

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
            storage.store(record).unwrap();
        }

        let results = storage.get_by_statute("statute-1").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].statute_id, "statute-1");
    }

    #[test]
    fn test_sqlite_last_hash() {
        let mut storage = SqliteStorage::in_memory().unwrap();

        assert_eq!(storage.get_last_hash().unwrap(), None);

        storage
            .set_last_hash(Some("test-hash".to_string()))
            .unwrap();
        assert_eq!(
            storage.get_last_hash().unwrap(),
            Some("test-hash".to_string())
        );

        storage.set_last_hash(None).unwrap();
        assert_eq!(storage.get_last_hash().unwrap(), None);
    }
}
