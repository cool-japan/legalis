//! Quantum-resistant content hashing for registry records and the whole store.
//!
//! Grover's quantum search reduces the cost of a pre-image attack on an `n`-bit
//! hash from `2^n` to `2^(n/2)`. A 256-bit digest therefore retains only ~128
//! bits of post-quantum pre-image security, while a 512-bit digest retains ~256
//! bits. This module exposes a pluggable [`QuantumHashAlgorithm`] favouring large
//! outputs and applies it with registry-specific semantics:
//!
//! - [`ContentHash`] computes and re-verifies a digest over a *canonicalized*
//!   [`StatuteEntry`] (sorted-key JSON), so structurally equal records hash
//!   identically regardless of map iteration order.
//! - [`hash_registry_store`] builds a [`StoreHashManifest`]: a per-statute digest
//!   list plus a Merkle root committing to the entire registry, enabling
//!   tamper-evident integrity snapshots and efficient inclusion checks.

use super::{
    canonical_json_bytes, constant_time_eq, merkle_root, sha256, sha512, sha512_256, to_hex,
};
use crate::{RegistryResult, StatuteEntry, StatuteRegistry};
use serde::{Deserialize, Serialize};

const CTX_LEAF: &[u8] = b"store-leaf";

/// A pluggable, quantum-aware content-hash algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QuantumHashAlgorithm {
    /// SHA-256 (256-bit; ~128-bit post-quantum pre-image security).
    Sha256,
    /// SHA-512 (512-bit; ~256-bit post-quantum pre-image security).
    Sha512,
    /// SHA-512/256 (256-bit truncation of SHA-512; fast on 64-bit hosts).
    Sha512Trunc256,
    /// Iterated SHA-512: the digest is re-hashed `rounds` times in total
    /// (`rounds >= 1`), adding a tunable work factor while staying 512-bit.
    IteratedSha512 {
        /// Total number of SHA-512 applications (clamped to at least 1).
        rounds: u32,
    },
    /// Concatenation combiner `SHA-512(x) ‖ SHA-256(x)` (768-bit; secure if
    /// either component is collision-resistant).
    ConcatSha512Sha256,
}

impl Default for QuantumHashAlgorithm {
    /// The recommended quantum-safe default: the SHA-512‖SHA-256 combiner, which
    /// stays collision-resistant as long as *either* component does.
    fn default() -> Self {
        QuantumHashAlgorithm::ConcatSha512Sha256
    }
}

impl QuantumHashAlgorithm {
    /// Computes the raw digest bytes for `data`.
    #[must_use]
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            QuantumHashAlgorithm::Sha256 => sha256(data).to_vec(),
            QuantumHashAlgorithm::Sha512 => sha512(data).to_vec(),
            QuantumHashAlgorithm::Sha512Trunc256 => sha512_256(data).to_vec(),
            QuantumHashAlgorithm::IteratedSha512 { rounds } => {
                let total = (*rounds).max(1);
                let mut state = sha512(data);
                for _ in 1..total {
                    state = sha512(&state);
                }
                state.to_vec()
            }
            QuantumHashAlgorithm::ConcatSha512Sha256 => {
                let mut combined = Vec::with_capacity(super::SHA512_BYTES + super::SHA256_BYTES);
                combined.extend_from_slice(&sha512(data));
                combined.extend_from_slice(&sha256(data));
                combined
            }
        }
    }

    /// Digest size in bits.
    #[must_use]
    pub fn digest_bits(&self) -> usize {
        match self {
            QuantumHashAlgorithm::Sha256 | QuantumHashAlgorithm::Sha512Trunc256 => 256,
            QuantumHashAlgorithm::Sha512 | QuantumHashAlgorithm::IteratedSha512 { .. } => 512,
            QuantumHashAlgorithm::ConcatSha512Sha256 => 768,
        }
    }

    /// Approximate post-quantum pre-image security in bits (`digest_bits / 2`, the
    /// Grover bound). The combiner is capped at its strongest component.
    #[must_use]
    pub fn quantum_preimage_bits(&self) -> u32 {
        match self {
            QuantumHashAlgorithm::ConcatSha512Sha256 => 256,
            other => (other.digest_bits() as u32) / 2,
        }
    }

    /// Returns `true` if the algorithm offers at least 128-bit post-quantum
    /// pre-image security.
    #[must_use]
    pub fn is_quantum_resistant(&self) -> bool {
        self.quantum_preimage_bits() >= 128
    }

    /// A stable, human-readable identifier including parameters.
    #[must_use]
    pub fn canonical_id(&self) -> String {
        match self {
            QuantumHashAlgorithm::IteratedSha512 { rounds } => {
                format!("iterated-sha-512-r{}", (*rounds).max(1))
            }
            other => other.family_id().to_string(),
        }
    }

    /// The parameter-free family identifier, used as a
    /// [`super::agility::PqAlgorithmRegistry`] key.
    #[must_use]
    pub fn family_id(&self) -> &'static str {
        match self {
            QuantumHashAlgorithm::Sha256 => "sha-256",
            QuantumHashAlgorithm::Sha512 => "sha-512",
            QuantumHashAlgorithm::Sha512Trunc256 => "sha-512_256",
            QuantumHashAlgorithm::IteratedSha512 { .. } => "iterated-sha-512",
            QuantumHashAlgorithm::ConcatSha512Sha256 => "concat-sha-512+sha-256",
        }
    }
}

/// A computed content hash: an algorithm plus its lowercase-hex digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentHash {
    /// Algorithm used to produce [`ContentHash::digest`].
    pub algorithm: QuantumHashAlgorithm,
    /// Lowercase-hex digest.
    pub digest: String,
}

impl ContentHash {
    /// Computes a content hash of raw `data`.
    #[must_use]
    pub fn of_bytes(algorithm: QuantumHashAlgorithm, data: &[u8]) -> Self {
        Self {
            algorithm,
            digest: to_hex(&algorithm.digest(data)),
        }
    }

    /// Computes a content hash over a canonicalized statute entry.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn of_entry(algorithm: QuantumHashAlgorithm, entry: &StatuteEntry) -> RegistryResult<Self> {
        let bytes = canonical_json_bytes(entry)?;
        Ok(Self::of_bytes(algorithm, &bytes))
    }

    /// Re-computes the digest of `data` and compares it in constant time against
    /// the stored value.
    #[must_use]
    pub fn verify_bytes(&self, data: &[u8]) -> bool {
        let expected = to_hex(&self.algorithm.digest(data));
        constant_time_eq(self.digest.as_bytes(), expected.as_bytes())
    }

    /// Re-canonicalizes `entry` and verifies the stored digest against it.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify_entry(&self, entry: &StatuteEntry) -> RegistryResult<bool> {
        let bytes = canonical_json_bytes(entry)?;
        Ok(self.verify_bytes(&bytes))
    }

    /// Digest size in bits.
    #[must_use]
    pub fn digest_bits(&self) -> usize {
        self.algorithm.digest_bits()
    }
}

/// A per-statute digest entry inside a [`StoreHashManifest`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreLeaf {
    /// Statute identifier.
    pub statute_id: String,
    /// Version of the entry that was hashed.
    pub version: u32,
    /// Lowercase-hex content digest of the canonicalized entry.
    pub digest: String,
}

/// A Merkle-committed integrity snapshot over the whole registry store.
///
/// The leaves are the per-statute content digests sorted by statute id; the
/// [`StoreHashManifest::merkle_root`] commits to all of them. Two registries with
/// identical content produce identical manifests, and any single-record tamper
/// changes the root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreHashManifest {
    /// Content-hash algorithm used for every leaf.
    pub algorithm: QuantumHashAlgorithm,
    /// Per-statute digests, sorted by statute id for determinism.
    pub leaves: Vec<StoreLeaf>,
    /// Merkle root (hex of 32 bytes) over the leaf commitments.
    pub merkle_root: String,
    /// RFC 3339 timestamp of when the manifest was produced.
    pub created_at: String,
}

impl StoreHashManifest {
    /// Number of statutes covered by the manifest.
    #[must_use]
    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    /// Whether the manifest covers no statutes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Looks up the recorded digest for a statute id.
    #[must_use]
    pub fn digest_for(&self, statute_id: &str) -> Option<&str> {
        self.leaves
            .iter()
            .find(|leaf| leaf.statute_id == statute_id)
            .map(|leaf| leaf.digest.as_str())
    }
}

/// Computes the Merkle leaf commitment for one store entry, binding the statute
/// id and version so digests cannot be transplanted between records.
///
/// The (possibly large) content digest is folded to a uniform 32-byte value so
/// the Merkle layer always operates on fixed-width leaves.
fn store_leaf_commitment(leaf: &StoreLeaf) -> [u8; 32] {
    let folded = sha256(leaf.digest.as_bytes());
    super::tagged_hash(
        CTX_LEAF,
        &[
            leaf.statute_id.as_bytes(),
            &leaf.version.to_be_bytes(),
            &folded,
        ],
    )
}

/// Builds a [`StoreHashManifest`] over every statute in `registry`.
///
/// # Errors
///
/// Propagates canonicalization failures.
pub fn hash_registry_store(
    registry: &StatuteRegistry,
    algorithm: QuantumHashAlgorithm,
) -> RegistryResult<StoreHashManifest> {
    let mut entries: Vec<&StatuteEntry> = registry.list();
    entries.sort_by(|a, b| a.statute.id.cmp(&b.statute.id));
    let mut leaves = Vec::with_capacity(entries.len());
    let mut commitments = Vec::with_capacity(entries.len());
    for entry in entries {
        let content = ContentHash::of_entry(algorithm, entry)?;
        let leaf = StoreLeaf {
            statute_id: entry.statute.id.clone(),
            version: entry.version,
            digest: content.digest,
        };
        commitments.push(store_leaf_commitment(&leaf));
        leaves.push(leaf);
    }
    Ok(StoreHashManifest {
        algorithm,
        leaves,
        merkle_root: to_hex(&merkle_root(&commitments)),
        created_at: super::now_rfc3339(),
    })
}

/// Re-derives a manifest from the current `registry` and reports whether it still
/// matches `manifest` (same algorithm, same per-statute digests, same root).
///
/// Returns the list of statute ids whose content changed (added, removed, or
/// mutated), empty when the store is intact.
///
/// # Errors
///
/// Propagates canonicalization failures.
pub fn verify_registry_store(
    registry: &StatuteRegistry,
    manifest: &StoreHashManifest,
) -> RegistryResult<Vec<String>> {
    let current = hash_registry_store(registry, manifest.algorithm)?;
    if current.merkle_root == manifest.merkle_root {
        return Ok(Vec::new());
    }
    let mut changed = Vec::new();
    // Records present now whose digest differs from (or is absent in) the manifest.
    for leaf in &current.leaves {
        match manifest.digest_for(&leaf.statute_id) {
            Some(previous) if previous == leaf.digest => {}
            _ => changed.push(leaf.statute_id.clone()),
        }
    }
    // Records that existed in the manifest but are gone now.
    for leaf in &manifest.leaves {
        if current.digest_for(&leaf.statute_id).is_none() {
            changed.push(leaf.statute_id.clone());
        }
    }
    changed.sort();
    changed.dedup();
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn entry(id: &str, title: &str) -> StatuteEntry {
        let statute = Statute::new(id, title, Effect::new(EffectType::Grant, "grant"));
        StatuteEntry::new(statute, "US")
    }

    fn registry_with(ids: &[(&str, &str)]) -> StatuteRegistry {
        let mut registry = StatuteRegistry::new();
        for (id, title) in ids {
            registry.register(entry(id, title)).expect("register");
        }
        registry
    }

    #[test]
    fn test_algorithm_levels() {
        assert_eq!(QuantumHashAlgorithm::Sha256.digest_bits(), 256);
        assert_eq!(QuantumHashAlgorithm::Sha512.digest_bits(), 512);
        assert_eq!(QuantumHashAlgorithm::ConcatSha512Sha256.digest_bits(), 768);
        assert_eq!(QuantumHashAlgorithm::Sha512.quantum_preimage_bits(), 256);
        assert!(QuantumHashAlgorithm::Sha512.is_quantum_resistant());
        assert!(QuantumHashAlgorithm::default().is_quantum_resistant());
        assert_eq!(
            QuantumHashAlgorithm::IteratedSha512 { rounds: 4 }.canonical_id(),
            "iterated-sha-512-r4"
        );
    }

    #[test]
    fn test_iterated_depends_on_rounds_and_clamps_zero() {
        let one = QuantumHashAlgorithm::IteratedSha512 { rounds: 1 }.digest(b"x");
        let many = QuantumHashAlgorithm::IteratedSha512 { rounds: 8 }.digest(b"x");
        let zero = QuantumHashAlgorithm::IteratedSha512 { rounds: 0 }.digest(b"x");
        assert_eq!(one, sha512(b"x").to_vec());
        assert_ne!(one, many);
        assert_eq!(zero, one);
    }

    #[test]
    fn test_content_hash_of_entry_roundtrip() {
        let item = entry("act-1", "An Act");
        let hash = ContentHash::of_entry(QuantumHashAlgorithm::default(), &item).expect("hash");
        assert!(hash.verify_entry(&item).expect("verify"));
        assert_eq!(hash.digest_bits(), 768);

        // A different statute must not verify against this digest.
        let other = entry("act-2", "Another Act");
        assert!(!hash.verify_entry(&other).expect("verify other"));
    }

    #[test]
    fn test_content_hash_is_deterministic_with_metadata() {
        // Insertion order of metadata must not change the digest.
        let mut a = entry("act-1", "An Act");
        a = a.with_metadata("z", "1").with_metadata("a", "2");
        let mut b = entry("act-1", "An Act");
        b = b.with_metadata("a", "2").with_metadata("z", "1");
        // Align the volatile id/timestamp fields so we compare content only.
        b.registry_id = a.registry_id;
        b.created_at = a.created_at;
        b.modified_at = a.modified_at;
        b.etag = a.etag.clone();
        let ha = ContentHash::of_entry(QuantumHashAlgorithm::Sha512, &a).expect("a");
        let hb = ContentHash::of_entry(QuantumHashAlgorithm::Sha512, &b).expect("b");
        assert_eq!(ha, hb);
    }

    #[test]
    fn test_store_manifest_commits_to_all_records() {
        let registry = registry_with(&[("b-act", "B"), ("a-act", "A"), ("c-act", "C")]);
        let manifest =
            hash_registry_store(&registry, QuantumHashAlgorithm::default()).expect("manifest");
        assert_eq!(manifest.len(), 3);
        assert!(!manifest.is_empty());
        // Leaves are sorted by id for determinism.
        assert_eq!(manifest.leaves[0].statute_id, "a-act");
        assert_eq!(manifest.leaves[2].statute_id, "c-act");
        assert!(manifest.digest_for("b-act").is_some());
        // Recomputation is stable.
        let again = hash_registry_store(&registry, QuantumHashAlgorithm::default()).expect("again");
        assert_eq!(manifest.merkle_root, again.merkle_root);
    }

    #[test]
    fn test_verify_store_detects_changes() {
        let mut registry = registry_with(&[("a-act", "A"), ("b-act", "B")]);
        let manifest =
            hash_registry_store(&registry, QuantumHashAlgorithm::default()).expect("manifest");
        assert!(
            verify_registry_store(&registry, &manifest)
                .expect("verify")
                .is_empty()
        );

        // Mutate one record: it must be reported as changed.
        registry
            .update(
                "a-act",
                Statute::new(
                    "a-act",
                    "A (amended)",
                    Effect::new(EffectType::Grant, "grant"),
                ),
            )
            .expect("update");
        let changed = verify_registry_store(&registry, &manifest).expect("verify changed");
        assert_eq!(changed, vec!["a-act".to_string()]);

        // Add a record: also reported as changed.
        registry.register(entry("c-act", "C")).expect("register");
        let changed2 = verify_registry_store(&registry, &manifest).expect("verify added");
        assert!(changed2.contains(&"c-act".to_string()));
    }

    #[test]
    fn test_serde_roundtrip_manifest() {
        let registry = registry_with(&[("a-act", "A")]);
        let manifest = hash_registry_store(&registry, QuantumHashAlgorithm::Sha512).expect("m");
        let json = serde_json::to_string(&manifest).expect("ser");
        let back: StoreHashManifest = serde_json::from_str(&json).expect("de");
        assert_eq!(manifest, back);
    }
}
