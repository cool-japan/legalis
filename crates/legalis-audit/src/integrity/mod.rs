//! Integrity verification using Merkle trees.
//!
//! Provides efficient verification of audit trail integrity using Merkle trees,
//! which allow for O(log n) verification of individual records instead of O(n)
//! verification required for simple hash chains.

pub mod blockchain;
// pub mod forest; // TODO: Refactor to match actual MerkleTree API
pub mod multiparty;
pub mod parallel;
pub mod sealed;
pub mod timestamp;
pub mod witness;

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A Merkle tree node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    /// Hash of this node
    pub hash: String,
    /// Left child (if not a leaf)
    pub left: Option<Box<MerkleNode>>,
    /// Right child (if not a leaf)
    pub right: Option<Box<MerkleNode>>,
    /// Record ID (if this is a leaf)
    pub record_id: Option<Uuid>,
}

impl MerkleNode {
    /// Creates a leaf node from a record hash.
    fn leaf(record_id: Uuid, record_hash: String) -> Self {
        Self {
            hash: record_hash,
            left: None,
            right: None,
            record_id: Some(record_id),
        }
    }

    /// Creates an internal node from two children.
    fn internal(left: MerkleNode, right: MerkleNode) -> Self {
        let combined = format!("{}{}", left.hash, right.hash);
        let hash = compute_hash(&combined);
        Self {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            record_id: None,
        }
    }

    /// Checks if this node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.record_id.is_some()
    }
}

/// A Merkle tree for audit trail verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    /// Root node of the tree
    pub root: Option<MerkleNode>,
    /// Number of records in the tree
    pub record_count: usize,
}

impl MerkleTree {
    /// Creates a new Merkle tree from a list of audit records.
    ///
    /// # Example
    /// ```
    /// use legalis_audit::{AuditRecord, EventType, Actor, DecisionContext, DecisionResult};
    /// use legalis_audit::integrity::MerkleTree;
    /// use std::collections::HashMap;
    /// use uuid::Uuid;
    ///
    /// let record = AuditRecord::new(
    ///     EventType::AutomaticDecision,
    ///     Actor::System { component: "test".to_string() },
    ///     "statute-1".to_string(),
    ///     Uuid::new_v4(),
    ///     DecisionContext::default(),
    ///     DecisionResult::Deterministic {
    ///         effect_applied: "test".to_string(),
    ///         parameters: HashMap::new(),
    ///     },
    ///     None,
    /// );
    ///
    /// let tree = MerkleTree::from_records(&[record]);
    /// assert!(tree.verify());
    /// ```
    pub fn from_records(records: &[AuditRecord]) -> Self {
        if records.is_empty() {
            return Self {
                root: None,
                record_count: 0,
            };
        }

        let mut nodes: Vec<MerkleNode> = records
            .iter()
            .map(|r| MerkleNode::leaf(r.id, r.record_hash.clone()))
            .collect();

        // Build tree bottom-up
        while nodes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in nodes.chunks(2) {
                if chunk.len() == 2 {
                    next_level.push(MerkleNode::internal(chunk[0].clone(), chunk[1].clone()));
                } else {
                    // Odd number of nodes - duplicate the last one
                    next_level.push(MerkleNode::internal(chunk[0].clone(), chunk[0].clone()));
                }
            }

            nodes = next_level;
        }

        Self {
            root: Some(nodes.into_iter().next().unwrap()),
            record_count: records.len(),
        }
    }

    /// Gets the root hash of the tree.
    pub fn root_hash(&self) -> Option<String> {
        self.root.as_ref().map(|r| r.hash.clone())
    }

    /// Verifies the integrity of the tree.
    pub fn verify(&self) -> bool {
        if let Some(ref root) = self.root {
            verify_node(root)
        } else {
            true // Empty tree is valid
        }
    }

    /// Generates a Merkle proof for a specific record.
    pub fn generate_proof(&self, record_id: Uuid) -> Option<MerkleProof> {
        if let Some(ref root) = self.root {
            let mut path = Vec::new();
            if find_and_generate_proof(root, record_id, &mut path) {
                Some(MerkleProof {
                    record_id,
                    path,
                    root_hash: root.hash.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Verifies a Merkle proof.
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        if let Some(ref root_hash) = self.root_hash() {
            proof.verify(root_hash)
        } else {
            false
        }
    }
}

/// Verifies a node recursively.
fn verify_node(node: &MerkleNode) -> bool {
    if node.is_leaf() {
        true
    } else if let (Some(left), Some(right)) = (&node.left, &node.right) {
        let combined = format!("{}{}", left.hash, right.hash);
        let expected_hash = compute_hash(&combined);
        if node.hash != expected_hash {
            return false;
        }
        verify_node(left) && verify_node(right)
    } else {
        false
    }
}

/// Finds a record and generates a proof path.
fn find_and_generate_proof(
    node: &MerkleNode,
    record_id: Uuid,
    path: &mut Vec<ProofElement>,
) -> bool {
    if node.is_leaf() {
        node.record_id == Some(record_id)
    } else if let (Some(left), Some(right)) = (&node.left, &node.right) {
        if find_and_generate_proof(left, record_id, path) {
            path.push(ProofElement {
                hash: right.hash.clone(),
                is_left: false,
            });
            true
        } else if find_and_generate_proof(right, record_id, path) {
            path.push(ProofElement {
                hash: left.hash.clone(),
                is_left: true,
            });
            true
        } else {
            false
        }
    } else {
        false
    }
}

/// A Merkle proof for a specific record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    /// The record ID being proven
    pub record_id: Uuid,
    /// Path from leaf to root
    pub path: Vec<ProofElement>,
    /// Expected root hash
    pub root_hash: String,
}

impl MerkleProof {
    /// Verifies this proof against a root hash.
    pub fn verify(&self, root_hash: &str) -> bool {
        self.root_hash == root_hash
    }

    /// Verifies this proof against a record.
    pub fn verify_record(&self, record: &AuditRecord) -> bool {
        if record.id != self.record_id {
            return false;
        }

        let mut current_hash = record.record_hash.clone();

        for element in &self.path {
            let combined = if element.is_left {
                format!("{}{}", element.hash, current_hash)
            } else {
                format!("{}{}", current_hash, element.hash)
            };
            current_hash = compute_hash(&combined);
        }

        current_hash == self.root_hash
    }
}

/// An element in a Merkle proof path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofElement {
    /// Hash of the sibling node
    pub hash: String,
    /// Whether the sibling is on the left
    pub is_left: bool,
}

/// Computes a simple hash (same as in lib.rs).
fn compute_hash(input: &str) -> String {
    let mut hash: u64 = 0;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    format!("{:x}", hash)
}

/// Batch verification of multiple records.
pub struct BatchVerifier {
    proofs: HashMap<Uuid, MerkleProof>,
}

impl BatchVerifier {
    /// Creates a new batch verifier.
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
        }
    }

    /// Adds a proof to the batch.
    pub fn add_proof(&mut self, proof: MerkleProof) {
        self.proofs.insert(proof.record_id, proof);
    }

    /// Verifies all proofs in the batch.
    pub fn verify_all(&self, records: &[AuditRecord]) -> AuditResult<bool> {
        for record in records {
            if let Some(proof) = self.proofs.get(&record.id) {
                if !proof.verify_record(record) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Returns the number of proofs in the batch.
    pub fn len(&self) -> usize {
        self.proofs.len()
    }

    /// Checks if the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.proofs.is_empty()
    }
}

impl Default for BatchVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_merkle_tree_creation() {
        let records = vec![
            create_test_record("statute-1"),
            create_test_record("statute-2"),
            create_test_record("statute-3"),
        ];

        let tree = MerkleTree::from_records(&records);
        assert!(tree.root.is_some());
        assert_eq!(tree.record_count, 3);
        assert!(tree.verify());
    }

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::from_records(&[]);
        assert!(tree.root.is_none());
        assert_eq!(tree.record_count, 0);
        assert!(tree.verify());
    }

    #[test]
    fn test_single_record_tree() {
        let records = vec![create_test_record("statute-1")];
        let tree = MerkleTree::from_records(&records);
        assert!(tree.root.is_some());
        assert!(tree.verify());
    }

    #[test]
    fn test_merkle_proof() {
        let records = vec![
            create_test_record("statute-1"),
            create_test_record("statute-2"),
            create_test_record("statute-3"),
            create_test_record("statute-4"),
        ];

        let tree = MerkleTree::from_records(&records);
        let record_id = records[1].id;

        let proof = tree.generate_proof(record_id).unwrap();
        assert_eq!(proof.record_id, record_id);
        assert!(proof.verify_record(&records[1]));
    }

    #[test]
    fn test_batch_verifier() {
        let records = vec![
            create_test_record("statute-1"),
            create_test_record("statute-2"),
            create_test_record("statute-3"),
        ];

        let tree = MerkleTree::from_records(&records);
        let mut verifier = BatchVerifier::new();

        for record in &records {
            if let Some(proof) = tree.generate_proof(record.id) {
                verifier.add_proof(proof);
            }
        }

        assert_eq!(verifier.len(), 3);
        assert!(verifier.verify_all(&records).unwrap());
    }

    #[test]
    fn test_proof_verification_fails_for_wrong_record() {
        let records = vec![
            create_test_record("statute-1"),
            create_test_record("statute-2"),
        ];

        let tree = MerkleTree::from_records(&records);
        let proof = tree.generate_proof(records[0].id).unwrap();

        // Try to verify with wrong record
        assert!(!proof.verify_record(&records[1]));
    }

    #[test]
    fn test_large_tree() {
        let records: Vec<_> = (0..100)
            .map(|i| create_test_record(&format!("statute-{}", i)))
            .collect();

        let tree = MerkleTree::from_records(&records);
        assert!(tree.verify());
        assert_eq!(tree.record_count, 100);

        // Test proof for a random record
        let proof = tree.generate_proof(records[42].id).unwrap();
        assert!(proof.verify_record(&records[42]));
    }
}
