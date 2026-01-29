//! Security and Privacy features for legalis-llm v0.5.5
//!
//! This module provides comprehensive security and privacy capabilities including:
//! - End-to-end encryption for sensitive data
//! - Secure credential management
//! - Audit trail for all operations
//! - Data retention policies
//! - GDPR compliance utilities
//! - Anonymization pipelines
//! - Access control and permissions
//! - Secure multi-tenancy

use anyhow::{Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Encrypted data container with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: String,
    pub nonce: String,
    pub algorithm: String,
    pub key_id: String,
    pub metadata: HashMap<String, String>,
}

/// Simple encryption service (in production, use proper crypto libraries)
pub struct EncryptionService {
    keys: Arc<RwLock<HashMap<String, String>>>,
    algorithm: String,
}

impl EncryptionService {
    /// Create a new encryption service
    pub fn new(algorithm: String) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            algorithm,
        }
    }

    /// Register encryption key
    pub async fn register_key(&self, key_id: String, key: String) -> Result<()> {
        let mut keys = self.keys.write().await;
        keys.insert(key_id, key);
        Ok(())
    }

    /// Encrypt data
    pub async fn encrypt(&self, data: &str, key_id: &str) -> Result<EncryptedData> {
        let keys = self.keys.read().await;
        let key = keys.get(key_id).context("Key not found")?;

        // Simple XOR encryption for demonstration (use proper crypto in production)
        let ciphertext = self.xor_encrypt(data, key);
        let nonce = uuid::Uuid::new_v4().to_string();

        Ok(EncryptedData {
            ciphertext,
            nonce,
            algorithm: self.algorithm.clone(),
            key_id: key_id.to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Decrypt data
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> Result<String> {
        let keys = self.keys.read().await;
        let key = keys.get(&encrypted.key_id).context("Key not found")?;

        // Simple XOR decryption (use proper crypto in production)
        let plaintext = self.xor_decrypt(&encrypted.ciphertext, key);
        Ok(plaintext)
    }

    fn xor_encrypt(&self, data: &str, key: &str) -> String {
        let key_bytes = key.as_bytes();
        let result: Vec<u8> = data
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();
        base64::engine::general_purpose::STANDARD.encode(&result)
    }

    fn xor_decrypt(&self, ciphertext: &str, key: &str) -> String {
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(ciphertext)
            .unwrap_or_default();
        let key_bytes = key.as_bytes();
        let result: Vec<u8> = decoded
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();
        String::from_utf8(result).unwrap_or_default()
    }
}

/// Secure credential manager
pub struct CredentialManager {
    credentials: Arc<RwLock<HashMap<String, SecureCredential>>>,
    encryption: Arc<EncryptionService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCredential {
    pub id: String,
    pub encrypted_value: EncryptedData,
    pub credential_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl CredentialManager {
    /// Create a new credential manager
    pub fn new(encryption: Arc<EncryptionService>) -> Self {
        Self {
            credentials: Arc::new(RwLock::new(HashMap::new())),
            encryption,
        }
    }

    /// Store credential securely
    pub async fn store(
        &self,
        id: String,
        value: &str,
        credential_type: String,
        key_id: &str,
    ) -> Result<()> {
        let encrypted = self.encryption.encrypt(value, key_id).await?;

        let credential = SecureCredential {
            id: id.clone(),
            encrypted_value: encrypted,
            credential_type,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };

        let mut credentials = self.credentials.write().await;
        credentials.insert(id, credential);

        info!("Stored credential securely");
        Ok(())
    }

    /// Retrieve credential
    pub async fn retrieve(&self, id: &str) -> Result<String> {
        let credentials = self.credentials.read().await;
        let credential = credentials.get(id).context("Credential not found")?;

        // Check expiration
        if let Some(expires_at) = credential.expires_at
            && expires_at < chrono::Utc::now()
        {
            anyhow::bail!("Credential expired");
        }

        self.encryption.decrypt(&credential.encrypted_value).await
    }

    /// Delete credential
    pub async fn delete(&self, id: &str) -> Result<()> {
        let mut credentials = self.credentials.write().await;
        credentials.remove(id).context("Credential not found")?;
        info!("Deleted credential: {}", id);
        Ok(())
    }

    /// List credential IDs
    pub async fn list_ids(&self) -> Vec<String> {
        let credentials = self.credentials.read().await;
        credentials.keys().cloned().collect()
    }
}

/// Audit trail for tracking operations
pub struct AuditTrail {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub tenant_id: Option<String>,
    pub operation: String,
    pub resource: String,
    pub result: AuditResult,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure { reason: String },
}

impl AuditTrail {
    /// Create a new audit trail
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            max_entries,
        }
    }

    /// Log an operation
    pub async fn log(
        &self,
        user_id: String,
        operation: String,
        resource: String,
        result: AuditResult,
    ) -> Result<()> {
        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            user_id,
            tenant_id: None,
            operation,
            resource,
            result,
            metadata: HashMap::new(),
        };

        let mut entries = self.entries.write().await;

        // Maintain size limit
        if entries.len() >= self.max_entries {
            entries.remove(0);
        }

        entries.push(entry);
        Ok(())
    }

    /// Log with tenant ID
    pub async fn log_with_tenant(
        &self,
        user_id: String,
        tenant_id: String,
        operation: String,
        resource: String,
        result: AuditResult,
    ) -> Result<()> {
        let entry = AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            user_id,
            tenant_id: Some(tenant_id),
            operation,
            resource,
            result,
            metadata: HashMap::new(),
        };

        let mut entries = self.entries.write().await;

        if entries.len() >= self.max_entries {
            entries.remove(0);
        }

        entries.push(entry);
        Ok(())
    }

    /// Query audit trail
    pub async fn query(&self, user_id: Option<&str>, operation: Option<&str>) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;

        entries
            .iter()
            .filter(|e| {
                let user_match = user_id.is_none_or(|id| e.user_id == id);
                let op_match = operation.is_none_or(|op| e.operation == op);
                user_match && op_match
            })
            .cloned()
            .collect()
    }

    /// Get recent entries
    pub async fn get_recent(&self, limit: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        let start = entries.len().saturating_sub(limit);
        entries[start..].to_vec()
    }

    /// Get entry count
    pub async fn count(&self) -> usize {
        let entries = self.entries.read().await;
        entries.len()
    }
}

/// Data retention policy manager
pub struct RetentionPolicyManager {
    policies: Arc<RwLock<HashMap<String, RetentionPolicy>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub policy_id: String,
    pub data_type: String,
    pub retention_days: u32,
    pub auto_delete: bool,
    pub archive_before_delete: bool,
}

impl RetentionPolicyManager {
    /// Create a new retention policy manager
    pub fn new() -> Self {
        Self {
            policies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add retention policy
    pub async fn add_policy(&self, policy: RetentionPolicy) -> Result<()> {
        let mut policies = self.policies.write().await;
        policies.insert(policy.policy_id.clone(), policy);
        info!("Added retention policy");
        Ok(())
    }

    /// Get policy for data type
    pub async fn get_policy(&self, data_type: &str) -> Option<RetentionPolicy> {
        let policies = self.policies.read().await;
        policies
            .values()
            .find(|p| p.data_type == data_type)
            .cloned()
    }

    /// Check if data should be retained
    pub async fn should_retain(
        &self,
        data_type: &str,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> bool {
        if let Some(policy) = self.get_policy(data_type).await {
            let age_days = (chrono::Utc::now() - created_at).num_days();
            age_days < policy.retention_days as i64
        } else {
            true // No policy means retain indefinitely
        }
    }

    /// List all policies
    pub async fn list_policies(&self) -> Vec<RetentionPolicy> {
        let policies = self.policies.read().await;
        policies.values().cloned().collect()
    }
}

impl Default for RetentionPolicyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// GDPR compliance utilities
pub struct GDPRCompliance {
    data_subject_requests: Arc<RwLock<Vec<DataSubjectRequest>>>,
    consent_records: Arc<RwLock<HashMap<String, ConsentRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRequest {
    pub request_id: String,
    pub subject_id: String,
    pub request_type: RequestType,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    pub status: RequestStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    AccessRequest,
    RightToBeForgotten,
    DataPortability,
    Rectification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Processing,
    Completed,
    Rejected { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub subject_id: String,
    pub purpose: String,
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub withdrawn: bool,
}

impl GDPRCompliance {
    /// Create a new GDPR compliance manager
    pub fn new() -> Self {
        Self {
            data_subject_requests: Arc::new(RwLock::new(Vec::new())),
            consent_records: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Submit data subject request
    pub async fn submit_request(
        &self,
        subject_id: String,
        request_type: RequestType,
    ) -> Result<String> {
        let request = DataSubjectRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            subject_id,
            request_type,
            submitted_at: chrono::Utc::now(),
            status: RequestStatus::Pending,
        };

        let request_id = request.request_id.clone();
        let mut requests = self.data_subject_requests.write().await;
        requests.push(request);

        info!("Submitted GDPR request: {}", request_id);
        Ok(request_id)
    }

    /// Record consent
    pub async fn record_consent(
        &self,
        subject_id: String,
        purpose: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<()> {
        let consent = ConsentRecord {
            subject_id: subject_id.clone(),
            purpose: purpose.clone(),
            granted_at: chrono::Utc::now(),
            expires_at,
            withdrawn: false,
        };

        let mut consents = self.consent_records.write().await;
        let key = format!("{}:{}", subject_id, purpose);
        consents.insert(key, consent);

        info!("Recorded consent for subject: {}", subject_id);
        Ok(())
    }

    /// Check consent validity
    pub async fn check_consent(&self, subject_id: &str, purpose: &str) -> bool {
        let consents = self.consent_records.read().await;
        let key = format!("{}:{}", subject_id, purpose);

        if let Some(consent) = consents.get(&key) {
            if consent.withdrawn {
                return false;
            }

            if let Some(expires_at) = consent.expires_at {
                return expires_at > chrono::Utc::now();
            }

            true
        } else {
            false
        }
    }

    /// Withdraw consent
    pub async fn withdraw_consent(&self, subject_id: &str, purpose: &str) -> Result<()> {
        let mut consents = self.consent_records.write().await;
        let key = format!("{}:{}", subject_id, purpose);

        if let Some(consent) = consents.get_mut(&key) {
            consent.withdrawn = true;
            info!("Withdrew consent for subject: {}", subject_id);
            Ok(())
        } else {
            anyhow::bail!("Consent not found")
        }
    }

    /// Get pending requests
    pub async fn get_pending_requests(&self) -> Vec<DataSubjectRequest> {
        let requests = self.data_subject_requests.read().await;
        requests
            .iter()
            .filter(|r| matches!(r.status, RequestStatus::Pending))
            .cloned()
            .collect()
    }
}

impl Default for GDPRCompliance {
    fn default() -> Self {
        Self::new()
    }
}

/// Data anonymization pipeline
pub struct AnonymizationPipeline {
    rules: Vec<AnonymizationRule>,
}

#[derive(Debug, Clone)]
pub struct AnonymizationRule {
    pub field: String,
    pub strategy: AnonymizationStrategy,
}

#[derive(Debug, Clone)]
pub enum AnonymizationStrategy {
    Redact,
    Hash,
    Generalize { pattern: String },
    Suppress,
}

impl AnonymizationPipeline {
    /// Create a new anonymization pipeline
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add anonymization rule
    pub fn add_rule(mut self, rule: AnonymizationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Anonymize data
    pub fn anonymize(&self, data: &mut HashMap<String, String>) -> Result<()> {
        for rule in &self.rules {
            if let Some(value) = data.get_mut(&rule.field) {
                *value = self.apply_strategy(value, &rule.strategy);
            }
        }
        Ok(())
    }

    fn apply_strategy(&self, value: &str, strategy: &AnonymizationStrategy) -> String {
        match strategy {
            AnonymizationStrategy::Redact => "[REDACTED]".to_string(),
            AnonymizationStrategy::Hash => {
                format!("{:x}", md5::compute(value.as_bytes()))
            }
            AnonymizationStrategy::Generalize { pattern } => pattern.clone(),
            AnonymizationStrategy::Suppress => String::new(),
        }
    }
}

impl Default for AnonymizationPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Access control manager
pub struct AccessControlManager {
    permissions: Arc<RwLock<HashMap<String, HashSet<AccessPermission>>>>,
    roles: Arc<RwLock<HashMap<String, AccessRole>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccessPermission {
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRole {
    pub role_id: String,
    pub role_name: String,
    pub permissions: HashSet<AccessPermission>,
}

impl AccessControlManager {
    /// Create a new access control manager
    pub fn new() -> Self {
        Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
            roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Grant permission to user
    pub async fn grant_permission(
        &self,
        user_id: String,
        permission: AccessPermission,
    ) -> Result<()> {
        let mut permissions = self.permissions.write().await;
        permissions
            .entry(user_id.clone())
            .or_insert_with(HashSet::new)
            .insert(permission);
        debug!("Granted permission to user: {}", user_id);
        Ok(())
    }

    /// Check if user has permission
    pub async fn has_permission(&self, user_id: &str, resource: &str, action: &str) -> bool {
        let permissions = self.permissions.read().await;

        if let Some(user_perms) = permissions.get(user_id) {
            user_perms.contains(&AccessPermission {
                resource: resource.to_string(),
                action: action.to_string(),
            })
        } else {
            false
        }
    }

    /// Revoke permission
    pub async fn revoke_permission(
        &self,
        user_id: &str,
        permission: &AccessPermission,
    ) -> Result<()> {
        let mut permissions = self.permissions.write().await;

        if let Some(user_perms) = permissions.get_mut(user_id) {
            user_perms.remove(permission);
            debug!("Revoked permission from user: {}", user_id);
            Ok(())
        } else {
            anyhow::bail!("User not found")
        }
    }

    /// Create role
    pub async fn create_role(&self, role: AccessRole) -> Result<()> {
        let mut roles = self.roles.write().await;
        roles.insert(role.role_id.clone(), role);
        Ok(())
    }

    /// Assign role to user
    pub async fn assign_role(&self, user_id: String, role_id: &str) -> Result<()> {
        let roles = self.roles.read().await;
        let role = roles.get(role_id).context("Role not found")?;

        let mut permissions = self.permissions.write().await;
        let user_perms = permissions.entry(user_id).or_insert_with(HashSet::new);

        for perm in &role.permissions {
            user_perms.insert(perm.clone());
        }

        Ok(())
    }
}

impl Default for AccessControlManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-tenancy manager
pub struct MultiTenancyManager {
    tenants: Arc<RwLock<HashMap<String, Tenant>>>,
    user_tenants: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub tenant_id: String,
    pub name: String,
    pub quota: ResourceQuota,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    pub max_requests_per_day: u32,
    pub max_storage_mb: u32,
    pub max_users: u32,
}

impl MultiTenancyManager {
    /// Create a new multi-tenancy manager
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            user_tenants: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create tenant
    pub async fn create_tenant(&self, tenant: Tenant) -> Result<()> {
        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.tenant_id.clone(), tenant);
        info!("Created tenant");
        Ok(())
    }

    /// Add user to tenant
    pub async fn add_user_to_tenant(&self, user_id: String, tenant_id: String) -> Result<()> {
        let tenants = self.tenants.read().await;
        if !tenants.contains_key(&tenant_id) {
            anyhow::bail!("Tenant not found");
        }

        let mut user_tenants = self.user_tenants.write().await;
        user_tenants
            .entry(user_id)
            .or_insert_with(HashSet::new)
            .insert(tenant_id);

        Ok(())
    }

    /// Check if user belongs to tenant
    pub async fn user_belongs_to_tenant(&self, user_id: &str, tenant_id: &str) -> bool {
        let user_tenants = self.user_tenants.read().await;

        if let Some(tenants) = user_tenants.get(user_id) {
            tenants.contains(tenant_id)
        } else {
            false
        }
    }

    /// Get user tenants
    pub async fn get_user_tenants(&self, user_id: &str) -> Vec<String> {
        let user_tenants = self.user_tenants.read().await;
        user_tenants
            .get(user_id)
            .map(|t| t.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get tenant
    pub async fn get_tenant(&self, tenant_id: &str) -> Option<Tenant> {
        let tenants = self.tenants.read().await;
        tenants.get(tenant_id).cloned()
    }
}

impl Default for MultiTenancyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption_service() {
        let service = EncryptionService::new("XOR".to_string());
        service
            .register_key("key1".to_string(), "secretkey".to_string())
            .await
            .expect("Failed to register key");

        let plaintext = "Hello, World!";
        let encrypted = service
            .encrypt(plaintext, "key1")
            .await
            .expect("Failed to encrypt");

        let decrypted = service
            .decrypt(&encrypted)
            .await
            .expect("Failed to decrypt");
        assert_eq!(plaintext, decrypted);
    }

    #[tokio::test]
    async fn test_credential_manager() {
        let encryption = Arc::new(EncryptionService::new("XOR".to_string()));
        encryption
            .register_key("key1".to_string(), "secretkey".to_string())
            .await
            .expect("Failed to register key");

        let manager = CredentialManager::new(encryption);

        manager
            .store(
                "api_key".to_string(),
                "sk-12345",
                "openai".to_string(),
                "key1",
            )
            .await
            .expect("Failed to store credential");

        let retrieved = manager
            .retrieve("api_key")
            .await
            .expect("Failed to retrieve credential");
        assert_eq!(retrieved, "sk-12345");

        let ids = manager.list_ids().await;
        assert_eq!(ids.len(), 1);

        manager.delete("api_key").await.expect("Failed to delete");
        let ids = manager.list_ids().await;
        assert_eq!(ids.len(), 0);
    }

    #[tokio::test]
    async fn test_audit_trail() {
        let audit = AuditTrail::new(100);

        audit
            .log(
                "user1".to_string(),
                "create".to_string(),
                "document".to_string(),
                AuditResult::Success,
            )
            .await
            .expect("Failed to log");

        let entries = audit.query(Some("user1"), Some("create")).await;
        assert_eq!(entries.len(), 1);

        let recent = audit.get_recent(10).await;
        assert_eq!(recent.len(), 1);

        assert_eq!(audit.count().await, 1);
    }

    #[tokio::test]
    async fn test_audit_trail_with_tenant() {
        let audit = AuditTrail::new(100);

        audit
            .log_with_tenant(
                "user1".to_string(),
                "tenant1".to_string(),
                "read".to_string(),
                "file".to_string(),
                AuditResult::Success,
            )
            .await
            .expect("Failed to log");

        let entries = audit.query(Some("user1"), None).await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].tenant_id, Some("tenant1".to_string()));
    }

    #[tokio::test]
    async fn test_retention_policy_manager() {
        let manager = RetentionPolicyManager::new();

        let policy = RetentionPolicy {
            policy_id: "pol1".to_string(),
            data_type: "logs".to_string(),
            retention_days: 30,
            auto_delete: true,
            archive_before_delete: false,
        };

        manager
            .add_policy(policy)
            .await
            .expect("Failed to add policy");

        let retrieved = manager.get_policy("logs").await;
        assert!(retrieved.is_some());

        let created_recently = chrono::Utc::now() - chrono::Duration::days(10);
        assert!(manager.should_retain("logs", created_recently).await);

        let created_long_ago = chrono::Utc::now() - chrono::Duration::days(40);
        assert!(!manager.should_retain("logs", created_long_ago).await);

        let policies = manager.list_policies().await;
        assert_eq!(policies.len(), 1);
    }

    #[tokio::test]
    async fn test_gdpr_compliance() {
        let gdpr = GDPRCompliance::new();

        let request_id = gdpr
            .submit_request("subject1".to_string(), RequestType::AccessRequest)
            .await
            .expect("Failed to submit request");

        assert!(!request_id.is_empty());

        let pending = gdpr.get_pending_requests().await;
        assert_eq!(pending.len(), 1);

        gdpr.record_consent("subject1".to_string(), "marketing".to_string(), None)
            .await
            .expect("Failed to record consent");

        assert!(gdpr.check_consent("subject1", "marketing").await);

        gdpr.withdraw_consent("subject1", "marketing")
            .await
            .expect("Failed to withdraw consent");

        assert!(!gdpr.check_consent("subject1", "marketing").await);
    }

    #[test]
    fn test_anonymization_pipeline() {
        let pipeline = AnonymizationPipeline::new()
            .add_rule(AnonymizationRule {
                field: "email".to_string(),
                strategy: AnonymizationStrategy::Redact,
            })
            .add_rule(AnonymizationRule {
                field: "ssn".to_string(),
                strategy: AnonymizationStrategy::Hash,
            });

        let mut data = HashMap::new();
        data.insert("email".to_string(), "user@example.com".to_string());
        data.insert("ssn".to_string(), "123-45-6789".to_string());

        pipeline.anonymize(&mut data).expect("Failed to anonymize");

        assert_eq!(data.get("email"), Some(&"[REDACTED]".to_string()));
        assert_ne!(data.get("ssn"), Some(&"123-45-6789".to_string()));
    }

    #[tokio::test]
    async fn test_access_control_manager() {
        let acl = AccessControlManager::new();

        let permission = AccessPermission {
            resource: "document".to_string(),
            action: "read".to_string(),
        };

        acl.grant_permission("user1".to_string(), permission.clone())
            .await
            .expect("Failed to grant permission");

        assert!(acl.has_permission("user1", "document", "read").await);
        assert!(!acl.has_permission("user1", "document", "write").await);

        acl.revoke_permission("user1", &permission)
            .await
            .expect("Failed to revoke permission");

        assert!(!acl.has_permission("user1", "document", "read").await);
    }

    #[tokio::test]
    async fn test_access_control_roles() {
        let acl = AccessControlManager::new();

        let mut permissions = HashSet::new();
        permissions.insert(AccessPermission {
            resource: "document".to_string(),
            action: "read".to_string(),
        });
        permissions.insert(AccessPermission {
            resource: "document".to_string(),
            action: "write".to_string(),
        });

        let role = AccessRole {
            role_id: "editor".to_string(),
            role_name: "Editor".to_string(),
            permissions,
        };

        acl.create_role(role).await.expect("Failed to create role");

        acl.assign_role("user1".to_string(), "editor")
            .await
            .expect("Failed to assign role");

        assert!(acl.has_permission("user1", "document", "read").await);
        assert!(acl.has_permission("user1", "document", "write").await);
    }

    #[tokio::test]
    async fn test_multi_tenancy_manager() {
        let manager = MultiTenancyManager::new();

        let tenant = Tenant {
            tenant_id: "tenant1".to_string(),
            name: "Acme Corp".to_string(),
            quota: ResourceQuota {
                max_requests_per_day: 1000,
                max_storage_mb: 100,
                max_users: 10,
            },
            created_at: chrono::Utc::now(),
        };

        manager
            .create_tenant(tenant)
            .await
            .expect("Failed to create tenant");

        manager
            .add_user_to_tenant("user1".to_string(), "tenant1".to_string())
            .await
            .expect("Failed to add user to tenant");

        assert!(manager.user_belongs_to_tenant("user1", "tenant1").await);
        assert!(!manager.user_belongs_to_tenant("user1", "tenant2").await);

        let tenants = manager.get_user_tenants("user1").await;
        assert_eq!(tenants.len(), 1);

        let retrieved = manager.get_tenant("tenant1").await;
        assert!(retrieved.is_some());
    }
}
