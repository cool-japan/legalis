//! Encrypted storage wrapper that encrypts records before storing them.

use crate::encryption::{EncryptedRecord, EncryptionKey};
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Encrypted storage wrapper that encrypts records at rest.
///
/// This wrapper encrypts all audit records before storing them using AES-256-GCM.
/// The record IDs are kept unencrypted for indexing purposes.
pub struct EncryptedStorage {
    /// Encrypted records storage (indexed by record ID)
    records: Arc<Mutex<HashMap<Uuid, EncryptedRecord>>>,
    /// Last hash for chain integrity
    last_hash: Arc<Mutex<Option<String>>>,
    /// Encryption key
    key: EncryptionKey,
}

impl EncryptedStorage {
    /// Creates a new encrypted storage with the given encryption key.
    ///
    /// # Example
    /// ```
    /// use legalis_audit::storage::encrypted::EncryptedStorage;
    /// use legalis_audit::encryption::EncryptionKey;
    ///
    /// let key = EncryptionKey::generate();
    /// let storage = EncryptedStorage::new(key);
    /// ```
    pub fn new(key: EncryptionKey) -> Self {
        Self {
            records: Arc::new(Mutex::new(HashMap::new())),
            last_hash: Arc::new(Mutex::new(None)),
            key,
        }
    }

    /// Returns a reference to the encryption key.
    pub fn key(&self) -> &EncryptionKey {
        &self.key
    }

    /// Exports all encrypted records (for backup/migration).
    pub fn export_encrypted(&self) -> AuditResult<Vec<EncryptedRecord>> {
        let records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;
        Ok(records.values().cloned().collect())
    }

    /// Imports encrypted records (for restore/migration).
    pub fn import_encrypted(&mut self, encrypted_records: Vec<EncryptedRecord>) -> AuditResult<()> {
        let mut records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;

        for encrypted in encrypted_records {
            let id = Uuid::parse_str(&encrypted.record_id)
                .map_err(|e| AuditError::StorageError(format!("Invalid record ID: {}", e)))?;
            records.insert(id, encrypted);
        }

        Ok(())
    }
}

impl super::AuditStorage for EncryptedStorage {
    fn store(&mut self, record: AuditRecord) -> AuditResult<()> {
        let encrypted = self.key.encrypt(&record)?;
        let id = record.id;

        let mut records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;

        records.insert(id, encrypted);
        Ok(())
    }

    fn get(&self, id: Uuid) -> AuditResult<AuditRecord> {
        let records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;

        let encrypted = records.get(&id).ok_or(AuditError::RecordNotFound(id))?;

        self.key.decrypt(encrypted)
    }

    fn get_all(&self) -> AuditResult<Vec<AuditRecord>> {
        let records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;

        records
            .values()
            .map(|encrypted| self.key.decrypt(encrypted))
            .collect()
    }

    fn get_by_statute(&self, statute_id: &str) -> AuditResult<Vec<AuditRecord>> {
        let all_records = self.get_all()?;
        Ok(all_records
            .into_iter()
            .filter(|r| r.statute_id == statute_id)
            .collect())
    }

    fn get_by_subject(&self, subject_id: Uuid) -> AuditResult<Vec<AuditRecord>> {
        let all_records = self.get_all()?;
        Ok(all_records
            .into_iter()
            .filter(|r| r.subject_id == subject_id)
            .collect())
    }

    fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> AuditResult<Vec<AuditRecord>> {
        let all_records = self.get_all()?;
        Ok(all_records
            .into_iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .collect())
    }

    fn count(&self) -> AuditResult<usize> {
        let records = self
            .records
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;
        Ok(records.len())
    }

    fn get_last_hash(&self) -> AuditResult<Option<String>> {
        let last_hash = self
            .last_hash
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;
        Ok(last_hash.clone())
    }

    fn set_last_hash(&mut self, hash: Option<String>) -> AuditResult<()> {
        let mut last_hash = self
            .last_hash
            .lock()
            .map_err(|e| AuditError::StorageError(format!("Failed to lock storage: {}", e)))?;
        *last_hash = hash;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::AuditStorage;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record(statute_id: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute_id.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_encrypted_storage() {
        let key = EncryptionKey::generate();
        let mut storage = EncryptedStorage::new(key);

        let record = create_test_record("statute-1");
        let id = record.id;

        storage.store(record).unwrap();
        assert_eq!(storage.count().unwrap(), 1);

        let retrieved = storage.get(id).unwrap();
        assert_eq!(retrieved.id, id);
        assert_eq!(retrieved.statute_id, "statute-1");
    }

    #[test]
    fn test_encrypted_query_by_statute() {
        let key = EncryptionKey::generate();
        let mut storage = EncryptedStorage::new(key);

        for i in 0..3 {
            let record = create_test_record(&format!("statute-{}", i));
            storage.store(record).unwrap();
        }

        let results = storage.get_by_statute("statute-1").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].statute_id, "statute-1");
    }

    #[test]
    fn test_encrypted_export_import() {
        let key = EncryptionKey::generate();
        let mut storage = EncryptedStorage::new(key);

        let records: Vec<_> = (0..5)
            .map(|i| create_test_record(&format!("statute-{}", i)))
            .collect();

        for record in &records {
            storage.store(record.clone()).unwrap();
        }

        // Export encrypted records
        let exported = storage.export_encrypted().unwrap();
        assert_eq!(exported.len(), 5);

        // Create new storage with same key
        let key2_str = storage.key().to_base64();
        let key2 = EncryptionKey::from_base64(&key2_str).unwrap();
        let mut storage2 = EncryptedStorage::new(key2);

        // Import
        storage2.import_encrypted(exported).unwrap();
        assert_eq!(storage2.count().unwrap(), 5);

        // Verify all records can be decrypted
        let all_records = storage2.get_all().unwrap();
        assert_eq!(all_records.len(), 5);
    }

    #[test]
    fn test_encrypted_last_hash() {
        let key = EncryptionKey::generate();
        let mut storage = EncryptedStorage::new(key);

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
