//! Digital evidence packaging for forensic use.
//!
//! This module provides digital evidence packaging capabilities:
//! - Evidence containers with metadata
//! - Digital signatures for evidence integrity
//! - Chain-of-custody integration
//! - Export formats for legal proceedings
//! - Evidence sealing and verification

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Digital evidence package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidencePackage {
    /// Package ID
    pub id: Uuid,
    /// Case identifier
    pub case_id: String,
    /// Package created timestamp
    pub created_at: DateTime<Utc>,
    /// Collector information
    pub collector: EvidenceCollector,
    /// Evidence items
    pub items: Vec<EvidenceItem>,
    /// Package metadata
    pub metadata: HashMap<String, String>,
    /// Digital signature (if sealed)
    pub signature: Option<String>,
    /// Package hash for integrity
    pub package_hash: String,
    /// Sealed status
    pub sealed: bool,
}

/// Evidence collector information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCollector {
    /// Collector ID (badge number, employee ID, etc.)
    pub id: String,
    /// Collector name
    pub name: String,
    /// Organization
    pub organization: String,
    /// Contact information
    pub contact: Option<String>,
}

/// An individual evidence item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    /// Item ID
    pub id: Uuid,
    /// Item type
    pub item_type: EvidenceType,
    /// Description
    pub description: String,
    /// Audit record (if applicable)
    pub audit_record: Option<AuditRecord>,
    /// File path (if file evidence)
    pub file_path: Option<String>,
    /// Item hash
    pub item_hash: String,
    /// Collection timestamp
    pub collected_at: DateTime<Utc>,
}

/// Type of evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Audit trail record
    AuditRecord,
    /// Log file
    LogFile,
    /// Configuration file
    ConfigFile,
    /// Screenshot
    Screenshot,
    /// Database dump
    DatabaseDump,
    /// Other evidence
    Other(String),
}

impl EvidencePackage {
    /// Creates a new evidence package.
    pub fn new(case_id: String, collector: EvidenceCollector) -> Self {
        let mut package = Self {
            id: Uuid::new_v4(),
            case_id,
            created_at: Utc::now(),
            collector,
            items: Vec::new(),
            metadata: HashMap::new(),
            signature: None,
            package_hash: String::new(),
            sealed: false,
        };
        package.package_hash = package.compute_hash();
        package
    }

    /// Adds an evidence item to the package.
    pub fn add_item(&mut self, item: EvidenceItem) -> AuditResult<()> {
        if self.sealed {
            return Err(crate::AuditError::InvalidRecord(
                "Cannot add items to sealed package".to_string(),
            ));
        }

        self.items.push(item);
        self.package_hash = self.compute_hash();
        Ok(())
    }

    /// Adds an audit record as evidence.
    pub fn add_audit_record(
        &mut self,
        record: AuditRecord,
        description: String,
    ) -> AuditResult<()> {
        let item = EvidenceItem {
            id: Uuid::new_v4(),
            item_type: EvidenceType::AuditRecord,
            description,
            audit_record: Some(record.clone()),
            file_path: None,
            item_hash: format!("{:x}", Self::simple_hash(&serde_json::to_string(&record)?)),
            collected_at: Utc::now(),
        };
        self.add_item(item)
    }

    /// Adds metadata to the package.
    pub fn add_metadata(&mut self, key: String, value: String) -> AuditResult<()> {
        if self.sealed {
            return Err(crate::AuditError::InvalidRecord(
                "Cannot modify sealed package".to_string(),
            ));
        }

        self.metadata.insert(key, value);
        self.package_hash = self.compute_hash();
        Ok(())
    }

    /// Seals the package (makes it read-only).
    pub fn seal(&mut self, signature: String) -> AuditResult<()> {
        if self.sealed {
            return Err(crate::AuditError::InvalidRecord(
                "Package already sealed".to_string(),
            ));
        }

        self.signature = Some(signature);
        self.sealed = true;
        self.package_hash = self.compute_hash();
        Ok(())
    }

    /// Verifies the integrity of the package.
    pub fn verify(&self) -> bool {
        let computed = self.compute_hash();
        computed == self.package_hash
    }

    /// Computes the hash for this package.
    fn compute_hash(&self) -> String {
        let mut data = format!("{}{}{}", self.id, self.case_id, self.created_at.timestamp());

        // Include all items
        for item in &self.items {
            data.push_str(&item.item_hash);
        }

        // Include metadata
        let mut keys: Vec<_> = self.metadata.keys().collect();
        keys.sort();
        for key in keys {
            data.push_str(key);
            data.push_str(self.metadata.get(key).unwrap());
        }

        format!("{:x}", Self::simple_hash(&data))
    }

    /// Simple hash implementation.
    fn simple_hash(input: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    /// Exports package to JSON format.
    pub fn to_json(&self) -> AuditResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::AuditError::ExportError(e.to_string()))
    }

    /// Exports package to legal-friendly text format.
    pub fn to_legal_format(&self) -> String {
        let mut output = String::new();

        output.push_str("=== DIGITAL EVIDENCE PACKAGE ===\n\n");
        output.push_str(&format!("Package ID: {}\n", self.id));
        output.push_str(&format!("Case ID: {}\n", self.case_id));
        output.push_str(&format!(
            "Created: {}\n",
            self.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        output.push_str(&format!(
            "Status: {}\n\n",
            if self.sealed { "SEALED" } else { "UNSEALED" }
        ));

        output.push_str("Collector Information:\n");
        output.push_str(&format!("  ID: {}\n", self.collector.id));
        output.push_str(&format!("  Name: {}\n", self.collector.name));
        output.push_str(&format!(
            "  Organization: {}\n",
            self.collector.organization
        ));
        if let Some(contact) = &self.collector.contact {
            output.push_str(&format!("  Contact: {}\n", contact));
        }
        output.push('\n');

        if !self.metadata.is_empty() {
            output.push_str("Package Metadata:\n");
            for (key, value) in &self.metadata {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
            output.push('\n');
        }

        output.push_str(&format!("Evidence Items ({}):\n\n", self.items.len()));
        for (idx, item) in self.items.iter().enumerate() {
            output.push_str(&format!("Item #{}\n", idx + 1));
            output.push_str(&format!("  ID: {}\n", item.id));
            output.push_str(&format!("  Type: {:?}\n", item.item_type));
            output.push_str(&format!("  Description: {}\n", item.description));
            output.push_str(&format!(
                "  Collected: {}\n",
                item.collected_at.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            output.push_str(&format!("  Hash: {}\n", item.item_hash));
            if let Some(path) = &item.file_path {
                output.push_str(&format!("  File: {}\n", path));
            }
            output.push('\n');
        }

        output.push_str(&format!("Package Hash: {}\n", self.package_hash));
        if let Some(signature) = &self.signature {
            output.push_str(&format!("Digital Signature: {}\n", signature));
        }

        output.push_str("\n=== END OF EVIDENCE PACKAGE ===\n");

        output
    }

    /// Creates a manifest file listing all evidence.
    pub fn create_manifest(&self) -> String {
        let mut manifest = String::new();

        manifest.push_str("EVIDENCE MANIFEST\n");
        manifest.push_str(&format!("Package: {}\n", self.id));
        manifest.push_str(&format!("Case: {}\n", self.case_id));
        manifest.push_str(&format!("Date: {}\n\n", self.created_at.format("%Y-%m-%d")));

        manifest.push_str("ITEMS:\n");
        for (idx, item) in self.items.iter().enumerate() {
            manifest.push_str(&format!(
                "{}. {} - {} ({})\n",
                idx + 1,
                item.id,
                item.description,
                item.item_hash
            ));
        }

        manifest.push_str(&format!("\nPackage Hash: {}\n", self.package_hash));
        manifest.push_str(&format!("Sealed: {}\n", self.sealed));

        manifest
    }
}

/// Evidence package builder.
pub struct EvidencePackageBuilder {
    case_id: String,
    collector: EvidenceCollector,
    items: Vec<EvidenceItem>,
    metadata: HashMap<String, String>,
}

impl EvidencePackageBuilder {
    /// Creates a new evidence package builder.
    pub fn new(case_id: String, collector: EvidenceCollector) -> Self {
        Self {
            case_id,
            collector,
            items: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds an audit record to the package.
    pub fn add_audit_record(
        mut self,
        record: AuditRecord,
        description: String,
    ) -> AuditResult<Self> {
        let item_hash = format!(
            "{:x}",
            EvidencePackage::simple_hash(&serde_json::to_string(&record)?)
        );
        let item = EvidenceItem {
            id: Uuid::new_v4(),
            item_type: EvidenceType::AuditRecord,
            description,
            audit_record: Some(record),
            file_path: None,
            item_hash,
            collected_at: Utc::now(),
        };
        self.items.push(item);
        Ok(self)
    }

    /// Adds metadata.
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Builds the evidence package.
    pub fn build(self) -> EvidencePackage {
        let mut package = EvidencePackage::new(self.case_id, self.collector);
        package.items = self.items;
        package.metadata = self.metadata;
        package.package_hash = package.compute_hash();
        package
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_collector() -> EvidenceCollector {
        EvidenceCollector {
            id: "COL001".to_string(),
            name: "John Doe".to_string(),
            organization: "Test Org".to_string(),
            contact: Some("john@test.org".to_string()),
        }
    }

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
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_evidence_package_creation() {
        let collector = create_test_collector();
        let package = EvidencePackage::new("CASE-001".to_string(), collector);

        assert_eq!(package.case_id, "CASE-001");
        assert!(!package.sealed);
        assert!(package.verify());
    }

    #[test]
    fn test_add_audit_record() {
        let collector = create_test_collector();
        let mut package = EvidencePackage::new("CASE-001".to_string(), collector);

        let record = create_test_record();
        package
            .add_audit_record(record, "Test evidence".to_string())
            .unwrap();

        assert_eq!(package.items.len(), 1);
        assert!(package.verify());
    }

    #[test]
    fn test_seal_package() {
        let collector = create_test_collector();
        let mut package = EvidencePackage::new("CASE-001".to_string(), collector);

        package.seal("test-signature".to_string()).unwrap();

        assert!(package.sealed);
        assert_eq!(package.signature, Some("test-signature".to_string()));
    }

    #[test]
    fn test_sealed_package_immutable() {
        let collector = create_test_collector();
        let mut package = EvidencePackage::new("CASE-001".to_string(), collector);

        package.seal("test-signature".to_string()).unwrap();

        let record = create_test_record();
        let result = package.add_audit_record(record, "Test".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_package_builder() {
        let collector = create_test_collector();
        let record = create_test_record();

        let package = EvidencePackageBuilder::new("CASE-001".to_string(), collector)
            .add_audit_record(record, "Test evidence".to_string())
            .unwrap()
            .add_metadata("key".to_string(), "value".to_string())
            .build();

        assert_eq!(package.items.len(), 1);
        assert_eq!(package.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_legal_format_export() {
        let collector = create_test_collector();
        let mut package = EvidencePackage::new("CASE-001".to_string(), collector);

        let record = create_test_record();
        package
            .add_audit_record(record, "Test evidence".to_string())
            .unwrap();

        let legal = package.to_legal_format();

        assert!(legal.contains("DIGITAL EVIDENCE PACKAGE"));
        assert!(legal.contains("CASE-001"));
        assert!(legal.contains("John Doe"));
    }

    #[test]
    fn test_manifest_creation() {
        let collector = create_test_collector();
        let mut package = EvidencePackage::new("CASE-001".to_string(), collector);

        let record = create_test_record();
        package
            .add_audit_record(record, "Test evidence".to_string())
            .unwrap();

        let manifest = package.create_manifest();

        assert!(manifest.contains("EVIDENCE MANIFEST"));
        assert!(manifest.contains("CASE-001"));
    }
}
