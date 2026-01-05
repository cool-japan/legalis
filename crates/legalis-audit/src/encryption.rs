//! Encryption at rest for audit records.
//!
//! Provides AES-256-GCM encryption for audit records to protect sensitive
//! information when stored on disk.

use crate::{AuditError, AuditRecord, AuditResult};
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};

/// Encrypted audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedRecord {
    /// Base64-encoded ciphertext
    pub ciphertext: String,
    /// Base64-encoded nonce (12 bytes)
    pub nonce: String,
    /// Record ID (not encrypted for indexing)
    pub record_id: String,
}

/// Encryption key for audit records.
pub struct EncryptionKey {
    key: Key<Aes256Gcm>,
}

impl EncryptionKey {
    /// Generates a new random encryption key.
    ///
    /// # Example
    /// ```
    /// use legalis_audit::encryption::EncryptionKey;
    ///
    /// let key = EncryptionKey::generate();
    /// ```
    pub fn generate() -> Self {
        let key = Aes256Gcm::generate_key(&mut OsRng);
        Self { key }
    }

    /// Creates a key from a base64-encoded string.
    ///
    /// # Example
    /// ```
    /// use legalis_audit::encryption::EncryptionKey;
    ///
    /// let key = EncryptionKey::generate();
    /// let encoded = key.to_base64();
    /// let restored = EncryptionKey::from_base64(&encoded).unwrap();
    /// ```
    pub fn from_base64(encoded: &str) -> AuditResult<Self> {
        let key_bytes = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| AuditError::StorageError(format!("Invalid key encoding: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(AuditError::StorageError(format!(
                "Invalid key length: expected 32 bytes, got {}",
                key_bytes.len()
            )));
        }

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Self { key: *key })
    }

    /// Converts the key to a base64-encoded string.
    ///
    /// # Security Note
    /// Store this key securely! Anyone with this key can decrypt your audit records.
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.key.as_slice())
    }

    /// Encrypts an audit record.
    pub fn encrypt(&self, record: &AuditRecord) -> AuditResult<EncryptedRecord> {
        let cipher = Aes256Gcm::new(&self.key);

        // Serialize the record
        let plaintext = serde_json::to_vec(record)?;

        // Generate a random nonce
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| AuditError::StorageError(format!("Encryption failed: {}", e)))?;

        Ok(EncryptedRecord {
            ciphertext: general_purpose::STANDARD.encode(&ciphertext),
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            record_id: record.id.to_string(),
        })
    }

    /// Decrypts an encrypted audit record.
    pub fn decrypt(&self, encrypted: &EncryptedRecord) -> AuditResult<AuditRecord> {
        let cipher = Aes256Gcm::new(&self.key);

        // Decode ciphertext and nonce
        let ciphertext = general_purpose::STANDARD
            .decode(&encrypted.ciphertext)
            .map_err(|e| AuditError::StorageError(format!("Invalid ciphertext: {}", e)))?;

        let nonce_bytes = general_purpose::STANDARD
            .decode(&encrypted.nonce)
            .map_err(|e| AuditError::StorageError(format!("Invalid nonce: {}", e)))?;

        if nonce_bytes.len() != 12 {
            return Err(AuditError::StorageError(format!(
                "Invalid nonce length: expected 12 bytes, got {}",
                nonce_bytes.len()
            )));
        }

        let nonce = Nonce::from_slice(&nonce_bytes);

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| AuditError::StorageError(format!("Decryption failed: {}", e)))?;

        // Deserialize
        let record: AuditRecord = serde_json::from_slice(&plaintext)?;

        Ok(record)
    }
}

/// Batch encryption/decryption operations.
pub struct BatchEncryptor {
    key: EncryptionKey,
}

impl BatchEncryptor {
    /// Creates a new batch encryptor with the given key.
    pub fn new(key: EncryptionKey) -> Self {
        Self { key }
    }

    /// Encrypts multiple records in batch.
    pub fn encrypt_batch(&self, records: &[AuditRecord]) -> AuditResult<Vec<EncryptedRecord>> {
        records.iter().map(|r| self.key.encrypt(r)).collect()
    }

    /// Decrypts multiple records in batch.
    pub fn decrypt_batch(&self, encrypted: &[EncryptedRecord]) -> AuditResult<Vec<AuditRecord>> {
        encrypted.iter().map(|e| self.key.decrypt(e)).collect()
    }

    /// Returns the underlying encryption key.
    pub fn key(&self) -> &EncryptionKey {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
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
    fn test_key_generation() {
        let key = EncryptionKey::generate();
        let encoded = key.to_base64();
        assert!(!encoded.is_empty());

        let restored = EncryptionKey::from_base64(&encoded).unwrap();
        assert_eq!(key.to_base64(), restored.to_base64());
    }

    #[test]
    fn test_encryption_decryption() {
        let key = EncryptionKey::generate();
        let record = create_test_record();
        let record_id = record.id;

        let encrypted = key.encrypt(&record).unwrap();
        assert_eq!(encrypted.record_id, record_id.to_string());
        assert!(!encrypted.ciphertext.is_empty());
        assert!(!encrypted.nonce.is_empty());

        let decrypted = key.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted.id, record_id);
        assert_eq!(decrypted.statute_id, record.statute_id);
    }

    #[test]
    fn test_encryption_uniqueness() {
        let key = EncryptionKey::generate();
        let record = create_test_record();

        let encrypted1 = key.encrypt(&record).unwrap();
        let encrypted2 = key.encrypt(&record).unwrap();

        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
        assert_ne!(encrypted1.nonce, encrypted2.nonce);

        // But both should decrypt to the same record
        let decrypted1 = key.decrypt(&encrypted1).unwrap();
        let decrypted2 = key.decrypt(&encrypted2).unwrap();
        assert_eq!(decrypted1.id, decrypted2.id);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = EncryptionKey::generate();
        let key2 = EncryptionKey::generate();
        let record = create_test_record();

        let encrypted = key1.encrypt(&record).unwrap();
        let result = key2.decrypt(&encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_batch_encryption() {
        let key = EncryptionKey::generate();
        let records = vec![
            create_test_record(),
            create_test_record(),
            create_test_record(),
        ];

        let encryptor = BatchEncryptor::new(key);
        let encrypted = encryptor.encrypt_batch(&records).unwrap();
        assert_eq!(encrypted.len(), 3);

        let decrypted = encryptor.decrypt_batch(&encrypted).unwrap();
        assert_eq!(decrypted.len(), 3);

        for (original, decrypted) in records.iter().zip(decrypted.iter()) {
            assert_eq!(original.id, decrypted.id);
            assert_eq!(original.statute_id, decrypted.statute_id);
        }
    }

    #[test]
    fn test_invalid_key_encoding() {
        let result = EncryptionKey::from_base64("not-valid-base64!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_key_length() {
        // Too short key
        let short_key = general_purpose::STANDARD.encode([0u8; 16]);
        let result = EncryptionKey::from_base64(&short_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_ciphertext() {
        let key = EncryptionKey::generate();
        let record = create_test_record();

        let mut encrypted = key.encrypt(&record).unwrap();
        // Tamper with the ciphertext
        encrypted.ciphertext = general_purpose::STANDARD.encode(b"tampered data");

        let result = key.decrypt(&encrypted);
        assert!(result.is_err());
    }
}
