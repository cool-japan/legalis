//! Quantum-Safe Registry (v0.3.4): post-quantum cryptography for statute records.
//!
//! This module hardens the registry against a future with cryptographically
//! relevant quantum computers. It is fully pure-Rust and `scirs2`-free, reusing
//! only the workspace's audited [`sha2`] and [`hex`] crates for the underlying
//! hash primitives. Concretely it provides:
//!
//! - **Quantum-resistant content hashing** ([`hashing`]): large-output digests
//!   ([`QuantumHashAlgorithm`]) over canonicalized [`StatuteEntry`] records and a
//!   Merkle commitment over the whole registry store ([`StoreHashManifest`]).
//!   Grover's algorithm only halves pre-image security, so a 512-bit digest still
//!   offers ~256-bit post-quantum pre-image resistance.
//! - **Post-quantum signatures** ([`hash_sig`], [`signatures`]): a self-contained
//!   hash-based Lamport one-time signature lifted to a many-time
//!   [`MerkleSigner`] (XMSS-style), used to sign individual statute entries and
//!   specific versions ([`SignedStatute`]). Hash-based schemes rely only on the
//!   pre-image / collision resistance of a hash, which Shor's algorithm does
//!   **not** break.
//! - **Cryptographic agility** ([`agility`]): a [`PqAlgorithmRegistry`] of hash,
//!   signature and KEM algorithms with classical/quantum security levels and a
//!   life-cycle status, so the scheme protecting a record is pluggable and
//!   upgradeable.
//! - **Hybrid classical+PQ envelopes** ([`hybrid`]): a [`HybridSignatureEnvelope`]
//!   binding a classical symmetric MAC (HMAC-SHA-256) to the post-quantum
//!   hash-based signature, with a configurable acceptance [`HybridPolicy`] for
//!   defence-in-depth during the migration period.
//! - **QKD key-agreement model** ([`qkd`]): a deterministic BB84-style
//!   [`Bb84Session`] simulation expressed purely as data structures (bases,
//!   sifting, QBER estimation, intercept-resend eavesdropping), able to derive
//!   shared key material that keys the hybrid envelope's classical layer.
//! - **Quantum-safe audit-trail verification** ([`audit`]): a tamper-evident
//!   hash-chain plus Merkle commitment over the registry's [`RegistryEvent`] log
//!   ([`QuantumAuditTrail`]), optionally signed with the post-quantum signer
//!   ([`SignedAuditTrail`]).
//!
//! # Deferred: standardized lattice schemes
//!
//! True lattice schemes (ML-DSA / ML-KEM, FIPS 203/204) and stateless hash-based
//! SLH-DSA (FIPS 205) are intentionally **deferred**: bundling them would pull in
//! heavy, non-pure-Rust dependencies, violating the workspace's pure-Rust policy.
//! They are catalogued in the [`PqAlgorithmRegistry`] with
//! [`agility::AlgorithmStatus::Planned`]; the implemented post-quantum option is
//! the hash-based scheme in [`hash_sig`], which is fully realized here.
//!
//! All hashing is domain-separated (see [`DOMAIN_SEP`]) to avoid cross-protocol
//! collisions, and every algorithm is deterministic, so signed and hashed
//! artifacts are byte-for-byte reproducible (signing keys are derived from a
//! caller-supplied seed rather than ambient randomness, keeping the crate
//! `rand`-free).
//!
//! # Example
//!
//! ```
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_registry::quantum_safe::{HybridPolicy, QuantumSafeRegistry};
//! use legalis_registry::{StatuteEntry, StatuteRegistry};
//!
//! let mut registry = StatuteRegistry::new();
//! let statute = Statute::new("act-1", "An Act", Effect::new(EffectType::Grant, "grant"));
//! registry.register(StatuteEntry::new(statute, "US")).expect("register");
//! let entry = registry.get_uncached("act-1").expect("entry");
//!
//! // Build a quantum-safe facade keyed from a deterministic seed.
//! let mut qsr = QuantumSafeRegistry::new([7u8; 32], 4).expect("signer");
//!
//! // Post-quantum sign the statute and verify it.
//! let signed = qsr.sign_entry(0, entry).expect("sign");
//! assert!(signed.verify(entry).expect("verify"));
//!
//! // Hybrid (classical MAC + PQ signature) envelope with defence-in-depth.
//! let envelope = qsr
//!     .sign_entry_hybrid(1, entry, HybridPolicy::RequireBoth)
//!     .expect("hybrid sign");
//! let verification = qsr.verify_hybrid(&envelope, entry).expect("hybrid verify");
//! assert!(verification.accepted);
//! ```

pub mod agility;
pub mod audit;
pub mod hash_sig;
pub mod hashing;
pub mod hybrid;
pub mod qkd;
pub mod signatures;

pub use agility::{
    AlgorithmDescriptor, AlgorithmKind, AlgorithmStatus, CryptoSuite, PqAlgorithmRegistry,
    SignatureScheme,
};
pub use audit::{AuditChainLink, AuditVerification, QuantumAuditTrail, SignedAuditTrail};
pub use hash_sig::{
    LamportKeyPair, LamportSignature, MAX_MERKLE_HEIGHT, MerklePublicKey, MerkleSignature,
    MerkleSigner, OTS_BITS, lamport_verify, merkle_verify, seed_from_bytes,
};
pub use hashing::{
    ContentHash, QuantumHashAlgorithm, StoreHashManifest, hash_registry_store,
    verify_registry_store,
};
pub use hybrid::{HybridPolicy, HybridSignatureEnvelope, HybridVerification};
pub use qkd::{Basis, Bb84Config, Bb84Session, EavesdropAssessment};
pub use signatures::{SignedStatute, StatuteSigner};

use sha2::{Digest, Sha256, Sha512, Sha512_256};

use crate::{RegistryError, RegistryResult, StatuteEntry};

/// Domain-separation tag prefixed (with a context label) to all hashing in this
/// module, preventing cross-protocol collisions with other Legalis subsystems.
pub const DOMAIN_SEP: &[u8] = b"legalis.registry.quantum-safe/v1";

/// Length, in bytes, of a SHA-256 digest.
pub const SHA256_BYTES: usize = 32;

/// Length, in bytes, of a SHA-512 digest.
pub const SHA512_BYTES: usize = 64;

/// SHA-256 (and SHA-512/256) block size in bytes, used by HMAC.
const HMAC_BLOCK_BYTES: usize = 64;

const CTX_MERKLE_EMPTY: &[u8] = b"content-merkle-empty";
const CTX_MERKLE_NODE: &[u8] = b"content-merkle-node";

/// Computes the 32-byte SHA-256 digest of `data`.
#[must_use]
pub fn sha256(data: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(hasher.finalize().as_ref());
    bytes
}

/// Computes the 64-byte SHA-512 digest of `data`.
#[must_use]
pub fn sha512(data: &[u8]) -> [u8; SHA512_BYTES] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let mut bytes = [0u8; SHA512_BYTES];
    bytes.copy_from_slice(hasher.finalize().as_ref());
    bytes
}

/// Computes the 32-byte SHA-512/256 digest of `data` (a truncated SHA-512 with a
/// distinct initialization vector; fast on 64-bit hardware).
#[must_use]
pub fn sha512_256(data: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha512_256::new();
    hasher.update(data);
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(hasher.finalize().as_ref());
    bytes
}

/// Computes a domain-separated SHA-256 over a context label and one or more byte
/// segments. Each segment is length-prefixed (8-byte big-endian) so the hash is
/// unambiguous regardless of segment boundaries.
#[must_use]
pub fn tagged_hash(context: &[u8], segments: &[&[u8]]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update((DOMAIN_SEP.len() as u64).to_be_bytes());
    hasher.update(DOMAIN_SEP);
    hasher.update((context.len() as u64).to_be_bytes());
    hasher.update(context);
    for segment in segments {
        hasher.update((segment.len() as u64).to_be_bytes());
        hasher.update(segment);
    }
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(hasher.finalize().as_ref());
    bytes
}

/// Encodes bytes as a lowercase hex string.
#[must_use]
pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decodes a hex string into a fixed-size `[u8; N]`, erroring on invalid digits
/// or the wrong length.
///
/// # Errors
///
/// Returns [`RegistryError::InvalidOperation`] if `text` is not valid hex or does
/// not decode to exactly `N` bytes.
pub fn from_hex_array<const N: usize>(text: &str) -> RegistryResult<[u8; N]> {
    let bytes = hex::decode(text.trim())
        .map_err(|err| RegistryError::InvalidOperation(format!("invalid hex string: {err}")))?;
    if bytes.len() != N {
        return Err(RegistryError::InvalidOperation(format!(
            "expected {} hex bytes, found {}",
            N,
            bytes.len()
        )));
    }
    let mut array = [0u8; N];
    array.copy_from_slice(&bytes);
    Ok(array)
}

/// Constant-time byte-slice equality, returning `false` for length mismatches.
///
/// Used for digest and signature comparison so verification does not leak the
/// position of the first differing byte through timing.
#[must_use]
pub fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0u8;
    for (a, b) in left.iter().zip(right.iter()) {
        difference |= a ^ b;
    }
    difference == 0
}

/// Computes an HMAC-SHA-256 tag (RFC 2104) over `message` keyed by `key`.
///
/// This is the *classical*, symmetric authentication primitive used by the
/// [`hybrid`] envelopes. It is implemented from the SHA-256 primitive directly to
/// avoid pulling in a new dependency.
#[must_use]
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; SHA256_BYTES] {
    let mut key_block = [0u8; HMAC_BLOCK_BYTES];
    if key.len() > HMAC_BLOCK_BYTES {
        let hashed = sha256(key);
        key_block[..SHA256_BYTES].copy_from_slice(&hashed);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; HMAC_BLOCK_BYTES];
    let mut opad = [0x5cu8; HMAC_BLOCK_BYTES];
    for ((inner_byte, outer_byte), key_byte) in
        ipad.iter_mut().zip(opad.iter_mut()).zip(key_block.iter())
    {
        *inner_byte ^= *key_byte;
        *outer_byte ^= *key_byte;
    }
    let mut inner = Sha256::new();
    inner.update(ipad);
    inner.update(message);
    let inner_digest = inner.finalize();
    let mut outer = Sha256::new();
    outer.update(opad);
    outer.update(inner_digest);
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(outer.finalize().as_ref());
    bytes
}

/// Computes a binary Merkle root over a slice of 32-byte leaves.
///
/// Empty input yields a fixed domain-separated constant; a single leaf is its own
/// root; odd levels duplicate the final node. Internal nodes are domain-separated
/// from leaves to prevent second-pre-image attacks.
#[must_use]
pub fn merkle_root(leaves: &[[u8; SHA256_BYTES]]) -> [u8; SHA256_BYTES] {
    if leaves.is_empty() {
        return tagged_hash(CTX_MERKLE_EMPTY, &[]);
    }
    let mut level: Vec<[u8; SHA256_BYTES]> = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut index = 0;
        while index < level.len() {
            let left = level[index];
            let right = if index + 1 < level.len() {
                level[index + 1]
            } else {
                left
            };
            next.push(tagged_hash(CTX_MERKLE_NODE, &[&left, &right]));
            index += 2;
        }
        level = next;
    }
    level[0]
}

/// Produces deterministic, canonical JSON bytes for any [`serde::Serialize`]
/// value by recursively sorting object keys.
///
/// Determinism is essential for content hashing and signing: two structurally
/// equal records must hash identically regardless of map iteration order or the
/// `serde_json` `preserve_order` feature.
///
/// # Errors
///
/// Returns [`RegistryError::InvalidOperation`] if the value cannot be serialized.
pub fn canonical_json_bytes<T: serde::Serialize>(value: &T) -> RegistryResult<Vec<u8>> {
    let raw = serde_json::to_value(value).map_err(|err| {
        RegistryError::InvalidOperation(format!("failed to serialize for hashing: {err}"))
    })?;
    let canonical = canonicalize_value(&raw);
    serde_json::to_vec(&canonical).map_err(|err| {
        RegistryError::InvalidOperation(format!("failed to canonicalize for hashing: {err}"))
    })
}

fn canonicalize_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(canonicalize_value).collect())
        }
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut sorted = serde_json::Map::new();
            for key in keys {
                if let Some(child) = map.get(key) {
                    sorted.insert(key.clone(), canonicalize_value(child));
                }
            }
            serde_json::Value::Object(sorted)
        }
        other => other.clone(),
    }
}

/// Returns the current UTC time as an RFC 3339 timestamp.
#[must_use]
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// A one-stop facade tying the quantum-safe primitives to a concrete registry.
///
/// The facade owns a post-quantum [`StatuteSigner`], a classical HMAC key, a
/// chosen [`QuantumHashAlgorithm`] and a populated [`PqAlgorithmRegistry`]. It is
/// stateful (the signer enforces one-time use of each Merkle leaf), so signing
/// methods take `&mut self`. The signing seed and classical key are secret and
/// therefore never serialized.
#[derive(Debug, Clone)]
pub struct QuantumSafeRegistry {
    algorithm: QuantumHashAlgorithm,
    algorithms: PqAlgorithmRegistry,
    signer: StatuteSigner,
    classical_key: [u8; SHA256_BYTES],
}

impl QuantumSafeRegistry {
    /// Builds a facade from a 32-byte signing `seed` and a Merkle tree `height`
    /// (`2^height` one-time signatures), using the recommended quantum-safe
    /// content-hash algorithm.
    ///
    /// The classical HMAC key is derived deterministically from the seed so a
    /// single seed bootstraps both layers.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if `height` is out of range.
    pub fn new(seed: [u8; SHA256_BYTES], height: u8) -> RegistryResult<Self> {
        Self::with_algorithm(seed, height, QuantumHashAlgorithm::default())
    }

    /// Builds a facade with an explicit content-hash `algorithm`.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if `height` is out of range.
    pub fn with_algorithm(
        seed: [u8; SHA256_BYTES],
        height: u8,
        algorithm: QuantumHashAlgorithm,
    ) -> RegistryResult<Self> {
        let signer = StatuteSigner::from_seed(seed, height)?;
        let classical_key = tagged_hash(b"hybrid-classical-key", &[&seed]);
        Ok(Self {
            algorithm,
            algorithms: PqAlgorithmRegistry::with_defaults(),
            signer,
            classical_key,
        })
    }

    /// The content-hash algorithm in effect.
    #[must_use]
    pub fn algorithm(&self) -> QuantumHashAlgorithm {
        self.algorithm
    }

    /// Installs the symmetric key used by the hybrid envelope's classical layer.
    ///
    /// This is how a [`Bb84Session::derive_key_material`] result is wired into the
    /// hybrid signatures: the information-theoretically agreed QKD key replaces
    /// the seed-derived default classical key.
    pub fn set_classical_key(&mut self, classical_key: [u8; SHA256_BYTES]) {
        self.classical_key = classical_key;
    }

    /// Adopts the key material from a (non-aborted) BB84 session for the hybrid
    /// classical layer.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError::InvalidOperation`] if the session was aborted or
    /// produced no usable key material.
    pub fn adopt_qkd_key(&mut self, session: &Bb84Session) -> RegistryResult<()> {
        let key = session.derive_key_material().ok_or_else(|| {
            RegistryError::InvalidOperation(
                "BB84 session produced no usable key material (aborted or empty)".to_string(),
            )
        })?;
        self.classical_key = key;
        Ok(())
    }

    /// The long-lived post-quantum public key.
    #[must_use]
    pub fn public_key(&self) -> MerklePublicKey {
        self.signer.public_key()
    }

    /// The number of unused one-time signing leaves remaining.
    #[must_use]
    pub fn remaining_signatures(&self) -> u32 {
        self.signer.remaining()
    }

    /// The catalogue of post-quantum algorithms and their life-cycle status.
    #[must_use]
    pub fn algorithm_registry(&self) -> &PqAlgorithmRegistry {
        &self.algorithms
    }

    /// Computes a quantum-resistant content hash of a statute entry.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn content_hash(&self, entry: &StatuteEntry) -> RegistryResult<ContentHash> {
        ContentHash::of_entry(self.algorithm, entry)
    }

    /// Computes a Merkle-committed content-hash manifest over the whole store.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn hash_store(
        &self,
        registry: &crate::StatuteRegistry,
    ) -> RegistryResult<StoreHashManifest> {
        hash_registry_store(registry, self.algorithm)
    }

    /// Post-quantum signs a statute entry with one-time leaf `leaf_index`.
    ///
    /// # Errors
    ///
    /// Propagates signer errors (out-of-range or reused leaf) and
    /// canonicalization failures.
    pub fn sign_entry(
        &mut self,
        leaf_index: u32,
        entry: &StatuteEntry,
    ) -> RegistryResult<SignedStatute> {
        self.signer.sign_entry(leaf_index, entry)
    }

    /// Produces a hybrid classical+PQ envelope over a statute entry.
    ///
    /// # Errors
    ///
    /// Propagates signer and canonicalization failures.
    pub fn sign_entry_hybrid(
        &mut self,
        leaf_index: u32,
        entry: &StatuteEntry,
        policy: HybridPolicy,
    ) -> RegistryResult<HybridSignatureEnvelope> {
        let signed = self.signer.sign_entry(leaf_index, entry)?;
        Ok(HybridSignatureEnvelope::seal(
            signed,
            &self.classical_key,
            policy,
        ))
    }

    /// Verifies a hybrid envelope against an entry under its embedded policy.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify_hybrid(
        &self,
        envelope: &HybridSignatureEnvelope,
        entry: &StatuteEntry,
    ) -> RegistryResult<HybridVerification> {
        envelope.verify(entry, &self.classical_key)
    }

    /// Builds a tamper-evident, quantum-safe audit trail over a registry's event
    /// log.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn audit_trail(
        &self,
        registry: &crate::StatuteRegistry,
    ) -> RegistryResult<QuantumAuditTrail> {
        QuantumAuditTrail::from_registry(registry, self.algorithm)
    }

    /// Builds and post-quantum signs an audit trail over a registry's event log,
    /// consuming one-time leaf `leaf_index`.
    ///
    /// # Errors
    ///
    /// Propagates signer and canonicalization failures.
    pub fn sign_audit_trail(
        &mut self,
        leaf_index: u32,
        registry: &crate::StatuteRegistry,
    ) -> RegistryResult<SignedAuditTrail> {
        let trail = QuantumAuditTrail::from_registry(registry, self.algorithm)?;
        trail.sign(self.signer.merkle_signer_mut(), leaf_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha_known_vectors() {
        assert_eq!(
            to_hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            to_hex(&sha512(b"abc")),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
        assert_ne!(sha512_256(b"abc"), sha256(b"abc"));
    }

    #[test]
    fn test_tagged_hash_is_unambiguous() {
        let a = tagged_hash(b"ctx", &[b"ab", b"c"]);
        let b = tagged_hash(b"ctx", &[b"a", b"bc"]);
        let c = tagged_hash(b"other", &[b"ab", b"c"]);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_eq!(a, tagged_hash(b"ctx", &[b"ab", b"c"]));
    }

    #[test]
    fn test_hex_roundtrip_and_errors() {
        let bytes = [0xde, 0xad, 0xbe, 0xef];
        let hex = to_hex(&bytes);
        assert_eq!(hex, "deadbeef");
        let array = from_hex_array::<4>(&hex).expect("decode");
        assert_eq!(array, bytes);
        assert!(from_hex_array::<4>("zz").is_err());
        assert!(from_hex_array::<8>("deadbeef").is_err());
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn test_hmac_sha256_rfc4231_vector() {
        // RFC 4231 test case 1: key = 0x0b * 20, data = "Hi There".
        let key = [0x0bu8; 20];
        let tag = hmac_sha256(&key, b"Hi There");
        assert_eq!(
            to_hex(&tag),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
        // Long key (> block size) path is exercised without panicking.
        let long_key = [0x42u8; 100];
        assert_eq!(hmac_sha256(&long_key, b"x"), hmac_sha256(&long_key, b"x"));
        assert_ne!(hmac_sha256(&long_key, b"x"), hmac_sha256(&long_key, b"y"));
    }

    #[test]
    fn test_merkle_root_properties() {
        let leaf_a = sha256(b"a");
        let leaf_b = sha256(b"b");
        let leaf_c = sha256(b"c");
        // Empty is a fixed constant; single leaf is its own root.
        assert_eq!(merkle_root(&[]), tagged_hash(CTX_MERKLE_EMPTY, &[]));
        assert_eq!(merkle_root(&[leaf_a]), leaf_a);
        // Order matters and odd levels duplicate the last node deterministically.
        assert_ne!(
            merkle_root(&[leaf_a, leaf_b]),
            merkle_root(&[leaf_b, leaf_a])
        );
        assert_eq!(
            merkle_root(&[leaf_a, leaf_b, leaf_c]),
            merkle_root(&[leaf_a, leaf_b, leaf_c])
        );
    }

    #[test]
    fn test_canonical_json_is_order_independent() {
        let mut first = serde_json::Map::new();
        first.insert("b".to_string(), serde_json::json!(1));
        first.insert("a".to_string(), serde_json::json!(2));
        let mut second = serde_json::Map::new();
        second.insert("a".to_string(), serde_json::json!(2));
        second.insert("b".to_string(), serde_json::json!(1));
        let left = canonical_json_bytes(&serde_json::Value::Object(first)).expect("left");
        let right = canonical_json_bytes(&serde_json::Value::Object(second)).expect("right");
        assert_eq!(left, right);
    }
}
