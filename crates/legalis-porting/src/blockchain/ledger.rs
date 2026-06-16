//! Immutable porting records on an append-only block-chain.
//!
//! A [`PortingLedger`] is a chain of [`Block`]s. Each block bundles a batch of
//! [`PortingLedgerRecord`]s (one per [`PortedStatute`]), commits to them with a
//! Merkle root, links to its predecessor by hash, and is sealed by
//! proof-of-work. Once sealed, any modification to a recorded port — or to the
//! order or linkage of blocks — is detectable by [`PortingLedger::validate`],
//! and the inclusion of a specific record can be proven with a compact
//! [`MerkleProof`] without revealing the rest of the block.
//!
//! The Merkle root that anchors every block is the *cryptographic audit trail*:
//! it is a single 256-bit commitment to the entire batch of ports, and an
//! inclusion proof authenticates one port against that commitment in
//! `O(log n)` sibling hashes.

use super::{ZERO_HASH, current_timestamp, sha256_hex, sha256_parts};
use crate::{PortedStatute, PortingError};
use serde::{Deserialize, Serialize};

/// Result alias local to the blockchain modules, over the crate's error type.
pub(crate) type LedgerResult<T> = Result<T, PortingError>;

/// A single porting operation committed to the ledger.
///
/// The full [`PortedStatute`] is retained for replay/audit, alongside a content
/// hash that is what actually gets committed into the Merkle tree. The source
/// and target jurisdiction codes, change count and compatibility score are
/// denormalised for cheap querying without rehashing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingLedgerRecord {
    /// Identifier of the source statute that was ported.
    pub original_id: String,
    /// Identifier of the resulting ported statute.
    pub ported_id: String,
    /// Source jurisdiction code (e.g. `"JP"`).
    pub source_jurisdiction: String,
    /// Target jurisdiction code (e.g. `"US"`).
    pub target_jurisdiction: String,
    /// SHA-256 of the canonical JSON serialization of the ported statute.
    pub content_hash: String,
    /// Number of adaptations captured by the port.
    pub change_count: usize,
    /// Compatibility score of the port (0.0 - 1.0).
    pub compatibility_score: f64,
    /// Identifier of the actor that recorded the port.
    pub actor: String,
    /// UNIX timestamp (seconds) the record was created.
    pub timestamp: u64,
    /// The full ported-statute payload.
    pub ported: PortedStatute,
}

impl PortingLedgerRecord {
    /// Builds a record from a ported statute, computing its content hash.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the jurisdiction codes are empty
    /// or the ported statute cannot be serialized.
    pub fn from_ported(
        ported: &PortedStatute,
        source_jurisdiction: impl Into<String>,
        target_jurisdiction: impl Into<String>,
        actor: impl Into<String>,
    ) -> LedgerResult<Self> {
        let source_jurisdiction = source_jurisdiction.into();
        let target_jurisdiction = target_jurisdiction.into();
        if source_jurisdiction.trim().is_empty() || target_jurisdiction.trim().is_empty() {
            return Err(PortingError::InvalidInput(
                "porting record requires non-empty source and target jurisdiction codes"
                    .to_string(),
            ));
        }
        let bytes = serde_json::to_vec(ported).map_err(|e| {
            PortingError::InvalidInput(format!("failed to serialize ported statute: {e}"))
        })?;
        Ok(Self {
            original_id: ported.original_id.clone(),
            ported_id: ported.statute.id.clone(),
            source_jurisdiction,
            target_jurisdiction,
            content_hash: sha256_hex(&bytes),
            change_count: ported.changes.len(),
            compatibility_score: ported.compatibility_score,
            actor: actor.into(),
            timestamp: current_timestamp(),
            ported: ported.clone(),
        })
    }

    /// The leaf hash committed into the Merkle tree.
    ///
    /// Binds together every field of the record so that tampering with any of
    /// them (not just the ported-statute body) changes the leaf.
    pub fn leaf_hash(&self) -> String {
        sha256_parts(&[
            self.original_id.as_bytes(),
            self.ported_id.as_bytes(),
            self.source_jurisdiction.as_bytes(),
            self.target_jurisdiction.as_bytes(),
            self.content_hash.as_bytes(),
            &(self.change_count as u64).to_le_bytes(),
            &self.compatibility_score.to_bits().to_le_bytes(),
            self.actor.as_bytes(),
            &self.timestamp.to_le_bytes(),
        ])
    }

    /// Recomputes the content hash from the embedded ported statute and checks it
    /// matches the stored [`PortingLedgerRecord::content_hash`].
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the ported statute cannot be
    /// serialized.
    pub fn verify_content(&self) -> LedgerResult<bool> {
        let bytes = serde_json::to_vec(&self.ported).map_err(|e| {
            PortingError::InvalidInput(format!("failed to serialize ported statute: {e}"))
        })?;
        Ok(sha256_hex(&bytes) == self.content_hash)
    }

    /// The directed jurisdiction corridor of this port, as `(source, target)`.
    pub fn corridor(&self) -> (&str, &str) {
        (&self.source_jurisdiction, &self.target_jurisdiction)
    }
}

/// A binary Merkle tree over a set of leaf hashes.
///
/// Odd levels are handled by promoting the final node (hashed against itself),
/// the standard Bitcoin-style construction. The tree supports compact inclusion
/// proofs via [`MerkleTree::proof`].
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
    /// Returns [`PortingError::InvalidInput`] if `index` is out of range.
    pub fn proof(&self, index: usize) -> LedgerResult<MerkleProof> {
        let leaves = self.levels.first().ok_or_else(|| {
            PortingError::InvalidInput("empty Merkle tree has no leaves to prove".to_string())
        })?;
        if index >= leaves.len() {
            return Err(PortingError::InvalidInput(format!(
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
                // The current node is the left child; the sibling sits on the
                // right and is duplicated when this is the final unpaired node.
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

/// A sealed block in the [`PortingLedger`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Height of this block (0 == genesis).
    pub index: u64,
    /// UNIX timestamp (seconds) the block was sealed.
    pub timestamp: u64,
    /// Hash of the predecessor block (zero hash for genesis).
    pub previous_hash: String,
    /// Merkle root over the block's records (the batch audit anchor).
    pub merkle_root: String,
    /// Proof-of-work nonce that satisfies the difficulty target.
    pub nonce: u64,
    /// The block's own hash.
    pub hash: String,
    /// Records committed by this block.
    pub records: Vec<PortingLedgerRecord>,
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

/// An append-only, Merkle-anchored chain of porting records.
///
/// Records are first staged (see [`PortingLedger::stage`]); calling
/// [`PortingLedger::seal_pending`] mines them into a new block via proof-of-work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingLedger {
    /// Number of leading zero hex digits a block hash must have.
    difficulty: usize,
    /// The sealed chain, starting with the genesis block.
    blocks: Vec<Block>,
    /// Records staged for the next block.
    pending: Vec<PortingLedgerRecord>,
}

impl PortingLedger {
    /// Creates a new ledger with a genesis block and the given proof-of-work
    /// difficulty (leading zero hex digits; values above 6 become very slow and
    /// are therefore capped at 6).
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
        self.blocks.last().unwrap_or(&self.blocks[0])
    }

    /// All sealed blocks.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Records staged but not yet sealed.
    pub fn pending(&self) -> &[PortingLedgerRecord] {
        &self.pending
    }

    /// Stages a record for inclusion in the next block.
    pub fn stage(&mut self, record: PortingLedgerRecord) {
        self.pending.push(record);
    }

    /// Seals all pending records into a new block via proof-of-work.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if there are no pending records to
    /// seal.
    pub fn seal_pending(&mut self) -> LedgerResult<&Block> {
        if self.pending.is_empty() {
            return Err(PortingError::InvalidInput(
                "no pending porting records to seal".to_string(),
            ));
        }
        let records = std::mem::take(&mut self.pending);
        let leaves: Vec<String> = records.iter().map(PortingLedgerRecord::leaf_hash).collect();
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

    /// Returns every committed record whose source statute matches `original_id`,
    /// in chain (chronological) order — the audit trail of one statute's ports.
    pub fn audit_trail(&self, original_id: &str) -> Vec<&PortingLedgerRecord> {
        self.blocks
            .iter()
            .flat_map(|b| b.records.iter())
            .filter(|r| r.original_id == original_id)
            .collect()
    }

    /// Returns every committed record on the directed `source -> target`
    /// jurisdiction corridor.
    pub fn records_for_corridor(&self, source: &str, target: &str) -> Vec<&PortingLedgerRecord> {
        self.blocks
            .iter()
            .flat_map(|b| b.records.iter())
            .filter(|r| r.source_jurisdiction == source && r.target_jurisdiction == target)
            .collect()
    }

    /// Returns every committed record whose target jurisdiction matches.
    pub fn records_for_target(&self, target: &str) -> Vec<&PortingLedgerRecord> {
        self.blocks
            .iter()
            .flat_map(|b| b.records.iter())
            .filter(|r| r.target_jurisdiction == target)
            .collect()
    }

    /// Validates the entire chain: per-block hash recomputation, difficulty,
    /// Merkle-root consistency, linkage and per-record content integrity.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] describing the first violation.
    pub fn validate(&self) -> LedgerResult<()> {
        if self.blocks.is_empty() {
            return Err(PortingError::InvalidInput(
                "ledger has no blocks".to_string(),
            ));
        }
        for (height, block) in self.blocks.iter().enumerate() {
            if block.index as usize != height {
                return Err(PortingError::InvalidInput(format!(
                    "chain integrity: block at position {} has index {}",
                    height, block.index
                )));
            }

            // Recompute and compare the block hash.
            let recomputed = block.recompute_hash();
            if recomputed != block.hash {
                return Err(PortingError::InvalidInput(format!(
                    "chain integrity: block {} hash mismatch (header tampered)",
                    block.index
                )));
            }

            // Proof-of-work must hold for every block after genesis.
            if height > 0 && !Block::meets_difficulty(&block.hash, self.difficulty) {
                return Err(PortingError::InvalidInput(format!(
                    "chain integrity: block {} does not satisfy proof-of-work",
                    block.index
                )));
            }

            // Merkle root must commit exactly to the stored records.
            let leaves: Vec<String> = block
                .records
                .iter()
                .map(PortingLedgerRecord::leaf_hash)
                .collect();
            let expected_root = MerkleTree::build(&leaves).root();
            if expected_root != block.merkle_root {
                return Err(PortingError::InvalidInput(format!(
                    "chain integrity: block {} Merkle root mismatch (records tampered)",
                    block.index
                )));
            }

            // Each record's embedded ported statute must match its content hash.
            for record in &block.records {
                if !record.verify_content()? {
                    return Err(PortingError::InvalidInput(format!(
                        "chain integrity: record for '{}' in block {} fails content verification",
                        record.original_id, block.index
                    )));
                }
            }

            // Linkage to the predecessor.
            if height == 0 {
                if block.previous_hash != ZERO_HASH {
                    return Err(PortingError::InvalidInput(
                        "chain integrity: genesis block has a non-zero previous hash".to_string(),
                    ));
                }
            } else {
                let prev = &self.blocks[height - 1];
                if block.previous_hash != prev.hash {
                    return Err(PortingError::InvalidInput(format!(
                        "chain integrity: block {} previous-hash does not link to block {}",
                        block.index, prev.index
                    )));
                }
            }
        }
        Ok(())
    }

    /// Produces an inclusion proof for the record at `record_index` within the
    /// block at `block_index`, authenticating it against that block's Merkle
    /// root (the batch audit anchor).
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if either index is out of range.
    pub fn inclusion_proof(
        &self,
        block_index: usize,
        record_index: usize,
    ) -> LedgerResult<MerkleProof> {
        let block = self.blocks.get(block_index).ok_or_else(|| {
            PortingError::InvalidInput(format!("block index {block_index} out of range"))
        })?;
        let leaves: Vec<String> = block
            .records
            .iter()
            .map(PortingLedgerRecord::leaf_hash)
            .collect();
        let tree = MerkleTree::build(&leaves);
        tree.proof(record_index)
    }
}

/// Mines a block: searches for a nonce whose resulting hash meets the difficulty
/// target. Returns `(nonce, hash)`.
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

/// Convenience helper: stages a ported statute onto the ledger as a record.
///
/// Equivalent to building a [`PortingLedgerRecord`] and calling
/// [`PortingLedger::stage`]. Call [`PortingLedger::seal_pending`] afterwards to
/// mine the staged records.
///
/// # Errors
///
/// Returns [`PortingError::InvalidInput`] if the record cannot be built.
pub fn record_port(
    ledger: &mut PortingLedger,
    ported: &PortedStatute,
    source_jurisdiction: impl Into<String>,
    target_jurisdiction: impl Into<String>,
    actor: impl Into<String>,
) -> LedgerResult<()> {
    let record =
        PortingLedgerRecord::from_ported(ported, source_jurisdiction, target_jurisdiction, actor)?;
    ledger.stage(record);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};
    use legalis_i18n::Locale;

    fn ported(original: &str, ported_id: &str, score: f64, changes: usize) -> PortedStatute {
        let mut change_vec = Vec::new();
        for i in 0..changes {
            change_vec.push(crate::PortingChange {
                change_type: crate::ChangeType::Translation,
                description: format!("change {i}"),
                original: Some(format!("o{i}")),
                adapted: Some(format!("a{i}")),
                reason: "test".to_string(),
            });
        }
        PortedStatute {
            original_id: original.to_string(),
            statute: Statute::new(
                ported_id,
                "Ported",
                Effect::new(EffectType::Grant, "Benefit"),
            ),
            changes: change_vec,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: score,
        }
    }

    #[test]
    fn test_genesis_chain_is_valid() {
        let ledger = PortingLedger::new(1);
        assert_eq!(ledger.len(), 1);
        assert_eq!(ledger.record_count(), 0);
        assert_eq!(ledger.latest_block().index, 0);
        assert_eq!(ledger.latest_block().previous_hash, ZERO_HASH);
        ledger.validate().expect("genesis chain valid");
    }

    #[test]
    fn test_record_and_seal() {
        let mut ledger = PortingLedger::new(2);
        let p = ported("jp-1", "us-1", 0.9, 2);
        record_port(&mut ledger, &p, "JP", "US", "alice").expect("stage");
        assert_eq!(ledger.pending().len(), 1);
        ledger.seal_pending().expect("seal");
        assert_eq!(ledger.len(), 2);
        assert_eq!(ledger.record_count(), 1);
        assert!(ledger.pending().is_empty());
        ledger.validate().expect("chain valid after seal");
    }

    #[test]
    fn test_record_rejects_empty_jurisdiction() {
        let p = ported("jp-1", "us-1", 0.9, 0);
        assert!(PortingLedgerRecord::from_ported(&p, "", "US", "a").is_err());
        assert!(PortingLedgerRecord::from_ported(&p, "JP", "  ", "a").is_err());
    }

    #[test]
    fn test_seal_empty_is_error() {
        let mut ledger = PortingLedger::new(1);
        assert!(ledger.seal_pending().is_err());
    }

    #[test]
    fn test_proof_of_work_difficulty_met() {
        let mut ledger = PortingLedger::new(3);
        record_port(&mut ledger, &ported("a", "b", 0.5, 1), "JP", "US", "bob").expect("stage");
        let block = ledger.seal_pending().expect("seal").clone();
        assert!(Block::meets_difficulty(&block.hash, 3));
        assert!(block.hash.starts_with("000"));
    }

    #[test]
    fn test_multiple_blocks_link() {
        let mut ledger = PortingLedger::new(2);
        for i in 0..3 {
            record_port(
                &mut ledger,
                &ported(&format!("s{i}"), &format!("t{i}"), 0.8, i),
                "JP",
                "US",
                "ed",
            )
            .expect("stage");
            ledger.seal_pending().expect("seal");
        }
        assert_eq!(ledger.len(), 4); // genesis + 3
        ledger.validate().expect("multi-block chain valid");
        let blocks = ledger.blocks();
        for w in blocks.windows(2) {
            assert_eq!(w[1].previous_hash, w[0].hash);
        }
    }

    #[test]
    fn test_tamper_with_record_detected() {
        let mut ledger = PortingLedger::new(1);
        record_port(
            &mut ledger,
            &ported("jp-t", "us-t", 0.7, 1),
            "JP",
            "US",
            "a",
        )
        .expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.validate().expect("valid before tamper");
        ledger.blocks[1].records[0].ported.original_id = "hacked".to_string();
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_tamper_with_block_header_detected() {
        let mut ledger = PortingLedger::new(1);
        record_port(
            &mut ledger,
            &ported("jp-h", "us-h", 0.7, 0),
            "JP",
            "US",
            "a",
        )
        .expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.blocks[1].timestamp += 9999;
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_broken_linkage_detected() {
        let mut ledger = PortingLedger::new(1);
        record_port(
            &mut ledger,
            &ported("jp-l", "us-l", 0.7, 0),
            "JP",
            "US",
            "a",
        )
        .expect("stage");
        ledger.seal_pending().expect("seal");
        ledger.blocks[1].previous_hash = ZERO_HASH.to_string();
        let (nonce, hash) = mine(&ledger.blocks[1], ledger.difficulty);
        ledger.blocks[1].nonce = nonce;
        ledger.blocks[1].hash = hash;
        assert!(ledger.validate().is_err());
    }

    #[test]
    fn test_audit_trail_and_corridor_queries() {
        let mut ledger = PortingLedger::new(1);
        record_port(
            &mut ledger,
            &ported("alpha", "us-a", 0.9, 0),
            "JP",
            "US",
            "u",
        )
        .expect("s");
        record_port(
            &mut ledger,
            &ported("beta", "de-b", 0.8, 0),
            "JP",
            "DE",
            "u",
        )
        .expect("s");
        record_port(
            &mut ledger,
            &ported("alpha", "de-a", 0.7, 0),
            "JP",
            "DE",
            "u",
        )
        .expect("s");
        ledger.seal_pending().expect("seal");
        assert_eq!(ledger.audit_trail("alpha").len(), 2);
        assert_eq!(ledger.audit_trail("beta").len(), 1);
        assert_eq!(ledger.audit_trail("missing").len(), 0);
        assert_eq!(ledger.records_for_corridor("JP", "DE").len(), 2);
        assert_eq!(ledger.records_for_corridor("JP", "US").len(), 1);
        assert_eq!(ledger.records_for_target("DE").len(), 2);
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
        let leaves: Vec<String> = (0..5)
            .map(|i| sha256_hex(format!("leaf-{i}").as_bytes()))
            .collect();
        let tree = MerkleTree::build(&leaves);
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof(i).expect("proof");
            assert!(proof.verify(), "proof for leaf {i} should verify");
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
            .map(|i| sha256_hex(format!("x{i}").as_bytes()))
            .collect();
        let tree = MerkleTree::build(&leaves);
        let mut proof = tree.proof(2).expect("proof");
        assert!(proof.verify());
        proof.leaf = sha256_hex(b"forged");
        assert!(!proof.verify());
    }

    #[test]
    fn test_ledger_inclusion_proof() {
        let mut ledger = PortingLedger::new(1);
        for i in 0u32..3 {
            record_port(
                &mut ledger,
                &ported(&format!("s{i}"), &format!("t{i}"), 0.6, i as usize),
                "JP",
                "US",
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
    fn test_record_content_verification_and_corridor() {
        let p = ported("verify-me", "us-v", 0.95, 3);
        let record = PortingLedgerRecord::from_ported(&p, "JP", "US", "alice").expect("record");
        assert!(record.verify_content().expect("verify"));
        assert_eq!(record.change_count, 3);
        assert_eq!(record.corridor(), ("JP", "US"));
    }

    #[test]
    fn test_difficulty_capped() {
        let ledger = PortingLedger::new(100);
        assert_eq!(ledger.difficulty(), 6);
    }

    #[test]
    fn test_ledger_serde_roundtrip() {
        let mut ledger = PortingLedger::new(1);
        record_port(&mut ledger, &ported("x", "y", 0.5, 1), "JP", "US", "a").expect("stage");
        ledger.seal_pending().expect("seal");
        let json = serde_json::to_string(&ledger).expect("ser");
        let back: PortingLedger = serde_json::from_str(&json).expect("de");
        back.validate().expect("deserialized chain valid");
        assert_eq!(back.record_count(), 1);
    }
}
