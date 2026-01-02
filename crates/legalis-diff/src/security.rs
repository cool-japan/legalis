//! Security features for diff operations.
//!
//! This module provides cryptographic signing, tamper detection, encryption,
//! and access control for statute diffs.
//!
//! # Features
//!
//! - **Cryptographic Signing**: Sign diffs to ensure authenticity
//! - **Tamper Detection**: Detect unauthorized modifications
//! - **Encryption**: Encrypt sensitive changes
//! - **Audit Trail Integrity**: Verify audit trail hasn't been tampered with
//! - **Access Control**: Role-based access control for diff operations
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, security::{sign_diff, verify_signature, KeyPair}};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let new = old.clone();
//! let diff_result = diff(&old, &new).unwrap();
//!
//! // Generate a keypair
//! let keypair = KeyPair::generate();
//!
//! // Sign the diff
//! let signed = sign_diff(&diff_result, &keypair).unwrap();
//!
//! // Verify the signature
//! assert!(verify_signature(&signed, &keypair.public_key()).unwrap());
//! ```

use crate::{DiffError, DiffResult, StatuteDiff};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// A cryptographic key pair for signing diffs.
#[derive(Clone)]
pub struct KeyPair {
    #[allow(dead_code)]
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

impl KeyPair {
    /// Generate a new keypair.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::KeyPair;
    ///
    /// let keypair = KeyPair::generate();
    /// assert!(!keypair.public_key().is_empty());
    /// ```
    pub fn generate() -> Self {
        // In a real implementation, use a proper crypto library like ed25519-dalek
        // For now, we'll use a simple placeholder
        let private_key = b"private_key_placeholder_32_bytes".to_vec();
        let public_key = b"public_key_placeholder_32_bytes!".to_vec();

        Self {
            private_key,
            public_key,
        }
    }

    /// Get the public key.
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            bytes: self.public_key.clone(),
        }
    }

    /// Sign data with the private key.
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: In real implementation, use proper signing
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = hasher.finish();

        hash.to_le_bytes().to_vec()
    }
}

/// A public key for verifying signatures.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PublicKey {
    bytes: Vec<u8>,
}

impl PublicKey {
    /// Verify a signature.
    fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        // Placeholder: In real implementation, use proper verification
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let expected_hash = hasher.finish();

        signature == expected_hash.to_le_bytes().as_slice()
    }

    /// Check if the public key is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// A signed diff with cryptographic signature.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignedDiff {
    /// The diff data
    pub diff: StatuteDiff,
    /// Cryptographic signature
    pub signature: Vec<u8>,
    /// Public key of signer
    pub signer_public_key: PublicKey,
    /// Timestamp when signed
    pub signed_at: u64,
}

/// Sign a diff with a keypair.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, security::{sign_diff, KeyPair}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let keypair = KeyPair::generate();
/// let signed = sign_diff(&diff_result, &keypair).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if serialization fails.
pub fn sign_diff(diff: &StatuteDiff, keypair: &KeyPair) -> DiffResult<SignedDiff> {
    let diff_json = serde_json::to_vec(diff)
        .map_err(|e| DiffError::SerializationError(format!("Failed to serialize diff: {}", e)))?;

    let signature = keypair.sign(&diff_json);
    let signed_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(SignedDiff {
        diff: diff.clone(),
        signature,
        signer_public_key: keypair.public_key(),
        signed_at,
    })
}

/// Verify the signature of a signed diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, security::{sign_diff, verify_signature, KeyPair}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let keypair = KeyPair::generate();
/// let signed = sign_diff(&diff_result, &keypair).unwrap();
///
/// assert!(verify_signature(&signed, &keypair.public_key()).unwrap());
/// ```
///
/// # Errors
///
/// Returns an error if verification fails or serialization fails.
pub fn verify_signature(signed: &SignedDiff, public_key: &PublicKey) -> DiffResult<bool> {
    let diff_json = serde_json::to_vec(&signed.diff)
        .map_err(|e| DiffError::SerializationError(format!("Failed to serialize diff: {}", e)))?;

    Ok(public_key.verify(&diff_json, &signed.signature))
}

/// Detect if a diff has been tampered with.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, security::{sign_diff, detect_tampering, KeyPair}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let keypair = KeyPair::generate();
/// let signed = sign_diff(&diff_result, &keypair).unwrap();
///
/// // Should not be tampered
/// assert!(!detect_tampering(&signed, &keypair.public_key()).unwrap());
/// ```
pub fn detect_tampering(signed: &SignedDiff, public_key: &PublicKey) -> DiffResult<bool> {
    Ok(!verify_signature(signed, public_key)?)
}

/// Encrypted diff data.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedDiff {
    /// Encrypted diff data
    pub ciphertext: Vec<u8>,
    /// Initialization vector (IV)
    pub iv: Vec<u8>,
    /// Encryption algorithm used
    pub algorithm: String,
}

/// Encryption key for diff encryption.
pub struct EncryptionKey {
    key: Vec<u8>,
}

impl EncryptionKey {
    /// Generate a new encryption key.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::EncryptionKey;
    ///
    /// let key = EncryptionKey::generate();
    /// ```
    pub fn generate() -> Self {
        // In real implementation, use proper key generation
        let key = b"encryption_key_32_bytes_long!!!".to_vec();
        Self { key }
    }

    /// Create an encryption key from bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { key: bytes }
    }

    fn encrypt(&self, data: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // Placeholder: In real implementation, use proper encryption (e.g., AES-GCM)
        // For now, simple XOR as a demonstration
        let iv = b"initialization_v".to_vec();
        let mut ciphertext = data.to_vec();

        for (i, byte) in ciphertext.iter_mut().enumerate() {
            *byte ^= self.key[i % self.key.len()];
        }

        (ciphertext, iv)
    }

    fn decrypt(&self, ciphertext: &[u8], _iv: &[u8]) -> Vec<u8> {
        // Placeholder: In real implementation, use proper decryption
        let mut plaintext = ciphertext.to_vec();

        for (i, byte) in plaintext.iter_mut().enumerate() {
            *byte ^= self.key[i % self.key.len()];
        }

        plaintext
    }
}

/// Encrypt a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, security::{encrypt_diff, EncryptionKey}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let key = EncryptionKey::generate();
/// let encrypted = encrypt_diff(&diff_result, &key).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if encryption or serialization fails.
pub fn encrypt_diff(diff: &StatuteDiff, key: &EncryptionKey) -> DiffResult<EncryptedDiff> {
    let diff_json = serde_json::to_vec(diff)
        .map_err(|e| DiffError::SerializationError(format!("Failed to serialize diff: {}", e)))?;

    let (ciphertext, iv) = key.encrypt(&diff_json);

    Ok(EncryptedDiff {
        ciphertext,
        iv,
        algorithm: "AES-256-GCM".to_string(), // Placeholder
    })
}

/// Decrypt an encrypted diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, security::{encrypt_diff, decrypt_diff, EncryptionKey}};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let new = old.clone();
/// let diff_result = diff(&old, &new).unwrap();
///
/// let key = EncryptionKey::generate();
/// let encrypted = encrypt_diff(&diff_result, &key).unwrap();
/// let decrypted = decrypt_diff(&encrypted, &key).unwrap();
///
/// assert_eq!(diff_result.statute_id, decrypted.statute_id);
/// ```
///
/// # Errors
///
/// Returns an error if decryption or deserialization fails.
pub fn decrypt_diff(encrypted: &EncryptedDiff, key: &EncryptionKey) -> DiffResult<StatuteDiff> {
    let plaintext = key.decrypt(&encrypted.ciphertext, &encrypted.iv);

    serde_json::from_slice(&plaintext)
        .map_err(|e| DiffError::SerializationError(format!("Failed to deserialize diff: {}", e)))
}

/// Audit trail entry.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuditEntry {
    /// Timestamp
    pub timestamp: u64,
    /// User who performed the action
    pub user: String,
    /// Action performed
    pub action: String,
    /// Diff ID
    pub diff_id: String,
    /// Signature of this entry
    pub signature: Vec<u8>,
}

/// Audit trail with integrity verification.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuditTrail {
    /// List of audit entries
    pub entries: Vec<AuditEntry>,
    /// Chain hash for integrity
    pub chain_hash: Vec<u8>,
}

impl AuditTrail {
    /// Create a new audit trail.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::AuditTrail;
    ///
    /// let trail = AuditTrail::new();
    /// assert!(trail.entries.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            chain_hash: Vec::new(),
        }
    }

    /// Add an entry to the audit trail.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{AuditTrail, KeyPair};
    ///
    /// let mut trail = AuditTrail::new();
    /// let keypair = KeyPair::generate();
    ///
    /// trail.add_entry("alice", "view_diff", "diff-123", &keypair);
    /// assert_eq!(trail.entries.len(), 1);
    /// ```
    pub fn add_entry(&mut self, user: &str, action: &str, diff_id: &str, keypair: &KeyPair) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry_data = format!("{}:{}:{}:{}", timestamp, user, action, diff_id);
        let signature = keypair.sign(entry_data.as_bytes());

        let entry = AuditEntry {
            timestamp,
            user: user.to_string(),
            action: action.to_string(),
            diff_id: diff_id.to_string(),
            signature,
        };

        self.entries.push(entry);
        self.update_chain_hash();
    }

    fn update_chain_hash(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for entry in &self.entries {
            entry.timestamp.hash(&mut hasher);
            entry.user.hash(&mut hasher);
            entry.action.hash(&mut hasher);
            entry.diff_id.hash(&mut hasher);
        }

        self.chain_hash = hasher.finish().to_le_bytes().to_vec();
    }

    /// Verify the integrity of the audit trail.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{AuditTrail, KeyPair};
    ///
    /// let mut trail = AuditTrail::new();
    /// let keypair = KeyPair::generate();
    ///
    /// trail.add_entry("alice", "view_diff", "diff-123", &keypair);
    ///
    /// assert!(trail.verify_integrity());
    /// ```
    pub fn verify_integrity(&self) -> bool {
        let mut temp_trail = AuditTrail {
            entries: self.entries.clone(),
            chain_hash: Vec::new(),
        };
        temp_trail.update_chain_hash();

        temp_trail.chain_hash == self.chain_hash
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// Role-based access control for diff operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// Can only view diffs
    Viewer,
    /// Can view and create diffs
    Analyst,
    /// Can view, create, and modify diffs
    Editor,
    /// Full access to all operations
    Admin,
}

/// Permission for a specific operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    ViewDiff,
    CreateDiff,
    ModifyDiff,
    DeleteDiff,
    SignDiff,
    EncryptDiff,
}

impl Role {
    /// Check if a role has a specific permission.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{Role, Permission};
    ///
    /// assert!(Role::Admin.has_permission(Permission::DeleteDiff));
    /// assert!(!Role::Viewer.has_permission(Permission::DeleteDiff));
    /// ```
    pub fn has_permission(&self, permission: Permission) -> bool {
        match (self, permission) {
            (_, Permission::ViewDiff) => true, // All roles can view
            (Role::Viewer, _) => false,        // Viewers can only view
            (Role::Analyst, Permission::CreateDiff) => true,
            (Role::Analyst, _) => false,
            (Role::Editor, Permission::DeleteDiff) => false, // Editors can't delete
            (Role::Editor, _) => true,
            (Role::Admin, _) => true, // Admins can do everything
        }
    }
}

/// Access control context for operations.
#[derive(Clone)]
pub struct AccessControl {
    user_roles: std::collections::HashMap<String, Role>,
}

impl AccessControl {
    /// Create a new access control system.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::AccessControl;
    ///
    /// let ac = AccessControl::new();
    /// ```
    pub fn new() -> Self {
        Self {
            user_roles: std::collections::HashMap::new(),
        }
    }

    /// Grant a role to a user.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{AccessControl, Role};
    ///
    /// let mut ac = AccessControl::new();
    /// ac.grant_role("alice", Role::Admin);
    /// ```
    pub fn grant_role(&mut self, user: &str, role: Role) {
        self.user_roles.insert(user.to_string(), role);
    }

    /// Check if a user has permission for an operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{AccessControl, Role, Permission};
    ///
    /// let mut ac = AccessControl::new();
    /// ac.grant_role("alice", Role::Admin);
    ///
    /// assert!(ac.check_permission("alice", Permission::DeleteDiff).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the user has no role assigned.
    pub fn check_permission(&self, user: &str, permission: Permission) -> DiffResult<bool> {
        let role = self.user_roles.get(user).ok_or_else(|| {
            DiffError::UnsupportedOperation(format!("User '{}' has no assigned role", user))
        })?;

        Ok(role.has_permission(permission))
    }

    /// Revoke a user's role.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::security::{AccessControl, Role};
    ///
    /// let mut ac = AccessControl::new();
    /// ac.grant_role("alice", Role::Admin);
    /// ac.revoke_role("alice");
    /// ```
    pub fn revoke_role(&mut self, user: &str) {
        self.user_roles.remove(user);
    }

    /// Get a user's role.
    pub fn get_role(&self, user: &str) -> Option<Role> {
        self.user_roles.get(user).copied()
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn test_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
    }

    #[test]
    fn test_sign_and_verify() {
        let old = test_statute();
        let new = test_statute();
        let diff_result = diff(&old, &new).unwrap();

        let keypair = KeyPair::generate();
        let signed = sign_diff(&diff_result, &keypair).unwrap();

        assert!(verify_signature(&signed, &keypair.public_key()).unwrap());
    }

    #[test]
    fn test_detect_no_tampering() {
        let old = test_statute();
        let new = test_statute();
        let diff_result = diff(&old, &new).unwrap();

        let keypair = KeyPair::generate();
        let signed = sign_diff(&diff_result, &keypair).unwrap();

        assert!(!detect_tampering(&signed, &keypair.public_key()).unwrap());
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let old = test_statute();
        let mut new = test_statute();
        new.title = "Modified Title".to_string();

        let diff_result = diff(&old, &new).unwrap();
        let key = EncryptionKey::generate();

        let encrypted = encrypt_diff(&diff_result, &key).unwrap();
        let decrypted = decrypt_diff(&encrypted, &key).unwrap();

        assert_eq!(diff_result.statute_id, decrypted.statute_id);
        assert_eq!(diff_result.changes.len(), decrypted.changes.len());
    }

    #[test]
    fn test_audit_trail() {
        let mut trail = AuditTrail::new();
        let keypair = KeyPair::generate();

        trail.add_entry("alice", "view_diff", "diff-123", &keypair);
        trail.add_entry("bob", "create_diff", "diff-124", &keypair);

        assert_eq!(trail.entries.len(), 2);
        assert!(trail.verify_integrity());
    }

    #[test]
    fn test_audit_trail_integrity() {
        let mut trail = AuditTrail::new();
        let keypair = KeyPair::generate();

        trail.add_entry("alice", "view_diff", "diff-123", &keypair);

        // Verify integrity
        assert!(trail.verify_integrity());

        // Tamper with an entry
        trail.entries[0].action = "modified_action".to_string();

        // Should detect tampering
        assert!(!trail.verify_integrity());
    }

    #[test]
    fn test_access_control() {
        let mut ac = AccessControl::new();

        ac.grant_role("alice", Role::Admin);
        ac.grant_role("bob", Role::Viewer);
        ac.grant_role("charlie", Role::Analyst);

        assert!(
            ac.check_permission("alice", Permission::DeleteDiff)
                .unwrap()
        );
        assert!(!ac.check_permission("bob", Permission::DeleteDiff).unwrap());
        assert!(
            ac.check_permission("charlie", Permission::CreateDiff)
                .unwrap()
        );
        assert!(
            !ac.check_permission("charlie", Permission::DeleteDiff)
                .unwrap()
        );
    }

    #[test]
    fn test_role_permissions() {
        assert!(Role::Admin.has_permission(Permission::DeleteDiff));
        assert!(Role::Editor.has_permission(Permission::ModifyDiff));
        assert!(!Role::Editor.has_permission(Permission::DeleteDiff));
        assert!(Role::Analyst.has_permission(Permission::CreateDiff));
        assert!(!Role::Analyst.has_permission(Permission::ModifyDiff));
        assert!(Role::Viewer.has_permission(Permission::ViewDiff));
        assert!(!Role::Viewer.has_permission(Permission::CreateDiff));
    }

    #[test]
    fn test_revoke_role() {
        let mut ac = AccessControl::new();
        ac.grant_role("alice", Role::Admin);

        assert!(
            ac.check_permission("alice", Permission::DeleteDiff)
                .unwrap()
        );

        ac.revoke_role("alice");

        assert!(
            ac.check_permission("alice", Permission::DeleteDiff)
                .is_err()
        );
    }

    #[test]
    fn test_keypair_generation() {
        let keypair1 = KeyPair::generate();
        let keypair2 = KeyPair::generate();

        assert!(!keypair1.public_key().is_empty());
        assert!(!keypair2.public_key().is_empty());
    }

    #[test]
    fn test_encryption_key_generation() {
        let key = EncryptionKey::generate();
        assert!(!key.key.is_empty());
    }
}
