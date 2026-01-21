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

/// Chain of custody entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyEntry {
    /// Entry ID
    pub id: Uuid,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Custodian information
    pub custodian: Custodian,
    /// Action performed
    pub action: CustodyAction,
    /// Location
    pub location: Option<String>,
    /// Notes
    pub notes: Option<String>,
    /// Signature/verification
    pub signature: Option<String>,
}

/// Custodian information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custodian {
    /// Custodian ID
    pub id: String,
    /// Name
    pub name: String,
    /// Role/title
    pub role: String,
    /// Organization
    pub organization: String,
}

/// Custody action type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyAction {
    /// Evidence collected
    Collected,
    /// Evidence transferred
    Transferred { from: String, to: String },
    /// Evidence analyzed
    Analyzed { method: String },
    /// Evidence stored
    Stored { location: String },
    /// Evidence retrieved
    Retrieved { location: String },
    /// Evidence sealed
    Sealed,
    /// Evidence returned
    Returned { to: String },
    /// Other action
    Other(String),
}

/// Chain of custody tracker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCustody {
    /// Evidence ID
    pub evidence_id: Uuid,
    /// Evidence description
    pub description: String,
    /// Chain entries
    pub entries: Vec<CustodyEntry>,
    /// Chain intact (no breaks detected)
    pub intact: bool,
}

impl ChainOfCustody {
    /// Creates a new chain of custody.
    pub fn new(evidence_id: Uuid, description: String) -> Self {
        Self {
            evidence_id,
            description,
            entries: Vec::new(),
            intact: true,
        }
    }

    /// Adds a custody entry.
    pub fn add_entry(&mut self, entry: CustodyEntry) {
        self.entries.push(entry);
    }

    /// Records evidence collection.
    pub fn record_collection(
        &mut self,
        custodian: Custodian,
        location: Option<String>,
        notes: Option<String>,
    ) {
        let entry = CustodyEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            custodian,
            action: CustodyAction::Collected,
            location,
            notes,
            signature: None,
        };
        self.add_entry(entry);
    }

    /// Records evidence transfer.
    pub fn record_transfer(
        &mut self,
        from: String,
        to: String,
        custodian: Custodian,
        notes: Option<String>,
    ) {
        let entry = CustodyEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            custodian,
            action: CustodyAction::Transferred { from, to },
            location: None,
            notes,
            signature: None,
        };
        self.add_entry(entry);
    }

    /// Gets the current custodian.
    pub fn current_custodian(&self) -> Option<&Custodian> {
        self.entries.last().map(|e| &e.custodian)
    }

    /// Validates chain integrity.
    pub fn validate(&self) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        // Check chronological order
        for i in 1..self.entries.len() {
            if self.entries[i].timestamp < self.entries[i - 1].timestamp {
                return false;
            }
        }

        true
    }

    /// Generates chain of custody report.
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== CHAIN OF CUSTODY REPORT ===\n\n");
        report.push_str(&format!("Evidence ID: {}\n", self.evidence_id));
        report.push_str(&format!("Description: {}\n", self.description));
        report.push_str(&format!("Total Entries: {}\n", self.entries.len()));
        report.push_str(&format!(
            "Chain Status: {}\n\n",
            if self.intact { "INTACT" } else { "BROKEN" }
        ));

        report.push_str("CUSTODY HISTORY:\n\n");
        for (idx, entry) in self.entries.iter().enumerate() {
            report.push_str(&format!("Entry #{}\n", idx + 1));
            report.push_str(&format!("  ID: {}\n", entry.id));
            report.push_str(&format!(
                "  Timestamp: {}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            report.push_str(&format!(
                "  Custodian: {} ({})\n",
                entry.custodian.name, entry.custodian.id
            ));
            report.push_str(&format!("  Action: {:?}\n", entry.action));
            if let Some(loc) = &entry.location {
                report.push_str(&format!("  Location: {}\n", loc));
            }
            if let Some(notes) = &entry.notes {
                report.push_str(&format!("  Notes: {}\n", notes));
            }
            report.push('\n');
        }

        report.push_str("=== END OF CHAIN OF CUSTODY ===\n");
        report
    }
}

/// Forensic image metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicImage {
    /// Image ID
    pub id: Uuid,
    /// Source description
    pub source: String,
    /// Image format (DD, E01, AFF, etc.)
    pub format: ImageFormat,
    /// Image file path
    pub file_path: String,
    /// Image size in bytes
    pub size_bytes: u64,
    /// Hash values
    pub hashes: ImageHashes,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Created by
    pub created_by: String,
    /// Imaging tool/method
    pub imaging_tool: String,
    /// Verification status
    pub verified: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Forensic image format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    /// Raw DD format
    DD,
    /// EnCase Evidence File
    E01,
    /// Advanced Forensic Format
    AFF,
    /// Expert Witness Format
    EWF,
    /// Other format
    Other(String),
}

/// Image hash values for integrity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageHashes {
    /// MD5 hash
    pub md5: Option<String>,
    /// SHA1 hash
    pub sha1: Option<String>,
    /// SHA256 hash
    pub sha256: Option<String>,
}

impl ForensicImage {
    /// Creates a new forensic image record.
    pub fn new(
        source: String,
        format: ImageFormat,
        file_path: String,
        size_bytes: u64,
        created_by: String,
        imaging_tool: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            format,
            file_path,
            size_bytes,
            hashes: ImageHashes {
                md5: None,
                sha1: None,
                sha256: None,
            },
            created_at: Utc::now(),
            created_by,
            imaging_tool,
            verified: false,
            metadata: HashMap::new(),
        }
    }

    /// Sets hash values.
    pub fn set_hashes(
        &mut self,
        md5: Option<String>,
        sha1: Option<String>,
        sha256: Option<String>,
    ) {
        self.hashes.md5 = md5;
        self.hashes.sha1 = sha1;
        self.hashes.sha256 = sha256;
    }

    /// Marks image as verified.
    pub fn mark_verified(&mut self) {
        self.verified = true;
    }

    /// Adds metadata.
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Generates imaging report.
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== FORENSIC IMAGE REPORT ===\n\n");
        report.push_str(&format!("Image ID: {}\n", self.id));
        report.push_str(&format!("Source: {}\n", self.source));
        report.push_str(&format!("Format: {:?}\n", self.format));
        report.push_str(&format!("File Path: {}\n", self.file_path));
        report.push_str(&format!("Size: {} bytes\n", self.size_bytes));
        report.push_str(&format!(
            "Created: {}\n",
            self.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        report.push_str(&format!("Created By: {}\n", self.created_by));
        report.push_str(&format!("Imaging Tool: {}\n", self.imaging_tool));
        report.push_str(&format!(
            "Verified: {}\n\n",
            if self.verified { "YES" } else { "NO" }
        ));

        report.push_str("Hash Values:\n");
        if let Some(md5) = &self.hashes.md5 {
            report.push_str(&format!("  MD5: {}\n", md5));
        }
        if let Some(sha1) = &self.hashes.sha1 {
            report.push_str(&format!("  SHA1: {}\n", sha1));
        }
        if let Some(sha256) = &self.hashes.sha256 {
            report.push_str(&format!("  SHA256: {}\n", sha256));
        }

        if !self.metadata.is_empty() {
            report.push_str("\nMetadata:\n");
            for (key, value) in &self.metadata {
                report.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        report.push_str("\n=== END OF REPORT ===\n");
        report
    }
}

/// Legal hold notice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalHold {
    /// Hold ID
    pub id: Uuid,
    /// Case/matter ID
    pub case_id: String,
    /// Hold name/title
    pub title: String,
    /// Description
    pub description: String,
    /// Issued by
    pub issued_by: String,
    /// Issue date
    pub issued_at: DateTime<Utc>,
    /// Expiration date (if any)
    pub expires_at: Option<DateTime<Utc>>,
    /// Status
    pub status: LegalHoldStatus,
    /// Custodians under hold
    pub custodians: Vec<String>,
    /// Scope/criteria
    pub scope: Vec<String>,
    /// Related evidence IDs
    pub evidence_ids: Vec<Uuid>,
}

/// Legal hold status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LegalHoldStatus {
    /// Hold is active
    Active,
    /// Hold is released
    Released,
    /// Hold is suspended
    Suspended,
}

impl LegalHold {
    /// Creates a new legal hold.
    pub fn new(case_id: String, title: String, description: String, issued_by: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            case_id,
            title,
            description,
            issued_by,
            issued_at: Utc::now(),
            expires_at: None,
            status: LegalHoldStatus::Active,
            custodians: Vec::new(),
            scope: Vec::new(),
            evidence_ids: Vec::new(),
        }
    }

    /// Adds a custodian to the hold.
    pub fn add_custodian(&mut self, custodian_id: String) {
        if !self.custodians.contains(&custodian_id) {
            self.custodians.push(custodian_id);
        }
    }

    /// Adds evidence to the hold.
    pub fn add_evidence(&mut self, evidence_id: Uuid) {
        if !self.evidence_ids.contains(&evidence_id) {
            self.evidence_ids.push(evidence_id);
        }
    }

    /// Adds scope criteria.
    pub fn add_scope(&mut self, criteria: String) {
        self.scope.push(criteria);
    }

    /// Releases the hold.
    pub fn release(&mut self) {
        self.status = LegalHoldStatus::Released;
    }

    /// Suspends the hold.
    pub fn suspend(&mut self) {
        self.status = LegalHoldStatus::Suspended;
    }

    /// Checks if hold is active.
    pub fn is_active(&self) -> bool {
        self.status == LegalHoldStatus::Active
    }

    /// Checks if hold has expired.
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Generates legal hold notice.
    pub fn generate_notice(&self) -> String {
        let mut notice = String::new();

        notice.push_str("=== LEGAL HOLD NOTICE ===\n\n");
        notice.push_str(&format!("Hold ID: {}\n", self.id));
        notice.push_str(&format!("Case ID: {}\n", self.case_id));
        notice.push_str(&format!("Title: {}\n", self.title));
        notice.push_str(&format!("Description: {}\n\n", self.description));
        notice.push_str(&format!("Issued By: {}\n", self.issued_by));
        notice.push_str(&format!(
            "Issued Date: {}\n",
            self.issued_at.format("%Y-%m-%d")
        ));
        if let Some(expires) = self.expires_at {
            notice.push_str(&format!("Expires: {}\n", expires.format("%Y-%m-%d")));
        }
        notice.push_str(&format!("Status: {:?}\n\n", self.status));

        if !self.custodians.is_empty() {
            notice.push_str("Custodians:\n");
            for custodian in &self.custodians {
                notice.push_str(&format!("  - {}\n", custodian));
            }
            notice.push('\n');
        }

        if !self.scope.is_empty() {
            notice.push_str("Scope:\n");
            for criteria in &self.scope {
                notice.push_str(&format!("  - {}\n", criteria));
            }
            notice.push('\n');
        }

        notice.push_str(&format!("Evidence Items: {}\n", self.evidence_ids.len()));

        notice.push_str("\n=== END OF NOTICE ===\n");
        notice
    }
}

/// Evidence search query.
#[derive(Debug, Clone)]
pub struct EvidenceSearchQuery {
    /// Case ID filter
    pub case_id: Option<String>,
    /// Evidence type filter
    pub evidence_type: Option<EvidenceType>,
    /// Collector ID filter
    pub collector_id: Option<String>,
    /// Date range start
    pub date_from: Option<DateTime<Utc>>,
    /// Date range end
    pub date_to: Option<DateTime<Utc>>,
    /// Sealed status filter
    pub sealed: Option<bool>,
    /// Text search in descriptions
    pub text_search: Option<String>,
}

impl EvidenceSearchQuery {
    /// Creates a new search query.
    pub fn new() -> Self {
        Self {
            case_id: None,
            evidence_type: None,
            collector_id: None,
            date_from: None,
            date_to: None,
            sealed: None,
            text_search: None,
        }
    }

    /// Sets case ID filter.
    pub fn with_case_id(mut self, case_id: String) -> Self {
        self.case_id = Some(case_id);
        self
    }

    /// Sets collector ID filter.
    pub fn with_collector_id(mut self, collector_id: String) -> Self {
        self.collector_id = Some(collector_id);
        self
    }

    /// Sets date range filter.
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.date_from = Some(from);
        self.date_to = Some(to);
        self
    }

    /// Sets sealed status filter.
    pub fn with_sealed(mut self, sealed: bool) -> Self {
        self.sealed = Some(sealed);
        self
    }

    /// Sets text search filter.
    pub fn with_text_search(mut self, text: String) -> Self {
        self.text_search = Some(text);
        self
    }

    /// Executes search on a collection of packages.
    pub fn search<'a>(&self, packages: &'a [EvidencePackage]) -> Vec<&'a EvidencePackage> {
        packages
            .iter()
            .filter(|pkg| {
                // Case ID filter
                if let Some(ref case_id) = self.case_id
                    && &pkg.case_id != case_id
                {
                    return false;
                }

                // Collector ID filter
                if let Some(ref collector_id) = self.collector_id
                    && &pkg.collector.id != collector_id
                {
                    return false;
                }

                // Date range filter
                if let Some(from) = self.date_from
                    && pkg.created_at < from
                {
                    return false;
                }
                if let Some(to) = self.date_to
                    && pkg.created_at > to
                {
                    return false;
                }

                // Sealed status filter
                if let Some(sealed) = self.sealed
                    && pkg.sealed != sealed
                {
                    return false;
                }

                // Text search filter
                if let Some(ref text) = self.text_search {
                    let search_lower = text.to_lowercase();
                    let mut found = false;

                    // Search in package metadata
                    for (key, value) in &pkg.metadata {
                        if key.to_lowercase().contains(&search_lower)
                            || value.to_lowercase().contains(&search_lower)
                        {
                            found = true;
                            break;
                        }
                    }

                    // Search in items
                    if !found {
                        for item in &pkg.items {
                            if item.description.to_lowercase().contains(&search_lower) {
                                found = true;
                                break;
                            }
                        }
                    }

                    if !found {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}

impl Default for EvidenceSearchQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Evidence export workflow.
#[derive(Debug, Clone)]
pub struct EvidenceExportWorkflow {
    /// Export ID
    pub id: Uuid,
    /// Export format
    pub format: ExportFormat,
    /// Include chain of custody
    pub include_custody: bool,
    /// Include forensic images
    pub include_images: bool,
    /// Encryption enabled
    pub encrypt: bool,
    /// Compression enabled
    pub compress: bool,
}

/// Export format for evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format
    JSON,
    /// Legal text format
    LegalText,
    /// PDF format
    PDF,
    /// ZIP archive
    ZipArchive,
    /// Custom format
    Custom(String),
}

impl EvidenceExportWorkflow {
    /// Creates a new export workflow.
    pub fn new(format: ExportFormat) -> Self {
        Self {
            id: Uuid::new_v4(),
            format,
            include_custody: true,
            include_images: false,
            encrypt: false,
            compress: false,
        }
    }

    /// Sets whether to include chain of custody.
    pub fn with_custody(mut self, include: bool) -> Self {
        self.include_custody = include;
        self
    }

    /// Sets whether to include forensic images.
    pub fn with_images(mut self, include: bool) -> Self {
        self.include_images = include;
        self
    }

    /// Sets whether to encrypt export.
    pub fn with_encryption(mut self, encrypt: bool) -> Self {
        self.encrypt = encrypt;
        self
    }

    /// Sets whether to compress export.
    pub fn with_compression(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// Exports a package according to workflow settings.
    pub fn export_package(&self, package: &EvidencePackage) -> AuditResult<String> {
        match self.format {
            ExportFormat::JSON => package.to_json(),
            ExportFormat::LegalText => Ok(package.to_legal_format()),
            ExportFormat::PDF => {
                // Placeholder for PDF generation
                Ok(format!("PDF export for package {}", package.id))
            }
            ExportFormat::ZipArchive => {
                // Placeholder for ZIP generation
                Ok(format!("ZIP export for package {}", package.id))
            }
            ExportFormat::Custom(ref format) => Ok(format!(
                "Custom export ({}) for package {}",
                format, package.id
            )),
        }
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

    // Chain of Custody tests
    #[test]
    fn test_chain_of_custody_creation() {
        let evidence_id = Uuid::new_v4();
        let chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        assert_eq!(chain.evidence_id, evidence_id);
        assert_eq!(chain.description, "Test Evidence");
        assert_eq!(chain.entries.len(), 0);
        assert!(chain.intact);
    }

    #[test]
    fn test_chain_record_collection() {
        let evidence_id = Uuid::new_v4();
        let mut chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        let custodian = Custodian {
            id: "CUST001".to_string(),
            name: "Jane Doe".to_string(),
            role: "Evidence Technician".to_string(),
            organization: "Test Org".to_string(),
        };

        chain.record_collection(
            custodian,
            Some("Evidence Room A".to_string()),
            Some("Initial collection".to_string()),
        );

        assert_eq!(chain.entries.len(), 1);
        assert!(matches!(chain.entries[0].action, CustodyAction::Collected));
    }

    #[test]
    fn test_chain_record_transfer() {
        let evidence_id = Uuid::new_v4();
        let mut chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        let custodian = Custodian {
            id: "CUST002".to_string(),
            name: "John Smith".to_string(),
            role: "Analyst".to_string(),
            organization: "Test Org".to_string(),
        };

        chain.record_transfer(
            "Jane Doe".to_string(),
            "John Smith".to_string(),
            custodian,
            Some("Transfer for analysis".to_string()),
        );

        assert_eq!(chain.entries.len(), 1);
        if let CustodyAction::Transferred { from, to } = &chain.entries[0].action {
            assert_eq!(from, "Jane Doe");
            assert_eq!(to, "John Smith");
        } else {
            panic!("Expected Transferred action");
        }
    }

    #[test]
    fn test_chain_validation() {
        let evidence_id = Uuid::new_v4();
        let mut chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        let custodian = Custodian {
            id: "CUST001".to_string(),
            name: "Jane Doe".to_string(),
            role: "Technician".to_string(),
            organization: "Test Org".to_string(),
        };

        chain.record_collection(custodian.clone(), None, None);
        chain.record_transfer(
            "Jane Doe".to_string(),
            "John Smith".to_string(),
            custodian,
            None,
        );

        assert!(chain.validate());
    }

    #[test]
    fn test_chain_current_custodian() {
        let evidence_id = Uuid::new_v4();
        let mut chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        let custodian = Custodian {
            id: "CUST001".to_string(),
            name: "Jane Doe".to_string(),
            role: "Technician".to_string(),
            organization: "Test Org".to_string(),
        };

        chain.record_collection(custodian.clone(), None, None);

        let current = chain.current_custodian();
        assert!(current.is_some());
        assert_eq!(current.unwrap().name, "Jane Doe");
    }

    #[test]
    fn test_chain_report_generation() {
        let evidence_id = Uuid::new_v4();
        let mut chain = ChainOfCustody::new(evidence_id, "Test Evidence".to_string());

        let custodian = Custodian {
            id: "CUST001".to_string(),
            name: "Jane Doe".to_string(),
            role: "Technician".to_string(),
            organization: "Test Org".to_string(),
        };

        chain.record_collection(custodian, None, None);

        let report = chain.generate_report();
        assert!(report.contains("CHAIN OF CUSTODY REPORT"));
        assert!(report.contains("Jane Doe"));
    }

    // Forensic Image tests
    #[test]
    fn test_forensic_image_creation() {
        let image = ForensicImage::new(
            "Server HDD".to_string(),
            ImageFormat::DD,
            "/path/to/image.dd".to_string(),
            1073741824, // 1GB
            "forensic-tech".to_string(),
            "dd v8.30".to_string(),
        );

        assert_eq!(image.source, "Server HDD");
        assert!(matches!(image.format, ImageFormat::DD));
        assert_eq!(image.size_bytes, 1073741824);
        assert!(!image.verified);
    }

    #[test]
    fn test_forensic_image_set_hashes() {
        let mut image = ForensicImage::new(
            "Server HDD".to_string(),
            ImageFormat::E01,
            "/path/to/image.e01".to_string(),
            1073741824,
            "forensic-tech".to_string(),
            "ewfacquire".to_string(),
        );

        image.set_hashes(
            Some("abc123".to_string()),
            Some("def456".to_string()),
            Some("789ghi".to_string()),
        );

        assert_eq!(image.hashes.md5, Some("abc123".to_string()));
        assert_eq!(image.hashes.sha1, Some("def456".to_string()));
        assert_eq!(image.hashes.sha256, Some("789ghi".to_string()));
    }

    #[test]
    fn test_forensic_image_verification() {
        let mut image = ForensicImage::new(
            "Test".to_string(),
            ImageFormat::AFF,
            "/path/to/image.aff".to_string(),
            1024,
            "tech".to_string(),
            "affcat".to_string(),
        );

        assert!(!image.verified);
        image.mark_verified();
        assert!(image.verified);
    }

    #[test]
    fn test_forensic_image_metadata() {
        let mut image = ForensicImage::new(
            "Test".to_string(),
            ImageFormat::DD,
            "/path/to/image.dd".to_string(),
            1024,
            "tech".to_string(),
            "dd".to_string(),
        );

        image.add_metadata("device".to_string(), "/dev/sda1".to_string());
        image.add_metadata("case".to_string(), "CASE-123".to_string());

        assert_eq!(image.metadata.get("device"), Some(&"/dev/sda1".to_string()));
        assert_eq!(image.metadata.get("case"), Some(&"CASE-123".to_string()));
    }

    #[test]
    fn test_forensic_image_report() {
        let mut image = ForensicImage::new(
            "Server HDD".to_string(),
            ImageFormat::DD,
            "/path/to/image.dd".to_string(),
            1073741824,
            "forensic-tech".to_string(),
            "dd".to_string(),
        );

        image.set_hashes(Some("abc123".to_string()), None, None);
        image.mark_verified();

        let report = image.generate_report();
        assert!(report.contains("FORENSIC IMAGE REPORT"));
        assert!(report.contains("Server HDD"));
        assert!(report.contains("abc123"));
        assert!(report.contains("YES"));
    }

    // Legal Hold tests
    #[test]
    fn test_legal_hold_creation() {
        let hold = LegalHold::new(
            "CASE-001".to_string(),
            "Document Preservation".to_string(),
            "Preserve all documents related to Project X".to_string(),
            "legal@company.com".to_string(),
        );

        assert_eq!(hold.case_id, "CASE-001");
        assert_eq!(hold.title, "Document Preservation");
        assert_eq!(hold.status, LegalHoldStatus::Active);
        assert!(hold.is_active());
    }

    #[test]
    fn test_legal_hold_add_custodian() {
        let mut hold = LegalHold::new(
            "CASE-001".to_string(),
            "Test Hold".to_string(),
            "Test description".to_string(),
            "legal@company.com".to_string(),
        );

        hold.add_custodian("CUST001".to_string());
        hold.add_custodian("CUST002".to_string());
        hold.add_custodian("CUST001".to_string()); // Duplicate

        assert_eq!(hold.custodians.len(), 2);
    }

    #[test]
    fn test_legal_hold_add_evidence() {
        let mut hold = LegalHold::new(
            "CASE-001".to_string(),
            "Test Hold".to_string(),
            "Test description".to_string(),
            "legal@company.com".to_string(),
        );

        let evidence_id1 = Uuid::new_v4();
        let evidence_id2 = Uuid::new_v4();

        hold.add_evidence(evidence_id1);
        hold.add_evidence(evidence_id2);
        hold.add_evidence(evidence_id1); // Duplicate

        assert_eq!(hold.evidence_ids.len(), 2);
    }

    #[test]
    fn test_legal_hold_release() {
        let mut hold = LegalHold::new(
            "CASE-001".to_string(),
            "Test Hold".to_string(),
            "Test description".to_string(),
            "legal@company.com".to_string(),
        );

        assert!(hold.is_active());
        hold.release();
        assert!(!hold.is_active());
        assert_eq!(hold.status, LegalHoldStatus::Released);
    }

    #[test]
    fn test_legal_hold_suspend() {
        let mut hold = LegalHold::new(
            "CASE-001".to_string(),
            "Test Hold".to_string(),
            "Test description".to_string(),
            "legal@company.com".to_string(),
        );

        hold.suspend();
        assert_eq!(hold.status, LegalHoldStatus::Suspended);
        assert!(!hold.is_active());
    }

    #[test]
    fn test_legal_hold_notice_generation() {
        let mut hold = LegalHold::new(
            "CASE-001".to_string(),
            "Document Preservation".to_string(),
            "Preserve all relevant documents".to_string(),
            "legal@company.com".to_string(),
        );

        hold.add_custodian("John Doe".to_string());
        hold.add_scope("All emails from 2024".to_string());

        let notice = hold.generate_notice();
        assert!(notice.contains("LEGAL HOLD NOTICE"));
        assert!(notice.contains("CASE-001"));
        assert!(notice.contains("John Doe"));
    }

    // Evidence Search tests
    #[test]
    fn test_evidence_search_case_id() {
        let collector = create_test_collector();
        let package1 = EvidencePackage::new("CASE-001".to_string(), collector.clone());
        let package2 = EvidencePackage::new("CASE-002".to_string(), collector);

        let packages = vec![package1, package2];

        let query = EvidenceSearchQuery::new().with_case_id("CASE-001".to_string());
        let results = query.search(&packages);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].case_id, "CASE-001");
    }

    #[test]
    fn test_evidence_search_collector() {
        let collector1 = EvidenceCollector {
            id: "COL001".to_string(),
            name: "John Doe".to_string(),
            organization: "Org1".to_string(),
            contact: None,
        };
        let collector2 = EvidenceCollector {
            id: "COL002".to_string(),
            name: "Jane Smith".to_string(),
            organization: "Org2".to_string(),
            contact: None,
        };

        let package1 = EvidencePackage::new("CASE-001".to_string(), collector1);
        let package2 = EvidencePackage::new("CASE-002".to_string(), collector2);

        let packages = vec![package1, package2];

        let query = EvidenceSearchQuery::new().with_collector_id("COL001".to_string());
        let results = query.search(&packages);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].collector.id, "COL001");
    }

    #[test]
    fn test_evidence_search_sealed_status() {
        let collector = create_test_collector();
        let mut package1 = EvidencePackage::new("CASE-001".to_string(), collector.clone());
        package1.seal("signature1".to_string()).unwrap();
        let package2 = EvidencePackage::new("CASE-002".to_string(), collector);

        let packages = vec![package1, package2];

        let query = EvidenceSearchQuery::new().with_sealed(true);
        let results = query.search(&packages);

        assert_eq!(results.len(), 1);
        assert!(results[0].sealed);
    }

    #[test]
    fn test_evidence_search_text() {
        let collector = create_test_collector();
        let mut package1 = EvidencePackage::new("CASE-001".to_string(), collector.clone());
        package1
            .add_metadata(
                "subject".to_string(),
                "Financial fraud investigation".to_string(),
            )
            .unwrap();
        let package2 = EvidencePackage::new("CASE-002".to_string(), collector);

        let packages = vec![package1, package2];

        let query = EvidenceSearchQuery::new().with_text_search("fraud".to_string());
        let results = query.search(&packages);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].case_id, "CASE-001");
    }

    // Export Workflow tests
    #[test]
    fn test_export_workflow_creation() {
        let workflow = EvidenceExportWorkflow::new(ExportFormat::JSON);

        assert!(matches!(workflow.format, ExportFormat::JSON));
        assert!(workflow.include_custody);
        assert!(!workflow.include_images);
        assert!(!workflow.encrypt);
        assert!(!workflow.compress);
    }

    #[test]
    fn test_export_workflow_builder() {
        let workflow = EvidenceExportWorkflow::new(ExportFormat::LegalText)
            .with_custody(false)
            .with_images(true)
            .with_encryption(true)
            .with_compression(true);

        assert!(!workflow.include_custody);
        assert!(workflow.include_images);
        assert!(workflow.encrypt);
        assert!(workflow.compress);
    }

    #[test]
    fn test_export_workflow_json_export() {
        let collector = create_test_collector();
        let package = EvidencePackage::new("CASE-001".to_string(), collector);

        let workflow = EvidenceExportWorkflow::new(ExportFormat::JSON);
        let result = workflow.export_package(&package);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("CASE-001"));
    }

    #[test]
    fn test_export_workflow_legal_text_export() {
        let collector = create_test_collector();
        let package = EvidencePackage::new("CASE-001".to_string(), collector);

        let workflow = EvidenceExportWorkflow::new(ExportFormat::LegalText);
        let result = workflow.export_package(&package);

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("DIGITAL EVIDENCE PACKAGE"));
    }
}
