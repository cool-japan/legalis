//! Chain-of-custody tracking for forensic audit trails.
//!
//! This module provides chain-of-custody capabilities for tracking evidence handling:
//! - Evidence custody transfers
//! - Handler verification
//! - Integrity seals
//! - Forensic timeline reconstruction

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Chain-of-custody record for audit evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyRecord {
    /// Unique identifier for this custody record
    pub id: Uuid,
    /// The audit record being tracked
    pub audit_record_id: Uuid,
    /// Timestamp of custody event
    pub timestamp: DateTime<Utc>,
    /// Type of custody event
    pub event: CustodyEvent,
    /// Handler information
    pub handler: CustodyHandler,
    /// Location where event occurred
    pub location: String,
    /// Notes or comments
    pub notes: Option<String>,
    /// Digital signature for verification
    pub signature: Option<String>,
    /// Hash of previous custody record
    pub previous_hash: Option<String>,
    /// Hash of this custody record
    pub record_hash: String,
}

/// Type of custody event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyEvent {
    /// Evidence collected
    Collected,
    /// Evidence transferred to another handler
    Transferred,
    /// Evidence examined
    Examined,
    /// Evidence sealed
    Sealed,
    /// Evidence unsealed
    Unsealed,
    /// Evidence stored
    Stored,
    /// Evidence retrieved
    Retrieved,
    /// Evidence returned
    Returned,
    /// Evidence disposed
    Disposed,
}

/// Handler who takes custody of evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyHandler {
    /// Handler ID (badge number, employee ID, etc.)
    pub id: String,
    /// Handler name
    pub name: String,
    /// Handler role (investigator, analyst, custodian, etc.)
    pub role: String,
    /// Organization
    pub organization: String,
}

impl CustodyRecord {
    /// Creates a new custody record.
    pub fn new(
        audit_record_id: Uuid,
        event: CustodyEvent,
        handler: CustodyHandler,
        location: String,
        notes: Option<String>,
        previous_hash: Option<String>,
    ) -> Self {
        let mut record = Self {
            id: Uuid::new_v4(),
            audit_record_id,
            timestamp: Utc::now(),
            event,
            handler,
            location,
            notes,
            signature: None,
            previous_hash,
            record_hash: String::new(),
        };
        record.record_hash = record.compute_hash();
        record
    }

    /// Computes the hash for this custody record.
    fn compute_hash(&self) -> String {
        let event_str = format!("{:?}", self.event);
        let data = format!(
            "{}{}{}{}{}{}",
            self.id,
            self.audit_record_id,
            self.timestamp.timestamp(),
            event_str,
            self.handler.id,
            self.previous_hash.as_deref().unwrap_or("")
        );
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

    /// Verifies the integrity of this custody record.
    pub fn verify(&self) -> bool {
        let computed = self.compute_hash();
        computed == self.record_hash
    }

    /// Signs this custody record.
    pub fn sign(&mut self, signature: String) {
        self.signature = Some(signature);
    }
}

/// Chain of custody for an audit record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyChain {
    /// The audit record being tracked
    pub audit_record_id: Uuid,
    /// Custody records in chronological order
    pub records: Vec<CustodyRecord>,
    /// Current handler (if in custody)
    pub current_handler: Option<CustodyHandler>,
    /// Current location
    pub current_location: Option<String>,
    /// Status of the evidence
    pub status: CustodyStatus,
}

/// Status of evidence in custody.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyStatus {
    /// Evidence is in custody
    InCustody,
    /// Evidence is sealed
    Sealed,
    /// Evidence has been returned
    Returned,
    /// Evidence has been disposed
    Disposed,
}

impl CustodyChain {
    /// Creates a new custody chain.
    pub fn new(audit_record_id: Uuid) -> Self {
        Self {
            audit_record_id,
            records: Vec::new(),
            current_handler: None,
            current_location: None,
            status: CustodyStatus::InCustody,
        }
    }

    /// Adds a custody record to the chain.
    pub fn add_record(&mut self, mut record: CustodyRecord) -> AuditResult<()> {
        // Verify record is for this chain
        if record.audit_record_id != self.audit_record_id {
            return Err(crate::AuditError::InvalidRecord(
                "Custody record is for a different audit record".to_string(),
            ));
        }

        // Set previous hash
        if let Some(last_record) = self.records.last() {
            record.previous_hash = Some(last_record.record_hash.clone());
            record.record_hash = record.compute_hash();
        }

        // Update current handler and location
        self.current_handler = Some(record.handler.clone());
        self.current_location = Some(record.location.clone());

        // Update status based on event
        match record.event {
            CustodyEvent::Sealed => self.status = CustodyStatus::Sealed,
            CustodyEvent::Returned => self.status = CustodyStatus::Returned,
            CustodyEvent::Disposed => self.status = CustodyStatus::Disposed,
            CustodyEvent::Unsealed => self.status = CustodyStatus::InCustody,
            _ => {}
        }

        self.records.push(record);
        Ok(())
    }

    /// Verifies the integrity of the entire custody chain.
    pub fn verify_integrity(&self) -> AuditResult<bool> {
        let mut expected_prev_hash: Option<String> = None;

        for record in &self.records {
            // Verify record hash
            if !record.verify() {
                return Err(crate::AuditError::TamperDetected(format!(
                    "Custody record {} has invalid hash",
                    record.id
                )));
            }

            // Verify chain
            if record.previous_hash != expected_prev_hash {
                return Err(crate::AuditError::TamperDetected(format!(
                    "Custody record {} has broken chain link",
                    record.id
                )));
            }

            expected_prev_hash = Some(record.record_hash.clone());
        }

        Ok(true)
    }

    /// Gets the custody timeline.
    pub fn timeline(&self) -> Vec<CustodyTimelineEntry> {
        self.records
            .iter()
            .map(|r| CustodyTimelineEntry {
                timestamp: r.timestamp,
                event: format!("{:?}", r.event),
                handler: r.handler.name.clone(),
                role: r.handler.role.clone(),
                location: r.location.clone(),
                notes: r.notes.clone(),
            })
            .collect()
    }

    /// Gets all handlers who have had custody.
    pub fn handlers(&self) -> Vec<&CustodyHandler> {
        let mut handlers = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for record in &self.records {
            if seen.insert(record.handler.id.clone()) {
                handlers.push(&record.handler);
            }
        }

        handlers
    }

    /// Gets the duration of custody (from first to last record).
    pub fn custody_duration(&self) -> Option<chrono::Duration> {
        if self.records.len() < 2 {
            return None;
        }

        let first = &self.records[0];
        let last = &self.records[self.records.len() - 1];

        Some(last.timestamp.signed_duration_since(first.timestamp))
    }
}

/// Entry in custody timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyTimelineEntry {
    pub timestamp: DateTime<Utc>,
    pub event: String,
    pub handler: String,
    pub role: String,
    pub location: String,
    pub notes: Option<String>,
}

/// Chain-of-custody manager.
pub struct CustodyManager {
    chains: HashMap<Uuid, CustodyChain>,
}

impl CustodyManager {
    /// Creates a new custody manager.
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    /// Initializes a custody chain for an audit record.
    pub fn init_chain(&mut self, audit_record_id: Uuid) {
        self.chains
            .insert(audit_record_id, CustodyChain::new(audit_record_id));
    }

    /// Adds a custody record.
    pub fn add_custody_record(
        &mut self,
        audit_record_id: Uuid,
        event: CustodyEvent,
        handler: CustodyHandler,
        location: String,
        notes: Option<String>,
    ) -> AuditResult<Uuid> {
        let chain = self
            .chains
            .entry(audit_record_id)
            .or_insert_with(|| CustodyChain::new(audit_record_id));

        let previous_hash = chain.records.last().map(|r| r.record_hash.clone());
        let record = CustodyRecord::new(
            audit_record_id,
            event,
            handler,
            location,
            notes,
            previous_hash,
        );

        let record_id = record.id;
        chain.add_record(record)?;

        Ok(record_id)
    }

    /// Gets a custody chain.
    pub fn get_chain(&self, audit_record_id: Uuid) -> Option<&CustodyChain> {
        self.chains.get(&audit_record_id)
    }

    /// Verifies all custody chains.
    pub fn verify_all(&self) -> AuditResult<bool> {
        for chain in self.chains.values() {
            chain.verify_integrity()?;
        }
        Ok(true)
    }

    /// Exports custody chains for audit records.
    pub fn export_chains(&self, audit_records: &[AuditRecord]) -> Vec<CustodyChain> {
        audit_records
            .iter()
            .filter_map(|r| self.chains.get(&r.id).cloned())
            .collect()
    }

    /// Generates a custody report.
    pub fn generate_report(&self) -> CustodyReport {
        CustodyReport {
            total_chains: self.chains.len(),
            total_custody_records: self.chains.values().map(|c| c.records.len()).sum(),
            chains_in_custody: self
                .chains
                .values()
                .filter(|c| c.status == CustodyStatus::InCustody)
                .count(),
            chains_sealed: self
                .chains
                .values()
                .filter(|c| c.status == CustodyStatus::Sealed)
                .count(),
            chains_returned: self
                .chains
                .values()
                .filter(|c| c.status == CustodyStatus::Returned)
                .count(),
            chains_disposed: self
                .chains
                .values()
                .filter(|c| c.status == CustodyStatus::Disposed)
                .count(),
        }
    }
}

impl Default for CustodyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Custody report summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyReport {
    pub total_chains: usize,
    pub total_custody_records: usize,
    pub chains_in_custody: usize,
    pub chains_sealed: usize,
    pub chains_returned: usize,
    pub chains_disposed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler(id: &str) -> CustodyHandler {
        CustodyHandler {
            id: id.to_string(),
            name: format!("Handler {}", id),
            role: "Investigator".to_string(),
            organization: "Test Org".to_string(),
        }
    }

    #[test]
    fn test_custody_record_creation() {
        let audit_id = Uuid::new_v4();
        let handler = create_test_handler("H001");
        let record = CustodyRecord::new(
            audit_id,
            CustodyEvent::Collected,
            handler,
            "Evidence Room A".to_string(),
            Some("Initial collection".to_string()),
            None,
        );

        assert_eq!(record.audit_record_id, audit_id);
        assert!(record.verify());
    }

    #[test]
    fn test_custody_chain() {
        let audit_id = Uuid::new_v4();
        let mut chain = CustodyChain::new(audit_id);

        let record1 = CustodyRecord::new(
            audit_id,
            CustodyEvent::Collected,
            create_test_handler("H001"),
            "Evidence Room A".to_string(),
            None,
            None,
        );
        chain.add_record(record1).unwrap();

        let record2 = CustodyRecord::new(
            audit_id,
            CustodyEvent::Transferred,
            create_test_handler("H002"),
            "Lab B".to_string(),
            None,
            None,
        );
        chain.add_record(record2).unwrap();

        assert_eq!(chain.records.len(), 2);
        assert!(chain.verify_integrity().unwrap());
    }

    #[test]
    fn test_custody_manager() {
        let mut manager = CustodyManager::new();
        let audit_id = Uuid::new_v4();

        manager.init_chain(audit_id);

        let record_id = manager
            .add_custody_record(
                audit_id,
                CustodyEvent::Collected,
                create_test_handler("H001"),
                "Evidence Room".to_string(),
                None,
            )
            .unwrap();

        assert!(record_id != Uuid::nil());

        let chain = manager.get_chain(audit_id).unwrap();
        assert_eq!(chain.records.len(), 1);
    }

    #[test]
    fn test_custody_timeline() {
        let audit_id = Uuid::new_v4();
        let mut chain = CustodyChain::new(audit_id);

        chain
            .add_record(CustodyRecord::new(
                audit_id,
                CustodyEvent::Collected,
                create_test_handler("H001"),
                "Room A".to_string(),
                None,
                None,
            ))
            .unwrap();

        chain
            .add_record(CustodyRecord::new(
                audit_id,
                CustodyEvent::Transferred,
                create_test_handler("H002"),
                "Lab B".to_string(),
                None,
                None,
            ))
            .unwrap();

        let timeline = chain.timeline();
        assert_eq!(timeline.len(), 2);
        assert_eq!(timeline[0].event, "Collected");
        assert_eq!(timeline[1].event, "Transferred");
    }

    #[test]
    fn test_custody_handlers() {
        let audit_id = Uuid::new_v4();
        let mut chain = CustodyChain::new(audit_id);

        chain
            .add_record(CustodyRecord::new(
                audit_id,
                CustodyEvent::Collected,
                create_test_handler("H001"),
                "Room A".to_string(),
                None,
                None,
            ))
            .unwrap();

        chain
            .add_record(CustodyRecord::new(
                audit_id,
                CustodyEvent::Transferred,
                create_test_handler("H002"),
                "Lab B".to_string(),
                None,
                None,
            ))
            .unwrap();

        let handlers = chain.handlers();
        assert_eq!(handlers.len(), 2);
    }

    #[test]
    fn test_custody_status() {
        let audit_id = Uuid::new_v4();
        let mut chain = CustodyChain::new(audit_id);

        assert_eq!(chain.status, CustodyStatus::InCustody);

        chain
            .add_record(CustodyRecord::new(
                audit_id,
                CustodyEvent::Sealed,
                create_test_handler("H001"),
                "Room A".to_string(),
                None,
                None,
            ))
            .unwrap();

        assert_eq!(chain.status, CustodyStatus::Sealed);
    }

    #[test]
    fn test_custody_report() {
        let mut manager = CustodyManager::new();

        let audit_id1 = Uuid::new_v4();
        manager
            .add_custody_record(
                audit_id1,
                CustodyEvent::Collected,
                create_test_handler("H001"),
                "Room A".to_string(),
                None,
            )
            .unwrap();

        let audit_id2 = Uuid::new_v4();
        manager
            .add_custody_record(
                audit_id2,
                CustodyEvent::Sealed,
                create_test_handler("H002"),
                "Room B".to_string(),
                None,
            )
            .unwrap();

        let report = manager.generate_report();
        assert_eq!(report.total_chains, 2);
        assert_eq!(report.total_custody_records, 2);
        assert_eq!(report.chains_in_custody, 1);
        assert_eq!(report.chains_sealed, 1);
    }
}
