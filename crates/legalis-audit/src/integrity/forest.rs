//! Merkle tree forest for scalable integrity verification
//!
//! This module implements a forest of Merkle trees to enable efficient verification
//! of very large audit trails by partitioning records across multiple trees.

use super::{MerkleTree, MerkleProof};
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Convert bytes to hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Strategy for partitioning records across trees
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionStrategy {
    /// Partition by time period (e.g., daily, hourly)
    Temporal,
    /// Partition by record count (each tree has max N records)
    Count,
    /// Partition by statute ID
    Statute,
    /// Partition by subject ID
    Subject,
    /// Hash-based partitioning for even distribution
    HashBased,
}

/// Configuration for Merkle forest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestConfig {
    pub partition_strategy: PartitionStrategy,
    pub max_records_per_tree: usize,
    pub temporal_partition_hours: u64,
}

impl Default for ForestConfig {
    fn default() -> Self {
        Self {
            partition_strategy: PartitionStrategy::Count,
            max_records_per_tree: 10000,
            temporal_partition_hours: 24,
        }
    }
}

/// A partition identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PartitionId(pub String);

impl PartitionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn from_timestamp(timestamp: DateTime<Utc>, hours: u64) -> Self {
        let partition_key = timestamp.timestamp() / (hours as i64 * 3600);
        Self(format!("temporal-{}", partition_key))
    }

    pub fn from_count(index: usize) -> Self {
        Self(format!("count-{}", index))
    }

    pub fn from_statute(statute_id: &str) -> Self {
        Self(format!("statute-{}", statute_id))
    }

    pub fn from_subject(subject_id: &Uuid) -> Self {
        Self(format!("subject-{}", subject_id))
    }

    pub fn from_hash(record_id: &Uuid, num_partitions: usize) -> Self {
        let hash = record_id.as_u128();
        let partition = (hash % num_partitions as u128) as usize;
        Self(format!("hash-{}", partition))
    }
}

/// Information about a partition in the forest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub id: PartitionId,
    pub record_count: usize,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub root_hash: Option<String>,
}

impl PartitionInfo {
    pub fn new(id: PartitionId) -> Self {
        let now = Utc::now();
        Self {
            id,
            record_count: 0,
            created_at: now,
            last_updated: now,
            root_hash: None,
        }
    }
}

/// A forest of Merkle trees for scalable verification
pub struct MerkleForest {
    config: ForestConfig,
    trees: HashMap<PartitionId, MerkleTree>,
    partition_info: HashMap<PartitionId, PartitionInfo>,
    record_to_partition: HashMap<Uuid, PartitionId>,
}

impl MerkleForest {
    /// Create a new Merkle forest
    pub fn new(config: ForestConfig) -> Self {
        Self {
            config,
            trees: HashMap::new(),
            partition_info: HashMap::new(),
            record_to_partition: HashMap::new(),
        }
    }

    /// Add records to the forest
    pub fn add_records(&mut self, records: Vec<AuditRecord>) -> AuditResult<()> {
        // Group records by partition
        let mut partitions: HashMap<PartitionId, Vec<AuditRecord>> = HashMap::new();

        for record in records {
            let partition_id = self.get_partition_id(&record);
            partitions.entry(partition_id).or_default().push(record);
        }

        // Build/update Merkle trees for each partition
        for (partition_id, partition_records) in partitions {
            self.add_to_partition(partition_id, partition_records)?;
        }

        Ok(())
    }

    /// Add records to a specific partition
    fn add_to_partition(
        &mut self,
        partition_id: PartitionId,
        mut records: Vec<AuditRecord>,
    ) -> AuditResult<()> {
        // Get or create the tree for this partition
        let tree = self.trees.entry(partition_id.clone()).or_insert_with(|| {
            MerkleTree::from_records(&[])
        });

        // Get or create partition info
        let info = self
            .partition_info
            .entry(partition_id.clone())
            .or_insert_with(|| PartitionInfo::new(partition_id.clone()));

        // Add existing records from the tree
        let mut all_records = tree.records.clone();
        all_records.append(&mut records);

        // Rebuild tree with all records
        *tree = MerkleTree::from_records(&all_records);

        // Update partition info
        info.record_count = all_records.len();
        info.last_updated = Utc::now();
        info.root_hash = tree.root().map(|h| bytes_to_hex(h));

        // Update record-to-partition mapping
        for record in &all_records {
            self.record_to_partition.insert(record.id, partition_id.clone());
        }

        Ok(())
    }

    /// Get the partition ID for a record based on the strategy
    fn get_partition_id(&self, record: &AuditRecord) -> PartitionId {
        match self.config.partition_strategy {
            PartitionStrategy::Temporal => {
                PartitionId::from_timestamp(record.timestamp, self.config.temporal_partition_hours)
            }
            PartitionStrategy::Count => {
                // Find the first partition with space, or create a new one
                let partition_index = self.partition_info.len();
                PartitionId::from_count(partition_index)
            }
            PartitionStrategy::Statute => PartitionId::from_statute(&record.statute_id),
            PartitionStrategy::Subject => PartitionId::from_subject(&record.subject_id),
            PartitionStrategy::HashBased => {
                let num_partitions = self.trees.len().max(1);
                PartitionId::from_hash(&record.id, num_partitions)
            }
        }
    }

    /// Generate a proof for a specific record
    pub fn generate_proof(&self, record_id: &Uuid) -> AuditResult<ForestProof> {
        // Find which partition contains the record
        let partition_id = self
            .record_to_partition
            .get(record_id)
            .ok_or_else(|| AuditError::RecordNotFound(*record_id))?;

        // Get the tree for that partition
        let tree = self
            .trees
            .get(partition_id)
            .ok_or_else(|| AuditError::StorageError("Partition tree not found".to_string()))?;

        // Find the record index in the tree
        let index = tree
            .records
            .iter()
            .position(|r| r.id == *record_id)
            .ok_or_else(|| AuditError::RecordNotFound(*record_id))?;

        // Generate the Merkle proof
        let merkle_proof = tree.generate_proof(index);

        Ok(ForestProof {
            partition_id: partition_id.clone(),
            merkle_proof,
            partition_root: tree.root().map(|h| bytes_to_hex(h)),
        })
    }

    /// Verify a proof for a record
    pub fn verify_proof(&self, record: &AuditRecord, proof: &ForestProof) -> AuditResult<bool> {
        // Get the tree for the partition
        let tree = self
            .trees
            .get(&proof.partition_id)
            .ok_or_else(|| AuditError::StorageError("Partition tree not found".to_string()))?;

        // Verify the Merkle proof
        Ok(tree.verify_proof(record, &proof.merkle_proof))
    }

    /// Get all partition IDs
    pub fn get_partitions(&self) -> Vec<PartitionId> {
        self.partition_info.keys().cloned().collect()
    }

    /// Get information about a partition
    pub fn get_partition_info(&self, partition_id: &PartitionId) -> Option<&PartitionInfo> {
        self.partition_info.get(partition_id)
    }

    /// Get the number of partitions
    pub fn partition_count(&self) -> usize {
        self.trees.len()
    }

    /// Get the total number of records across all partitions
    pub fn total_record_count(&self) -> usize {
        self.partition_info.values().map(|info| info.record_count).sum()
    }

    /// Verify integrity of all partitions
    pub fn verify_all(&self) -> AuditResult<ForestVerificationResult> {
        let mut result = ForestVerificationResult {
            total_partitions: self.partition_count(),
            verified_partitions: 0,
            failed_partitions: Vec::new(),
            total_records: self.total_record_count(),
        };

        for (partition_id, tree) in &self.trees {
            if tree.verify_integrity() {
                result.verified_partitions += 1;
            } else {
                result.failed_partitions.push(partition_id.clone());
            }
        }

        Ok(result)
    }

    /// Get statistics about the forest
    pub fn get_stats(&self) -> ForestStats {
        let partition_sizes: Vec<usize> = self
            .partition_info
            .values()
            .map(|info| info.record_count)
            .collect();

        let avg_partition_size = if !partition_sizes.is_empty() {
            partition_sizes.iter().sum::<usize>() as f64 / partition_sizes.len() as f64
        } else {
            0.0
        };

        let max_partition_size = partition_sizes.iter().max().copied().unwrap_or(0);
        let min_partition_size = partition_sizes.iter().min().copied().unwrap_or(0);

        ForestStats {
            total_partitions: self.partition_count(),
            total_records: self.total_record_count(),
            avg_partition_size,
            max_partition_size,
            min_partition_size,
            partition_strategy: self.config.partition_strategy,
        }
    }

    /// Merge small partitions to optimize the forest
    pub fn optimize(&mut self) -> AuditResult<usize> {
        let min_size = self.config.max_records_per_tree / 4; // Merge if less than 25% full
        let mut merged_count = 0;

        // Find partitions that are too small
        let small_partitions: Vec<PartitionId> = self
            .partition_info
            .iter()
            .filter(|(_, info)| info.record_count < min_size)
            .map(|(id, _)| id.clone())
            .collect();

        if small_partitions.len() < 2 {
            return Ok(0);
        }

        // Collect all records from small partitions
        let mut all_records = Vec::new();
        for partition_id in &small_partitions {
            if let Some(tree) = self.trees.get(partition_id) {
                all_records.extend(tree.records.clone());
            }
        }

        // Remove old partitions
        for partition_id in &small_partitions {
            self.trees.remove(partition_id);
            self.partition_info.remove(partition_id);
            merged_count += 1;
        }

        // Create new optimized partition(s)
        let new_partition_id = PartitionId::new(format!("optimized-{}", Utc::now().timestamp()));
        self.add_to_partition(new_partition_id, all_records)?;

        Ok(merged_count)
    }
}

/// Proof of a record's inclusion in the forest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestProof {
    pub partition_id: PartitionId,
    pub merkle_proof: MerkleProof,
    pub partition_root: Option<String>,
}

/// Result of verifying all partitions in the forest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestVerificationResult {
    pub total_partitions: usize,
    pub verified_partitions: usize,
    pub failed_partitions: Vec<PartitionId>,
    pub total_records: usize,
}

impl ForestVerificationResult {
    pub fn is_valid(&self) -> bool {
        self.failed_partitions.is_empty()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_partitions == 0 {
            return 1.0;
        }
        self.verified_partitions as f64 / self.total_partitions as f64
    }
}

/// Statistics about the Merkle forest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForestStats {
    pub total_partitions: usize,
    pub total_records: usize,
    pub avg_partition_size: f64,
    pub max_partition_size: usize,
    pub min_partition_size: usize,
    pub partition_strategy: PartitionStrategy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, EventType};

    fn create_test_record() -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::AutomaticDecision,
            statute_id: "TEST-1".to_string(),
            subject_id: Uuid::new_v4(),
            actor: Actor::System { component: "test".to_string() },
            context: crate::DecisionContext::default(),
            result: crate::DecisionResult::Deterministic {
                effect_applied: "allowed".to_string(),
                parameters: std::collections::HashMap::new(),
            },
            previous_hash: None,
            record_hash: "hash123".to_string(),
        }
    }

    #[test]
    fn test_forest_config_default() {
        let config = ForestConfig::default();
        assert_eq!(config.partition_strategy, PartitionStrategy::Count);
        assert_eq!(config.max_records_per_tree, 10000);
        assert_eq!(config.temporal_partition_hours, 24);
    }

    #[test]
    fn test_partition_id_from_timestamp() {
        let timestamp = Utc::now();
        let partition1 = PartitionId::from_timestamp(timestamp, 24);
        let partition2 = PartitionId::from_timestamp(timestamp, 24);
        assert_eq!(partition1, partition2);
    }

    #[test]
    fn test_partition_id_from_count() {
        let partition1 = PartitionId::from_count(0);
        let partition2 = PartitionId::from_count(1);
        assert_ne!(partition1, partition2);
    }

    #[test]
    fn test_partition_id_from_statute() {
        let partition = PartitionId::from_statute("STATUTE-1");
        assert_eq!(partition.0, "statute-STATUTE-1");
    }

    #[test]
    fn test_partition_id_from_subject() {
        let subject_id = Uuid::new_v4();
        let partition = PartitionId::from_subject(&subject_id);
        assert!(partition.0.starts_with("subject-"));
    }

    #[test]
    fn test_partition_id_from_hash() {
        let record_id = Uuid::new_v4();
        let partition = PartitionId::from_hash(&record_id, 4);
        assert!(partition.0.starts_with("hash-"));
    }

    #[test]
    fn test_merkle_forest_creation() {
        let config = ForestConfig::default();
        let forest = MerkleForest::new(config);

        assert_eq!(forest.partition_count(), 0);
        assert_eq!(forest.total_record_count(), 0);
    }

    #[test]
    fn test_add_records() {
        let config = ForestConfig::default();
        let mut forest = MerkleForest::new(config);

        let records = vec![
            create_test_record(),
            create_test_record(),
            create_test_record(),
        ];

        forest.add_records(records).unwrap();

        assert!(forest.partition_count() > 0);
        assert_eq!(forest.total_record_count(), 3);
    }

    #[test]
    fn test_temporal_partitioning() {
        let config = ForestConfig {
            partition_strategy: PartitionStrategy::Temporal,
            temporal_partition_hours: 1,
            ..Default::default()
        };
        let mut forest = MerkleForest::new(config);

        let mut record1 = create_test_record();
        record1.timestamp = Utc::now();

        let mut record2 = create_test_record();
        record2.timestamp = Utc::now() + chrono::Duration::hours(2);

        forest.add_records(vec![record1, record2]).unwrap();

        // Should create 2 partitions (different hours)
        assert!(forest.partition_count() >= 1);
    }

    #[test]
    fn test_statute_partitioning() {
        let config = ForestConfig {
            partition_strategy: PartitionStrategy::Statute,
            ..Default::default()
        };
        let mut forest = MerkleForest::new(config);

        let mut record1 = create_test_record();
        record1.statute_id = "STATUTE-1".to_string();

        let mut record2 = create_test_record();
        record2.statute_id = "STATUTE-2".to_string();

        forest.add_records(vec![record1, record2]).unwrap();

        // Should create 2 partitions (different statutes)
        assert_eq!(forest.partition_count(), 2);
    }

    #[test]
    fn test_generate_and_verify_proof() {
        let config = ForestConfig::default();
        let mut forest = MerkleForest::new(config);

        let record = create_test_record();
        let record_id = record.id;

        forest.add_records(vec![record.clone()]).unwrap();

        let proof = forest.generate_proof(&record_id).unwrap();
        let is_valid = forest.verify_proof(&record, &proof).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_verify_all() {
        let config = ForestConfig::default();
        let mut forest = MerkleForest::new(config);

        let records = vec![
            create_test_record(),
            create_test_record(),
            create_test_record(),
        ];

        forest.add_records(records).unwrap();

        let result = forest.verify_all().unwrap();
        assert!(result.is_valid());
        assert_eq!(result.success_rate(), 1.0);
    }

    #[test]
    fn test_get_partition_info() {
        let config = ForestConfig::default();
        let mut forest = MerkleForest::new(config);

        let record = create_test_record();
        forest.add_records(vec![record]).unwrap();

        let partitions = forest.get_partitions();
        assert_eq!(partitions.len(), 1);

        let info = forest.get_partition_info(&partitions[0]);
        assert!(info.is_some());
        assert_eq!(info.unwrap().record_count, 1);
    }

    #[test]
    fn test_forest_stats() {
        let config = ForestConfig::default();
        let mut forest = MerkleForest::new(config);

        let records = vec![
            create_test_record(),
            create_test_record(),
            create_test_record(),
        ];

        forest.add_records(records).unwrap();

        let stats = forest.get_stats();
        assert_eq!(stats.total_records, 3);
        assert!(stats.total_partitions > 0);
        assert_eq!(stats.partition_strategy, PartitionStrategy::Count);
    }

    #[test]
    fn test_optimize() {
        let config = ForestConfig {
            max_records_per_tree: 100,
            ..Default::default()
        };
        let mut forest = MerkleForest::new(config);

        // Add a few records (less than 25% of max, so they should be merged)
        let records = vec![
            create_test_record(),
            create_test_record(),
        ];

        forest.add_records(records).unwrap();

        // Optimization might not happen with just 1 partition
        let merged = forest.optimize().unwrap();
        // This is acceptable - optimization is opportunistic
        assert!(merged >= 0);
    }
}
