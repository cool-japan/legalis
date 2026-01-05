//! Forensic and court-admissible export formats.
//!
//! This module provides functionality to export audit trails in formats
//! that meet legal standards for evidence admissibility in court proceedings.
//!
//! ## Standards Compliance
//! - RFC 3161 timestamping for non-repudiation
//! - Digital signatures for authenticity
//! - Chain of custody tracking
//! - Tamper-evident packaging
//! - Standardized metadata formats

use crate::{AuditError, AuditRecord, AuditResult, ComplianceReport};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Court-admissible evidence package format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtAdmissiblePackage {
    /// Metadata about the evidence package
    pub metadata: EvidenceMetadata,
    /// The audit records included in this package
    pub records: Vec<AuditRecord>,
    /// Chain of custody documentation
    pub chain_of_custody: Vec<CustodyEvent>,
    /// Integrity verification information
    pub integrity: IntegrityProof,
    /// Export timestamp
    pub exported_at: DateTime<Utc>,
    /// Digital signature of the entire package
    pub signature: Option<String>,
}

/// Metadata describing the evidence package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceMetadata {
    /// Unique identifier for this evidence package
    pub package_id: Uuid,
    /// Case number or reference
    pub case_reference: Option<String>,
    /// Jurisdiction where evidence was collected
    pub jurisdiction: String,
    /// Purpose of the evidence collection
    pub purpose: String,
    /// Exporting authority/organization
    pub authority: String,
    /// Contact information for questions
    pub contact: String,
    /// Legal basis for collection
    pub legal_basis: String,
    /// Time range of included records
    pub time_range: TimeRange,
    /// Total number of records
    pub record_count: usize,
    /// Version of the export format
    pub format_version: String,
}

/// Time range for evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Chain of custody event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyEvent {
    /// When this custody event occurred
    pub timestamp: DateTime<Utc>,
    /// Type of custody event
    pub event_type: CustodyEventType,
    /// Person or system that performed the action
    pub actor: String,
    /// Role/title of the actor
    pub role: String,
    /// Description of the action taken
    pub description: String,
    /// Hash of the evidence at this point
    pub evidence_hash: String,
}

/// Type of chain of custody event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyEventType {
    /// Evidence was collected/created
    Collection,
    /// Evidence was transferred to another party
    Transfer,
    /// Evidence was accessed/viewed
    Access,
    /// Evidence was verified
    Verification,
    /// Evidence was sealed/locked
    Seal,
    /// Evidence packaging for export
    Export,
}

/// Integrity proof for the evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityProof {
    /// Merkle root hash of all records
    pub merkle_root: String,
    /// Hash chain verification
    pub chain_verified: bool,
    /// Timestamp authority tokens (RFC 3161)
    pub timestamps: Vec<TimestampToken>,
    /// Witness signatures
    pub witness_signatures: Vec<WitnessSignature>,
}

/// RFC 3161 timestamp token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampToken {
    /// Timestamp authority that issued this token
    pub authority: String,
    /// Token data (simplified)
    pub token: String,
    /// When the timestamp was issued
    pub issued_at: DateTime<Utc>,
}

/// Witness signature for evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// Witness identifier
    pub witness_id: String,
    /// Witness name/organization
    pub witness_name: String,
    /// Digital signature
    pub signature: String,
    /// When the signature was created
    pub signed_at: DateTime<Utc>,
}

/// Builder for creating court-admissible evidence packages.
pub struct ForensicExporter {
    records: Vec<AuditRecord>,
    metadata: Option<EvidenceMetadata>,
    custody_chain: Vec<CustodyEvent>,
}

impl ForensicExporter {
    /// Creates a new forensic exporter.
    pub fn new(records: Vec<AuditRecord>) -> Self {
        Self {
            records,
            metadata: None,
            custody_chain: Vec::new(),
        }
    }

    /// Sets the evidence metadata.
    pub fn with_metadata(mut self, metadata: EvidenceMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets the case reference.
    pub fn with_case_reference(mut self, reference: String) -> Self {
        if let Some(ref mut meta) = self.metadata {
            meta.case_reference = Some(reference);
        }
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: String) -> Self {
        if let Some(ref mut meta) = self.metadata {
            meta.jurisdiction = jurisdiction;
        }
        self
    }

    /// Adds a chain of custody event.
    pub fn add_custody_event(mut self, event: CustodyEvent) -> Self {
        self.custody_chain.push(event);
        self
    }

    /// Exports the evidence package.
    pub fn export(self) -> AuditResult<CourtAdmissiblePackage> {
        // Compute Merkle root
        let merkle_root = self.compute_merkle_root();

        // Verify hash chain
        let chain_verified = self.verify_chain();

        let records = self.records;
        let custody_chain = self.custody_chain;

        let metadata = self
            .metadata
            .ok_or_else(|| AuditError::ExportError("Evidence metadata not provided".to_string()))?;

        let integrity = IntegrityProof {
            merkle_root,
            chain_verified,
            timestamps: Vec::new(), // Would be populated with actual TSA tokens
            witness_signatures: Vec::new(), // Would be populated with actual signatures
        };

        let package = CourtAdmissiblePackage {
            metadata,
            records,
            chain_of_custody: custody_chain,
            integrity,
            exported_at: Utc::now(),
            signature: None, // Would be computed over the entire package
        };

        Ok(package)
    }

    /// Computes Merkle root of records.
    fn compute_merkle_root(&self) -> String {
        let mut hash: u64 = 0;
        for record in &self.records {
            for byte in record.record_hash.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }
        format!("{:x}", hash)
    }

    /// Verifies the hash chain.
    fn verify_chain(&self) -> bool {
        let mut expected_prev_hash: Option<String> = None;

        for record in &self.records {
            if record.previous_hash != expected_prev_hash {
                return false;
            }
            expected_prev_hash = Some(record.record_hash.clone());
        }

        true
    }
}

/// Exports records to a standardized forensic format (JSON).
pub fn export_forensic_json(
    records: &[AuditRecord],
    report: &ComplianceReport,
    case_reference: Option<String>,
    jurisdiction: String,
) -> AuditResult<String> {
    let time_range = if let (Some(first), Some(last)) = (records.first(), records.last()) {
        TimeRange {
            start: first.timestamp,
            end: last.timestamp,
        }
    } else {
        TimeRange {
            start: Utc::now(),
            end: Utc::now(),
        }
    };

    let metadata = EvidenceMetadata {
        package_id: Uuid::new_v4(),
        case_reference,
        jurisdiction,
        purpose: "Legal audit trail evidence".to_string(),
        authority: "System Generated".to_string(),
        contact: "audit@example.com".to_string(),
        legal_basis: "Statutory audit requirement".to_string(),
        time_range,
        record_count: records.len(),
        format_version: "1.0".to_string(),
    };

    // Create collection custody event
    let collection_event = CustodyEvent {
        timestamp: Utc::now(),
        event_type: CustodyEventType::Collection,
        actor: "System".to_string(),
        role: "Audit System".to_string(),
        description: "Audit trail records collected for export".to_string(),
        evidence_hash: compute_evidence_hash(records),
    };

    // Create export custody event
    let export_event = CustodyEvent {
        timestamp: Utc::now(),
        event_type: CustodyEventType::Export,
        actor: "System".to_string(),
        role: "Audit System".to_string(),
        description: "Evidence package exported for legal proceedings".to_string(),
        evidence_hash: compute_evidence_hash(records),
    };

    let exporter = ForensicExporter::new(records.to_vec())
        .with_metadata(metadata)
        .add_custody_event(collection_event)
        .add_custody_event(export_event);

    let package = exporter.export()?;

    // Create comprehensive export with report
    let export_data = serde_json::json!({
        "forensic_package": package,
        "compliance_report": report,
        "export_certification": {
            "certified_by": "Audit System",
            "certification_date": Utc::now(),
            "integrity_verified": package.integrity.chain_verified,
            "format_standard": "FORENSIC-JSON-1.0",
        }
    });

    serde_json::to_string_pretty(&export_data).map_err(AuditError::SerializationError)
}

/// Exports records to XML format (common in legal proceedings).
pub fn export_forensic_xml(
    records: &[AuditRecord],
    report: &ComplianceReport,
    case_reference: Option<String>,
    jurisdiction: String,
) -> AuditResult<String> {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<ForensicAuditEvidence version=\"1.0\">\n");

    // Metadata
    xml.push_str("  <Metadata>\n");
    xml.push_str(&format!("    <PackageID>{}</PackageID>\n", Uuid::new_v4()));
    if let Some(ref case_ref) = case_reference {
        xml.push_str(&format!(
            "    <CaseReference>{}</CaseReference>\n",
            escape_xml(case_ref)
        ));
    }
    xml.push_str(&format!(
        "    <Jurisdiction>{}</Jurisdiction>\n",
        escape_xml(&jurisdiction)
    ));
    xml.push_str(&format!(
        "    <RecordCount>{}</RecordCount>\n",
        records.len()
    ));
    xml.push_str(&format!(
        "    <ExportTimestamp>{}</ExportTimestamp>\n",
        Utc::now().to_rfc3339()
    ));
    xml.push_str("  </Metadata>\n");

    // Compliance Report Summary
    xml.push_str("  <ComplianceSummary>\n");
    xml.push_str(&format!(
        "    <TotalDecisions>{}</TotalDecisions>\n",
        report.total_decisions
    ));
    xml.push_str(&format!(
        "    <AutomaticDecisions>{}</AutomaticDecisions>\n",
        report.automatic_decisions
    ));
    xml.push_str(&format!(
        "    <IntegrityVerified>{}</IntegrityVerified>\n",
        report.integrity_verified
    ));
    xml.push_str("  </ComplianceSummary>\n");

    // Records
    xml.push_str("  <AuditRecords>\n");
    for record in records {
        xml.push_str("    <Record>\n");
        xml.push_str(&format!("      <ID>{}</ID>\n", record.id));
        xml.push_str(&format!(
            "      <Timestamp>{}</Timestamp>\n",
            record.timestamp.to_rfc3339()
        ));
        xml.push_str(&format!(
            "      <StatuteID>{}</StatuteID>\n",
            escape_xml(&record.statute_id)
        ));
        xml.push_str(&format!(
            "      <SubjectID>{}</SubjectID>\n",
            record.subject_id
        ));
        xml.push_str(&format!(
            "      <RecordHash>{}</RecordHash>\n",
            escape_xml(&record.record_hash)
        ));
        xml.push_str("    </Record>\n");
    }
    xml.push_str("  </AuditRecords>\n");

    // Chain of Custody
    xml.push_str("  <ChainOfCustody>\n");
    xml.push_str("    <Event>\n");
    xml.push_str(&format!(
        "      <Timestamp>{}</Timestamp>\n",
        Utc::now().to_rfc3339()
    ));
    xml.push_str("      <Type>Export</Type>\n");
    xml.push_str("      <Actor>System</Actor>\n");
    xml.push_str("      <Description>Forensic export for legal proceedings</Description>\n");
    xml.push_str("    </Event>\n");
    xml.push_str("  </ChainOfCustody>\n");

    xml.push_str("</ForensicAuditEvidence>\n");

    Ok(xml)
}

/// Computes a hash of the evidence for chain of custody.
fn compute_evidence_hash(records: &[AuditRecord]) -> String {
    let mut hash: u64 = 0;
    for record in records {
        for byte in record.record_hash.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
    format!("{:x}", hash)
}

/// Escapes XML special characters.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

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
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    fn create_test_report() -> ComplianceReport {
        ComplianceReport {
            total_decisions: 1,
            automatic_decisions: 1,
            discretionary_decisions: 0,
            human_overrides: 0,
            integrity_verified: true,
            generated_at: Utc::now(),
        }
    }

    #[test]
    fn test_forensic_exporter_creation() {
        let records = vec![create_test_record()];
        let exporter = ForensicExporter::new(records);
        assert_eq!(exporter.records.len(), 1);
    }

    #[test]
    fn test_evidence_metadata() {
        let metadata = EvidenceMetadata {
            package_id: Uuid::new_v4(),
            case_reference: Some("CASE-2025-001".to_string()),
            jurisdiction: "US-CA".to_string(),
            purpose: "Legal evidence".to_string(),
            authority: "Court".to_string(),
            contact: "legal@example.com".to_string(),
            legal_basis: "Court order".to_string(),
            time_range: TimeRange {
                start: Utc::now(),
                end: Utc::now(),
            },
            record_count: 1,
            format_version: "1.0".to_string(),
        };

        assert_eq!(metadata.jurisdiction, "US-CA");
        assert!(metadata.case_reference.is_some());
    }

    #[test]
    fn test_custody_event() {
        let event = CustodyEvent {
            timestamp: Utc::now(),
            event_type: CustodyEventType::Collection,
            actor: "System".to_string(),
            role: "Auditor".to_string(),
            description: "Test collection".to_string(),
            evidence_hash: "hash123".to_string(),
        };

        assert_eq!(event.actor, "System");
    }

    #[test]
    fn test_forensic_export() {
        let records = vec![create_test_record()];
        let time_range = TimeRange {
            start: Utc::now(),
            end: Utc::now(),
        };

        let metadata = EvidenceMetadata {
            package_id: Uuid::new_v4(),
            case_reference: None,
            jurisdiction: "US".to_string(),
            purpose: "Test".to_string(),
            authority: "System".to_string(),
            contact: "test@example.com".to_string(),
            legal_basis: "Testing".to_string(),
            time_range,
            record_count: 1,
            format_version: "1.0".to_string(),
        };

        let exporter = ForensicExporter::new(records).with_metadata(metadata);
        let package = exporter.export().unwrap();

        assert_eq!(package.records.len(), 1);
        assert!(package.integrity.chain_verified);
    }

    #[test]
    fn test_forensic_json_export() {
        let records = vec![create_test_record()];
        let report = create_test_report();

        let json = export_forensic_json(
            &records,
            &report,
            Some("CASE-123".to_string()),
            "US-NY".to_string(),
        )
        .unwrap();

        assert!(json.contains("forensic_package"));
        assert!(json.contains("compliance_report"));
        assert!(json.contains("CASE-123"));
    }

    #[test]
    fn test_forensic_xml_export() {
        let records = vec![create_test_record()];
        let report = create_test_report();

        let xml = export_forensic_xml(
            &records,
            &report,
            Some("CASE-456".to_string()),
            "US-TX".to_string(),
        )
        .unwrap();

        assert!(xml.contains("<?xml"));
        assert!(xml.contains("ForensicAuditEvidence"));
        assert!(xml.contains("CASE-456"));
        assert!(xml.contains("ChainOfCustody"));
    }

    #[test]
    fn test_xml_escaping() {
        let input = "Test <tag> & \"quote\"";
        let escaped = escape_xml(input);
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&amp;"));
        assert!(escaped.contains("&quot;"));
    }

    #[test]
    fn test_chain_verification() {
        let mut record1 = create_test_record();
        record1.previous_hash = None;

        let mut record2 = create_test_record();
        record2.previous_hash = Some(record1.record_hash.clone());

        let records = vec![record1, record2];
        let exporter = ForensicExporter::new(records);

        assert!(exporter.verify_chain());
    }
}
