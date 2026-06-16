//! Immutable diff recording on an append-only block-chain.
//!
//! A [`DiffLedger`] is a chain of [`Block`]s. Each block bundles a batch of
//! [`DiffRecord`]s (one per [`crate::StatuteDiff`]), commits to them with a
//! Merkle root, links to its predecessor by hash, and is sealed by
//! proof-of-work. Once sealed, any modification to a recorded diff — or to the
//! order or linkage of blocks — is detectable by [`DiffLedger::validate`], and
//! the inclusion of a specific record can be proven with a compact
//! [`MerkleProof`] without revealing the rest of the block.

use super::{current_timestamp, sha256_hex, sha256_parts};
use crate::{DiffError, DiffResult, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// The all-zero hash used as the genesis block's predecessor.
const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// A single statute diff committed to the ledger.
///
/// The full diff is retained for replay/audit, alongside a content hash that is
/// what actually gets committed into the Merkle tree. Severity and change count
/// are denormalised for cheap querying without rehashing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffRecord {
    /// The statute the diff applies to.
    pub statute_id: String,
    /// SHA-256 of the canonical JSON serialization of the diff.
    pub diff_hash: String,
    /// Overall severity of the recorded diff.
    pub severity: Severity,
    /// Number of changes captured by the diff.
    pub change_count: usize,
    /// Identifier of the actor that recorded the diff.
    pub recorder: String,
    /// UNIX timestamp (seconds) the record was created.
    pub timestamp: u64,
    /// The full diff payload.
    pub diff: StatuteDiff,
}

impl DiffRecord {
    /// Builds a record from a diff, computing its content hash.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the diff cannot be
    /// serialized.
    pub fn from_diff(diff: &StatuteDiff, recorder: impl Into<String>) -> DiffResult<Self> {
        let bytes = serde_json::to_vec(diff).map_err(|e| {
            DiffError::SerializationError(format!("failed to serialize diff: {}", e))
        })?;
        Ok(Self {
            statute_id: diff.statute_id.clone(),
            diff_hash: sha256_hex(&bytes),
            severity: diff.impact.severity,
            change_count: diff.changes.len(),
            recorder: recorder.into(),
            timestamp: current_timestamp(),
            diff: diff.clone(),
        })
    }

    /// The leaf hash committed into the Merkle tree.
    ///
    /// Binds together every field of the record so that tampering with any of
    /// them (not just the diff body) changes the leaf.
    pub fn leaf_hash(&self) -> String {
        sha256_parts(&[
            self.statute_id.as_bytes(),
            self.diff_hash.as_bytes(),
            &(self.severity as u8).to_le_bytes(),
            &(self.change_count as u64).to_le_bytes(),
            self.recorder.as_bytes(),
            &self.timestamp.to_le_bytes(),
        ])
    }

    /// Recomputes the content hash from the embedded diff and checks it matches
    /// the stored `diff_hash`.
    pub fn verify_content(&self) -> DiffResult<bool> {
        let bytes = serde_json::to_vec(&self.diff).map_err(|e| {
            DiffError::SerializationError(format!("failed to serialize diff: {}", e))
        })?;
        Ok(sha256_hex(&bytes) == self.diff_hash)
    }
}

/// A binary Merkle tree over a set of leaf hashes.
///
/// Odd levels are handled by promoting the final node (hashed against itself),
/// the standard Bitcoin-style construction. The tree supports compact
/// inclusion proofs via [`MerkleTree::proof`].
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Each entry is one level of the tree, level 0 being the leaves.
    levels: Vec<Vec<String>>,
}

impl MerkleTree {
    /// Builds a Merkle tree from leaf hashes. An empty input yields a tree whose
    /// root is the all-zero hash.
    pub fn build(leaves: &[String]) -> Self {
        if leaves.is_empty() {
            // An empty tree has no leaves; its root resolves to the zero hash.
            return Self {
                levels: vec![Vec::new()],
            };
        }
        let mut levels = vec![leaves.to_vec()];
        let mut current = leaves.to_vec();
        while current.len() > 1 {
            let mut next = Vec::with_capacity(current.len().div_ceil(2));
            let mut i = 0;
            while i < current.len() {
                let left = &current[i];
                let right = if i + 1 < current.len() {
                    &current[i + 1]
                } else {
                    left
                };
                next.push(sha256_parts(&[left.as_bytes(), right.as_bytes()]));
                i += 2;
            }
            levels.push(next.clone());
            current = next;
        }
        Self { levels }
    }

    /// The Merkle root committing to all leaves.
    pub fn root(&self) -> String {
        self.levels
            .last()
            .and_then(|lvl| lvl.first())
            .cloned()
            .unwrap_or_else(|| ZERO_HASH.to_string())
    }

    /// Number of leaves in the tree.
    pub fn leaf_count(&self) -> usize {
        self.levels.first().map(|lvl| lvl.len()).unwrap_or(0)
    }

    /// Generates an inclusion proof for the leaf at `index`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InvalidComparison`] if `index` is out of range.
    pub fn proof(&self, index: usize) -> DiffResult<MerkleProof> {
        let leaves = self
            .levels
            .first()
            .ok_or_else(|| DiffError::ChainIntegrity("empty Merkle tree".to_string()))?;
        if index >= leaves.len() {
            return Err(DiffError::InvalidComparison(format!(
                "leaf index {} out of range (have {} leaves)",
                index,
                leaves.len()
            )));
        }
        let mut siblings = Vec::new();
        let mut idx = index;
        for level in &self.levels {
            if level.len() <= 1 {
                break;
            }
            let (sibling_index, is_left) = if idx.is_multiple_of(2) {
                // The current node is the left child; sibling is on the right,
                // duplicated when this is the final, unpaired node.
                (if idx + 1 < level.len() { idx + 1 } else { idx }, false)
            } else {
                (idx - 1, true)
            };
            siblings.push(SiblingHash {
                hash: level[sibling_index].clone(),
                sibling_is_left: is_left,
            });
            idx /= 2;
        }
        Ok(MerkleProof {
            leaf: leaves[index].clone(),
            siblings,
            root: self.root(),
        })
    }
}

/// One step in a Merkle inclusion proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiblingHash {
    /// The sibling node hash to combine with.
    pub hash: String,
    /// Whether the sibling sits to the left of the running hash.
    pub sibling_is_left: bool,
}

/// A compact proof that a leaf is included under a Merkle root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    /// The leaf hash being proven.
    pub leaf: String,
    /// The authentication path from leaf to root.
    pub siblings: Vec<SiblingHash>,
    /// The root the proof should reconstruct.
    pub root: String,
}

impl MerkleProof {
    /// Recomputes the root from `leaf` and the authentication path and checks it
    /// matches [`MerkleProof::root`].
    pub fn verify(&self) -> bool {
        let mut running = self.leaf.clone();
        for sibling in &self.siblings {
            running = if sibling.sibling_is_left {
                sha256_parts(&[sibling.hash.as_bytes(), running.as_bytes()])
            } else {
                sha256_parts(&[running.as_bytes(), sibling.hash.as_bytes()])
            };
        }
        running == self.root
    }
}

/// A sealed block in the [`DiffLedger`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Height of this block (0 == genesis).
    pub index: u64,
    /// UNIX timestamp (seconds) the block was sealed.
    pub timestamp: u64,
    /// Hash of the predecessor block (zero hash for genesis).
    pub previous_hash: String,
    /// Merkle root over the block's records.
    pub merkle_root: String,
    /// Proof-of-work nonce that satisfies the difficulty target.
    pub nonce: u64,
    /// The block's own hash.
    pub hash: String,
    /// Records committed by this block.
    pub records: Vec<DiffRecord>,
}

impl Block {
    /// The pre-image hashed for proof-of-work, excluding the nonce/hash.
    fn header_seed(index: u64, timestamp: u64, previous_hash: &str, merkle_root: &str) -> Vec<u8> {
        let mut seed = Vec::new();
        seed.extend_from_slice(&index.to_le_bytes());
        seed.extend_from_slice(&timestamp.to_le_bytes());
        seed.extend_from_slice(previous_hash.as_bytes());
        seed.extend_from_slice(merkle_root.as_bytes());
        seed
    }

    /// Computes the block hash for a given nonce.
    fn compute_hash(
        index: u64,
        timestamp: u64,
        previous_hash: &str,
        merkle_root: &str,
        nonce: u64,
    ) -> String {
        let mut seed = Self::header_seed(index, timestamp, previous_hash, merkle_root);
        seed.extend_from_slice(&nonce.to_le_bytes());
        sha256_hex(&seed)
    }

    /// Recomputes this block's hash from its header fields.
    pub fn recompute_hash(&self) -> String {
        Self::compute_hash(
            self.index,
            self.timestamp,
            &self.previous_hash,
            &self.merkle_root,
            self.nonce,
        )
    }

    /// Whether `hash` satisfies a difficulty of `difficulty` leading zero hex
    /// digits.
    pub fn meets_difficulty(hash: &str, difficulty: usize) -> bool {
        hash.bytes().take(difficulty).all(|b| b == b'0')
    }
}

/// An append-only, Merkle-anchored chain of statute-diff records.
///
/// Records are first staged (see [`DiffLedger::stage`]); calling
/// [`DiffLedger::seal_pending`] mines them into a new block via proof-of-work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLedger {
    /// Number of leading zero hex digits a block hash must have.
    difficulty: usize,
    /// The sealed chain, starting with the genesis block.
    blocks: Vec<Block>,
    /// Records staged for the next block.
    pending: Vec<DiffRecord>,
}

impl DiffLedger {
    /// Creates a new ledger with a genesis block and the given proof-of-work
    /// difficulty (leading zero hex digits; values above 6 become very slow).
    pub fn new(difficulty: usize) -> Self {
        let difficulty = difficulty.min(6);
        let timestamp = current_timestamp();
        let merkle_root = MerkleTree::build(&[]).root();
        let mut block = Block {
            index: 0,
            timestamp,
            previous_hash: ZERO_HASH.to_string(),
            merkle_root,
            nonce: 0,
            hash: String::new(),
            records: Vec::new(),
        };
        // The genesis block is a fixed anchor (nonce 0); like other chains it is
        // exempt from the proof-of-work target so construction is instant even at
        // high difficulty.
        block.hash = block.recompute_hash();
        Self {
            difficulty,
            blocks: vec![block],
            pending: Vec::new(),
        }
    }

    /// The configured proof-of-work difficulty.
    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    /// Number of sealed blocks, including genesis.
    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    /// Always `false`: a ledger always contains at least the genesis block.
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// The most recently sealed block.
    pub fn latest_block(&self) -> &Block {
        // The genesis block is always present and blocks are never removed, so
        // `last()` always yields a block; the fallback is never reached.
        self.blocks.last().unwrap_or_else(|| &self.blocks[0])
    }

    /// All sealed blocks.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Records staged but not yet sealed.
    pub fn pending(&self) -> &[DiffRecord] {
        &self.pending
    }

    /// Stages a record for inclusion in the next block.
    pub fn stage(&mut self, record: DiffRecord) {
        self.pending.push(record);
    }

    /// Seals all pending records into a new block via proof-of-work.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::UnsupportedOperation`] if there are no pending
    /// records to seal.
    pub fn seal_pending(&mut self) -> DiffResult<&Block> {
        if self.pending.is_empty() {
            return Err(DiffError::UnsupportedOperation(
                "no pending records to seal".to_string(),
            ));
        }
        let records = std::mem::take(&mut self.pending);
        let leaves: Vec<String> = records.iter().map(DiffRecord::leaf_hash).collect();
        let merkle_root = MerkleTree::build(&leaves).root();
        let (prev_index, prev_hash) = {
            let previous = self.latest_block();
            (previous.index, previous.hash.clone())
        };
        let mut block = Block {
            index: prev_index + 1,
            timestamp: current_timestamp(),
            previous_hash: prev_hash,
            merkle_root,
            nonce: 0,
            hash: String::new(),
            records,
        };
        let (nonce, hash) = mine(&block, self.difficulty);
        block.nonce = nonce;
        block.hash = hash;
        self.blocks.push(block);
        Ok(self.latest_block())
    }

    /// Total number of records committed across all sealed blocks.
    pub fn record_count(&self) -> usize {
        self.blocks.iter().map(|b| b.records.len()).sum()
    }

    /// Returns every record in the chain whose `statute_id` matches.
    pub fn records_for(&self, statute_id: &str) -> Vec<&DiffRecord> {
        self.blocks
            .iter()
            .flat_map(|b| b.records.iter())
            .filter(|r| r.statute_id == statute_id)
            .collect()
    }

    /// Validates the entire chain: per-block hash recomputation, difficulty,
    /// Merkle-root consistency, linkage and per-record content integrity.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::ChainIntegrity`] describing the first violation.
    pub fn validate(&self) -> DiffResult<()> {
        if self.blocks.is_empty() {
            return Err(DiffError::ChainIntegrity(
                "ledger has no blocks".to_string(),
            ));
        }
        for (height, block) in self.blocks.iter().enumerate() {
            if block.index as usize != height {
                return Err(DiffError::ChainIntegrity(format!(
                    "block at position {} has index {}",
                    height, block.index
                )));
            }

            // Recompute and compare the block hash.
            let recomputed = block.recompute_hash();
            if recomputed != block.hash {
                return Err(DiffError::ChainIntegrity(format!(
                    "block {} hash mismatch (header tampered)",
                    block.index
                )));
            }

            // Proof-of-work must hold for every block after genesis.
            if height > 0 && !Block::meets_difficulty(&block.hash, self.difficulty) {
                return Err(DiffError::ChainIntegrity(format!(
                    "block {} does not satisfy proof-of-work",
                    block.index
                )));
            }

            // Merkle root must commit exactly to the stored records.
            let leaves: Vec<String> = block.records.iter().map(DiffRecord::leaf_hash).collect();
            let expected_root = MerkleTree::build(&leaves).root();
            if expected_root != block.merkle_root {
                return Err(DiffError::ChainIntegrity(format!(
                    "block {} Merkle root mismatch (records tampered)",
                    block.index
                )));
            }

            // Each record's embedded diff must match its content hash.
            for record in &block.records {
                if !record.verify_content()? {
                    return Err(DiffError::ChainIntegrity(format!(
                        "record for '{}' in block {} fails content verification",
                        record.statute_id, block.index
                    )));
                }
            }

            // Linkage to the predecessor.
            if height == 0 {
                if block.previous_hash != ZERO_HASH {
                    return Err(DiffError::ChainIntegrity(
                        "genesis block has a non-zero previous hash".to_string(),
                    ));
                }
            } else {
                let prev = &self.blocks[height - 1];
                if block.previous_hash != prev.hash {
                    return Err(DiffError::ChainIntegrity(format!(
                        "block {} previous-hash does not link to block {}",
                        block.index, prev.index
                    )));
                }
            }
        }
        Ok(())
    }

    /// Produces an inclusion proof for the record at `record_index` within the
    /// block at `block_index`.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::InvalidComparison`] if either index is out of range.
    pub fn inclusion_proof(
        &self,
        block_index: usize,
        record_index: usize,
    ) -> DiffResult<MerkleProof> {
        let block = self.blocks.get(block_index).ok_or_else(|| {
            DiffError::InvalidComparison(format!("block index {} out of range", block_index))
        })?;
        let leaves: Vec<String> = block.records.iter().map(DiffRecord::leaf_hash).collect();
        let tree = MerkleTree::build(&leaves);
        tree.proof(record_index)
    }
}

/// Mines a block: searches for a nonce whose resulting hash meets the
/// difficulty target. Returns `(nonce, hash)`.
fn mine(block: &Block, difficulty: usize) -> (u64, String) {
    let mut nonce: u64 = 0;
    loop {
        let hash = Block::compute_hash(
            block.index,
            block.timestamp,
            &block.previous_hash,
            &block.merkle_root,
            nonce,
        );
        if Block::meets_difficulty(&hash, difficulty) {
            return (nonce, hash);
        }
        nonce = nonce.wrapping_add(1);
    }
}

/// Convenience helper: stages a diff onto the ledger as a record.
///
/// Equivalent to building a [`DiffRecord`] and calling [`DiffLedger::stage`].
/// Call [`DiffLedger::seal_pending`] afterwards to mine the staged records.
///
/// # Errors
///
/// Returns [`DiffError::SerializationError`] if the diff cannot be serialized.
pub fn record_diff(
    ledger: &mut DiffLedger,
    diff: &StatuteDiff,
    recorder: impl Into<String>,
) -> DiffResult<()> {
    let record = DiffRecord::from_diff(diff, recorder)?;
    ledger.stage(record);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{Effect, EffectType, Statute};

    fn sample_diff(id: &str, severity_breaking: bool) -> StatuteDiff {
        let old = Statute::new(id, "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        if severity_breaking {
            new.effect = Effect::new(EffectType::Revoke, "Revoked");
        } else {
            new.title = "New".to_string();
        }
        diff(&old, &new).expect("diff should succeed")
    }

    #[test]
    fn test_genesis_chain_is_valid() {
        let ledger = DiffLedger::new(1);
        assert_eq!(ledger.len(), 1);
        assert_eq!(ledger.record_count(), 0);
        assert_eq!(ledger.latest_block().index, 0);
        assert_eq!(ledger.latest_block().previous_hash, ZERO_HASH);
        ledger.validate().expect("genesis chain valid");
    }

    #[test]
    fn test_record_and_seal() {
        let mut ledger = DiffLedger::new(2);
        let d = sample_diff("law-1", true);
        record_diff(&mut ledger, &d, "alice").expect("stage");
        assert_eq!(ledger.pending().len(), 1);
        ledger.seal_pending().expect("seal");
        assert_eq!(ledger.len(), 2);
        assert_eq!(ledger.record_count(), 1);
        assert!(ledger.pending().is_empty());
        ledger.validate().expect("chain valid after seal");
    }

    #[test]
    fn test_seal_empty_is_error() {
        let mut ledger = DiffLedger::new(1);
        assert!(ledger.seal_pending().is_err());
    }

    #[test]
    fn test_proof_of_work_difficulty_met() {
        let mut ledger = DiffLedger::new(3);
        record_diff(&mut ledger, &sample_diff("law-x", false), "bob").expect("stage");
        let block = ledger.seal_pending().expect("seal").clone();
        assert!(Block::meets_difficulty(&block.hash, 3));
        assert!(block.hash.starts_with("000"));
    }

    #[test]
    fn test_multiple_blocks_link() {
        let mut ledger = DiffLedger::new(2);
        for i in 0..3 {
            record_diff(
                &mut ledger,
                &sample_diff(&format!("law-{}", i), false),
                "ed",
            )
            .expect("stage");
            ledger.seal_pending().expect("seal");
        }
        assert_eq!(ledger.len(), 4); // genesis + 3
        ledger.validate().expect("multi-block chain valid");
        // Verify explicit linkage.
        let blocks = ledger.blocks();
        for w in blocks.windows(2) {
            assert_eq!(w[1].previous_hash, w[0].hash);
        }
    }

    #[test]
    fn test_tamper_with_record_detected() {
        let mut ledger = DiffLedger::new(1);
        record_diff(&mut ledger, &sample_diff("law-t", true), "alice").expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.validate().expect("valid before tamper");
        // Mutate a committed diff body.
        ledger.blocks[1].records[0].diff.statute_id = "hacked".to_string();
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_tamper_with_block_header_detected() {
        let mut ledger = DiffLedger::new(1);
        record_diff(&mut ledger, &sample_diff("law-h", false), "alice").expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.blocks[1].timestamp += 9999;
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_broken_linkage_detected() {
        let mut ledger = DiffLedger::new(1);
        record_diff(&mut ledger, &sample_diff("law-l", false), "alice").expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.blocks[1].previous_hash = ZERO_HASH.to_string();
        // Re-mine so the hash matches the header but linkage is wrong.
        let (nonce, hash) = mine(&ledger.blocks[1], ledger.difficulty);
        ledger.blocks[1].nonce = nonce;
        ledger.blocks[1].hash = hash;
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_records_for_statute() {
        let mut ledger = DiffLedger::new(1);
        record_diff(&mut ledger, &sample_diff("alpha", false), "u").expect("stage");
        record_diff(&mut ledger, &sample_diff("beta", false), "u").expect("stage");
        record_diff(&mut ledger, &sample_diff("alpha", true), "u").expect("stage");
        ledger.seal_pending().expect("seal");
        assert_eq!(ledger.records_for("alpha").len(), 2);
        assert_eq!(ledger.records_for("beta").len(), 1);
        assert_eq!(ledger.records_for("missing").len(), 0);
    }

    #[test]
    fn test_merkle_root_empty_is_zero() {
        let tree = MerkleTree::build(&[]);
        assert_eq!(tree.root(), ZERO_HASH);
        assert_eq!(tree.leaf_count(), 0);
    }

    #[test]
    fn test_merkle_single_leaf() {
        let leaves = vec![sha256_hex(b"only")];
        let tree = MerkleTree::build(&leaves);
        assert_eq!(tree.leaf_count(), 1);
        let proof = tree.proof(0).expect("proof");
        assert!(proof.verify());
        assert_eq!(proof.root, tree.root());
    }

    #[test]
    fn test_merkle_proof_all_indices_odd_count() {
        // Odd leaf count exercises the duplicated-last-node path.
        let leaves: Vec<String> = (0..5)
            .map(|i| sha256_hex(format!("leaf-{}", i).as_bytes()))
            .collect();
        let tree = MerkleTree::build(&leaves);
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof(i).expect("proof");
            assert!(proof.verify(), "proof for leaf {} should verify", i);
            assert_eq!(&proof.leaf, leaf);
        }
    }

    #[test]
    fn test_merkle_proof_out_of_range() {
        let leaves = vec![sha256_hex(b"a"), sha256_hex(b"b")];
        let tree = MerkleTree::build(&leaves);
        assert!(tree.proof(2).is_err());
    }

    #[test]
    fn test_merkle_proof_rejects_wrong_leaf() {
        let leaves: Vec<String> = (0..4)
            .map(|i| sha256_hex(format!("x{}", i).as_bytes()))
            .collect();
        let tree = MerkleTree::build(&leaves);
        let mut proof = tree.proof(2).expect("proof");
        assert!(proof.verify());
        proof.leaf = sha256_hex(b"forged");
        assert!(!proof.verify());
    }

    #[test]
    fn test_ledger_inclusion_proof() {
        let mut ledger = DiffLedger::new(1);
        for i in 0u32..3 {
            record_diff(
                &mut ledger,
                &sample_diff(&format!("s{}", i), i.is_multiple_of(2)),
                "u",
            )
            .expect("stage");
        }
        ledger.seal_pending().expect("seal");
        let proof = ledger.inclusion_proof(1, 1).expect("inclusion proof");
        assert!(proof.verify());
        assert_eq!(proof.root, ledger.blocks()[1].merkle_root);
    }

    #[test]
    fn test_record_content_verification() {
        let d = sample_diff("verify-me", true);
        let record = DiffRecord::from_diff(&d, "alice").expect("record");
        assert!(record.verify_content().expect("verify"));
        assert_eq!(record.severity, Severity::Major);
        assert!(record.change_count >= 1);
    }

    #[test]
    fn test_difficulty_capped() {
        let ledger = DiffLedger::new(100);
        assert_eq!(ledger.difficulty(), 6);
    }
}
