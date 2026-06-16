//! Quantum-resistant, hash-based digital signatures.
//!
//! Hash-based signatures are the most conservative post-quantum signature
//! family: their security relies only on the (second-)preimage resistance of an
//! underlying hash, which Grover's algorithm merely halves. This module
//! implements, in pure Rust on top of [`super::pq_hash`]:
//!
//! * [`LamportKeyPair`] — the classic Lamport **one-time** signature (OTS).
//! * [`wots_keygen`]/[`wots_sign`]/[`wots_verify`] — the more compact
//!   **Winternitz** OTS (WOTS), the building block of XMSS/SPHINCS+.
//! * [`MerkleSignatureScheme`] — a **stateful many-time** signature (the Merkle
//!   Signature Scheme / XMSS construction) that authenticates many WOTS leaves
//!   under a single compact [`MerklePublicKey`] root via Merkle paths.
//!
//! One-time keys must never sign two different messages; reuse safety is the job
//! of [`super::key_management`].

use super::pq_hash::{PqHashAlgorithm, hash_pair, hash_tagged, pq_hash, prg, to_hex};
use crate::{AuditError, AuditResult};
use serde::{Deserialize, Serialize};

// Domain-separation tags.
const LAMPORT_SK_LABEL: u8 = 0x20;
const LAMPORT_PK_TAG: u8 = 0x21;
const WOTS_SK_LABEL: u8 = 0x30;
const WOTS_CHAIN_TAG: u8 = 0x31;
const WOTS_PK_TAG: u8 = 0x32;
const MSS_LEAF_SEED_LABEL: u8 = 0x40;
const MSS_LEAF_TAG: u8 = 0x41;
const MSS_NODE_TAG: u8 = 0x42;

/// Winternitz parameter (`w = 16` ⇒ 4 bits per hash chain).
const WOTS_W: u32 = 16;
/// `log2(WOTS_W)`.
const WOTS_LOG_W: usize = 4;

// ---------------------------------------------------------------------------
// Lamport one-time signatures
// ---------------------------------------------------------------------------

/// A Lamport one-time signing key (secret).
///
/// Holds `2 * digest_bits` secret values; signing a single message reveals half
/// of them, so the key must be used exactly once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LamportSecretKey {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// Secret values, laid out as `[bit0_for0, bit0_for1, bit1_for0, ...]`.
    values: Vec<Vec<u8>>,
}

/// A Lamport one-time verification key (public).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LamportPublicKey {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// Hashes of each secret value (same layout as the secret key).
    hashes: Vec<Vec<u8>>,
}

/// A Lamport one-time signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LamportSignature {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// One revealed secret value per message-digest bit.
    revealed: Vec<Vec<u8>>,
}

/// A complete Lamport key pair derived deterministically from a seed.
#[derive(Debug, Clone)]
pub struct LamportKeyPair {
    /// Secret signing key.
    pub secret: LamportSecretKey,
    /// Public verification key.
    pub public: LamportPublicKey,
}

/// Returns bit `i` (MSB-first within each byte) of `bytes`, or `0` if out of
/// range.
fn bit_at(bytes: &[u8], i: usize) -> u8 {
    match bytes.get(i / 8) {
        Some(byte) => (byte >> (7 - (i % 8))) & 1,
        None => 0,
    }
}

impl LamportKeyPair {
    /// Deterministically derives a Lamport key pair from `seed`.
    pub fn from_seed(algorithm: PqHashAlgorithm, seed: &[u8]) -> Self {
        let bits = algorithm.digest_len() * 8;
        let mut values = Vec::with_capacity(bits * 2);
        let mut hashes = Vec::with_capacity(bits * 2);
        for i in 0..(bits * 2) {
            let value = prg(algorithm, seed, LAMPORT_SK_LABEL, i as u64, 0);
            hashes.push(hash_tagged(algorithm, LAMPORT_PK_TAG, &value));
            values.push(value);
        }
        Self {
            secret: LamportSecretKey { algorithm, values },
            public: LamportPublicKey { algorithm, hashes },
        }
    }
}

impl LamportSecretKey {
    /// Produces a one-time signature over `message`.
    pub fn sign(&self, message: &[u8]) -> LamportSignature {
        let digest = pq_hash(self.algorithm, message);
        let bits = digest.len() * 8;
        let mut revealed = Vec::with_capacity(bits);
        for i in 0..bits {
            let idx = 2 * i + bit_at(&digest, i) as usize;
            revealed.push(self.values.get(idx).cloned().unwrap_or_default());
        }
        LamportSignature {
            algorithm: self.algorithm,
            revealed,
        }
    }
}

impl LamportPublicKey {
    /// Verifies `signature` over `message`.
    pub fn verify(&self, message: &[u8], signature: &LamportSignature) -> bool {
        if signature.algorithm != self.algorithm {
            return false;
        }
        let digest = pq_hash(self.algorithm, message);
        let bits = digest.len() * 8;
        if signature.revealed.len() != bits {
            return false;
        }
        for i in 0..bits {
            let idx = 2 * i + bit_at(&digest, i) as usize;
            let Some(expected) = self.hashes.get(idx) else {
                return false;
            };
            let Some(revealed) = signature.revealed.get(i) else {
                return false;
            };
            if &hash_tagged(self.algorithm, LAMPORT_PK_TAG, revealed) != expected {
                return false;
            }
        }
        true
    }

    /// Compact hex fingerprint of this public key.
    pub fn fingerprint(&self) -> String {
        let mut buf = Vec::new();
        for hash in &self.hashes {
            buf.extend_from_slice(hash);
        }
        to_hex(&pq_hash(self.algorithm, &buf))
    }
}

// ---------------------------------------------------------------------------
// Winternitz one-time signatures (WOTS)
// ---------------------------------------------------------------------------

/// A WOTS secret key (one secret per hash chain).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WotsSecretKey {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// One secret start value per chain.
    values: Vec<Vec<u8>>,
}

/// A WOTS signature (one chain value per base-`w` digit).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WotsSignature {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// Intermediate chain values.
    chains: Vec<Vec<u8>>,
}

/// Computes `(len1, len2)` — the number of message and checksum base-`w`
/// digits — for a digest of `n` bytes.
fn wots_params(n: usize) -> (usize, usize) {
    let len1 = (8 * n).div_ceil(WOTS_LOG_W);
    let max_checksum = (len1 as u32) * (WOTS_W - 1);
    let mut len2 = 1usize;
    let mut capacity = WOTS_W;
    while capacity <= max_checksum {
        capacity = capacity.saturating_mul(WOTS_W);
        len2 += 1;
    }
    (len1, len2)
}

/// Iterates the hash chain `steps` times starting from `start`.
fn wots_chain(algorithm: PqHashAlgorithm, start: Vec<u8>, steps: u8) -> Vec<u8> {
    let mut value = start;
    for _ in 0..steps {
        value = hash_tagged(algorithm, WOTS_CHAIN_TAG, &value);
    }
    value
}

/// Extracts the first `out_len` base-`w` (4-bit) digits of `digest`.
fn base_w_digits(digest: &[u8], out_len: usize) -> Vec<u8> {
    let mut digits = Vec::with_capacity(out_len);
    for &byte in digest {
        if digits.len() >= out_len {
            break;
        }
        digits.push(byte >> 4);
        if digits.len() >= out_len {
            break;
        }
        digits.push(byte & 0x0f);
    }
    digits.truncate(out_len);
    digits
}

/// Computes the `len2` big-endian base-`w` checksum digits for `msg_digits`.
fn checksum_digits(msg_digits: &[u8], len2: usize) -> Vec<u8> {
    let csum: u32 = msg_digits.iter().map(|&d| (WOTS_W - 1) - d as u32).sum();
    (0..len2)
        .map(|j| {
            let shift = WOTS_LOG_W * (len2 - 1 - j);
            ((csum >> shift) & (WOTS_W - 1)) as u8
        })
        .collect()
}

/// Builds the full ordered digit vector (message digits ++ checksum digits).
fn wots_digits(algorithm: PqHashAlgorithm, message: &[u8]) -> Vec<u8> {
    let n = algorithm.digest_len();
    let (len1, len2) = wots_params(n);
    let digest = pq_hash(algorithm, message);
    let mut digits = base_w_digits(&digest, len1);
    let checksum = checksum_digits(&digits, len2);
    digits.extend_from_slice(&checksum);
    digits
}

/// Compresses WOTS chain endpoints into a single compact public key.
fn wots_compress(algorithm: PqHashAlgorithm, endpoints: &[Vec<u8>]) -> Vec<u8> {
    let mut buf = vec![WOTS_PK_TAG];
    for endpoint in endpoints {
        buf.extend_from_slice(endpoint);
    }
    pq_hash(algorithm, &buf)
}

/// Deterministically derives a WOTS key pair from `seed`, returning the secret
/// key and the compact public key.
pub fn wots_keygen(algorithm: PqHashAlgorithm, seed: &[u8]) -> (WotsSecretKey, Vec<u8>) {
    let n = algorithm.digest_len();
    let (len1, len2) = wots_params(n);
    let len = len1 + len2;
    let mut values = Vec::with_capacity(len);
    let mut endpoints = Vec::with_capacity(len);
    let full = (WOTS_W - 1) as u8;
    for i in 0..len {
        let sk_value = prg(algorithm, seed, WOTS_SK_LABEL, i as u64, 0);
        endpoints.push(wots_chain(algorithm, sk_value.clone(), full));
        values.push(sk_value);
    }
    let public = wots_compress(algorithm, &endpoints);
    (WotsSecretKey { algorithm, values }, public)
}

/// Signs `message` with a WOTS secret key.
pub fn wots_sign(secret: &WotsSecretKey, message: &[u8]) -> WotsSignature {
    let digits = wots_digits(secret.algorithm, message);
    let chains = secret
        .values
        .iter()
        .zip(digits.iter())
        .map(|(start, &digit)| wots_chain(secret.algorithm, start.clone(), digit))
        .collect();
    WotsSignature {
        algorithm: secret.algorithm,
        chains,
    }
}

/// Recovers the compact WOTS public key implied by a signature over `message`,
/// or `None` if the signature is structurally invalid.
pub fn wots_recover_public(
    algorithm: PqHashAlgorithm,
    message: &[u8],
    signature: &WotsSignature,
) -> Option<Vec<u8>> {
    if signature.algorithm != algorithm {
        return None;
    }
    let digits = wots_digits(algorithm, message);
    if signature.chains.len() != digits.len() {
        return None;
    }
    let full = (WOTS_W - 1) as u8;
    let endpoints: Vec<Vec<u8>> = signature
        .chains
        .iter()
        .zip(digits.iter())
        .map(|(chain, &digit)| wots_chain(algorithm, chain.clone(), full - digit))
        .collect();
    Some(wots_compress(algorithm, &endpoints))
}

/// Verifies a WOTS signature against a known compact public key.
pub fn wots_verify(
    public: &[u8],
    algorithm: PqHashAlgorithm,
    message: &[u8],
    signature: &WotsSignature,
) -> bool {
    matches!(wots_recover_public(algorithm, message, signature), Some(pk) if pk.as_slice() == public)
}

// ---------------------------------------------------------------------------
// Merkle signature scheme (XMSS-style many-time signatures)
// ---------------------------------------------------------------------------

/// Maximum supported Merkle tree height (`2^height` one-time leaves).
pub const MAX_MERKLE_HEIGHT: u32 = 16;

/// The compact public key of a [`MerkleSignatureScheme`]: just the algorithm,
/// the tree height and the Merkle root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerklePublicKey {
    /// Hash algorithm.
    pub algorithm: PqHashAlgorithm,
    /// Tree height (`2^height` leaves / available one-time signatures).
    pub height: u32,
    /// Merkle root that authenticates all WOTS leaves.
    pub root: Vec<u8>,
}

impl MerklePublicKey {
    /// Returns the root as a hex string.
    pub fn root_hex(&self) -> String {
        to_hex(&self.root)
    }

    /// Verifies a [`MerkleSignature`] over `message`.
    pub fn verify(&self, message: &[u8], signature: &MerkleSignature) -> bool {
        if signature.auth_path.len() != self.height as usize {
            return false;
        }
        let Some(leaf_public) = wots_recover_public(self.algorithm, message, &signature.wots)
        else {
            return false;
        };
        let mut node = hash_tagged(self.algorithm, MSS_LEAF_TAG, &leaf_public);
        let mut index = signature.leaf_index;
        for sibling in &signature.auth_path {
            node = if index & 1 == 0 {
                hash_pair(self.algorithm, MSS_NODE_TAG, &node, sibling)
            } else {
                hash_pair(self.algorithm, MSS_NODE_TAG, sibling, &node)
            };
            index >>= 1;
        }
        node == self.root
    }
}

/// A many-time Merkle signature: a WOTS signature on one leaf plus the Merkle
/// authentication path linking that leaf to the root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleSignature {
    /// Index of the one-time leaf used.
    pub leaf_index: u64,
    /// The leaf's WOTS signature.
    pub wots: WotsSignature,
    /// Sibling hashes from the leaf up to (but excluding) the root.
    pub auth_path: Vec<Vec<u8>>,
}

/// A stateful many-time signature scheme: a Merkle tree whose `2^height` leaves
/// are WOTS public keys, all derived deterministically from a single seed.
#[derive(Debug, Clone)]
pub struct MerkleSignatureScheme {
    algorithm: PqHashAlgorithm,
    height: u32,
    seed: Vec<u8>,
    /// Tree levels, `levels[0]` = leaves, `levels[height]` = `[root]`.
    levels: Vec<Vec<Vec<u8>>>,
}

impl MerkleSignatureScheme {
    /// Builds a scheme of the given `height` deterministically from `seed`.
    pub fn from_seed(algorithm: PqHashAlgorithm, height: u32, seed: &[u8]) -> AuditResult<Self> {
        if height == 0 || height > MAX_MERKLE_HEIGHT {
            return Err(AuditError::InvalidRecord(format!(
                "merkle height must be in 1..={MAX_MERKLE_HEIGHT}, got {height}"
            )));
        }
        let num_leaves = 1usize << height;
        let mut leaves = Vec::with_capacity(num_leaves);
        for leaf in 0..num_leaves {
            let leaf_seed = prg(algorithm, seed, MSS_LEAF_SEED_LABEL, leaf as u64, 0);
            let (_secret, public) = wots_keygen(algorithm, &leaf_seed);
            leaves.push(hash_tagged(algorithm, MSS_LEAF_TAG, &public));
        }

        let mut levels = vec![leaves];
        while levels.last().map(Vec::len).unwrap_or(0) > 1 {
            let current = levels.last().cloned().unwrap_or_default();
            let mut next = Vec::with_capacity(current.len().div_ceil(2));
            for pair in current.chunks(2) {
                let left = pair.first().cloned().unwrap_or_default();
                let right = pair.get(1).cloned().unwrap_or_else(|| left.clone());
                next.push(hash_pair(algorithm, MSS_NODE_TAG, &left, &right));
            }
            levels.push(next);
        }

        Ok(Self {
            algorithm,
            height,
            seed: seed.to_vec(),
            levels,
        })
    }

    /// Number of one-time leaves (`2^height`).
    pub fn num_leaves(&self) -> u64 {
        1u64 << self.height
    }

    /// Tree height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Hash algorithm.
    pub fn algorithm(&self) -> PqHashAlgorithm {
        self.algorithm
    }

    /// Returns the compact public key (algorithm, height, root).
    pub fn public_key(&self) -> MerklePublicKey {
        let root = self
            .levels
            .last()
            .and_then(|level| level.first())
            .cloned()
            .unwrap_or_default();
        MerklePublicKey {
            algorithm: self.algorithm,
            height: self.height,
            root,
        }
    }

    /// Signs `message` using the one-time leaf at `leaf_index`.
    ///
    /// The caller is responsible for never reusing a leaf index (see
    /// [`super::key_management`]).
    pub fn sign(&self, message: &[u8], leaf_index: u64) -> AuditResult<MerkleSignature> {
        if leaf_index >= self.num_leaves() {
            return Err(AuditError::InvalidRecord(format!(
                "leaf index {leaf_index} out of range (capacity {})",
                self.num_leaves()
            )));
        }
        let leaf_seed = prg(
            self.algorithm,
            &self.seed,
            MSS_LEAF_SEED_LABEL,
            leaf_index,
            0,
        );
        let (secret, _public) = wots_keygen(self.algorithm, &leaf_seed);
        let wots = wots_sign(&secret, message);

        let mut auth_path = Vec::with_capacity(self.height as usize);
        let mut index = leaf_index as usize;
        for level in 0..self.height as usize {
            let sibling_index = index ^ 1;
            let sibling = self
                .levels
                .get(level)
                .and_then(|nodes| nodes.get(sibling_index))
                .cloned()
                .unwrap_or_default();
            auth_path.push(sibling);
            index >>= 1;
        }

        Ok(MerkleSignature {
            leaf_index,
            wots,
            auth_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lamport_sign_verify_roundtrip() {
        let kp = LamportKeyPair::from_seed(PqHashAlgorithm::Sha256, b"lamport-seed");
        let sig = kp.secret.sign(b"hello world");
        assert!(kp.public.verify(b"hello world", &sig));
        assert!(!kp.public.fingerprint().is_empty());
    }

    #[test]
    fn test_lamport_rejects_wrong_message() {
        let kp = LamportKeyPair::from_seed(PqHashAlgorithm::Sha256, b"seed-a");
        let sig = kp.secret.sign(b"message-one");
        assert!(!kp.public.verify(b"message-two", &sig));
    }

    #[test]
    fn test_lamport_rejects_tampered_signature() {
        let kp = LamportKeyPair::from_seed(PqHashAlgorithm::Sha512, b"seed-b");
        let mut sig = kp.secret.sign(b"payload");
        if let Some(first) = sig.revealed.get_mut(0) {
            first.clear();
        }
        assert!(!kp.public.verify(b"payload", &sig));
    }

    #[test]
    fn test_lamport_deterministic_from_seed() {
        let a = LamportKeyPair::from_seed(PqHashAlgorithm::Sha256, b"same");
        let b = LamportKeyPair::from_seed(PqHashAlgorithm::Sha256, b"same");
        assert_eq!(a.public, b.public);
    }

    #[test]
    fn test_wots_params_known_values() {
        assert_eq!(wots_params(32), (64, 3));
        assert_eq!(wots_params(64), (128, 3));
    }

    #[test]
    fn test_wots_sign_verify_roundtrip() {
        let (sk, pk) = wots_keygen(PqHashAlgorithm::Sha256, b"wots-seed");
        let sig = wots_sign(&sk, b"a winternitz message");
        assert!(wots_verify(
            &pk,
            PqHashAlgorithm::Sha256,
            b"a winternitz message",
            &sig
        ));
    }

    #[test]
    fn test_wots_rejects_wrong_message() {
        let (sk, pk) = wots_keygen(PqHashAlgorithm::Sha256, b"wots-seed-2");
        let sig = wots_sign(&sk, b"original");
        assert!(!wots_verify(&pk, PqHashAlgorithm::Sha256, b"forged", &sig));
    }

    #[test]
    fn test_wots_recover_public_matches_keygen() {
        let (sk, pk) = wots_keygen(PqHashAlgorithm::Sha512, b"wots-seed-3");
        let sig = wots_sign(&sk, b"recover me");
        let recovered = wots_recover_public(PqHashAlgorithm::Sha512, b"recover me", &sig);
        assert_eq!(recovered, Some(pk));
    }

    #[test]
    fn test_merkle_scheme_sign_verify_all_leaves() {
        let scheme = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 3, b"mss-seed")
            .expect("scheme builds");
        let public = scheme.public_key();
        assert_eq!(scheme.num_leaves(), 8);
        for leaf in 0..scheme.num_leaves() {
            let message = format!("message for leaf {leaf}");
            let sig = scheme.sign(message.as_bytes(), leaf).expect("sign");
            assert!(public.verify(message.as_bytes(), &sig));
        }
    }

    #[test]
    fn test_merkle_rejects_wrong_message() {
        let scheme = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 2, b"mss-seed-2")
            .expect("scheme builds");
        let public = scheme.public_key();
        let sig = scheme.sign(b"correct", 1).expect("sign");
        assert!(public.verify(b"correct", &sig));
        assert!(!public.verify(b"wrong", &sig));
    }

    #[test]
    fn test_merkle_rejects_tampered_auth_path() {
        let scheme = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 3, b"mss-seed-3")
            .expect("scheme builds");
        let public = scheme.public_key();
        let mut sig = scheme.sign(b"data", 2).expect("sign");
        if let Some(node) = sig.auth_path.get_mut(0) {
            node.clear();
        }
        assert!(!public.verify(b"data", &sig));
    }

    #[test]
    fn test_merkle_rejects_foreign_root() {
        let a = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 2, b"key-a")
            .expect("scheme builds");
        let b = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 2, b"key-b")
            .expect("scheme builds");
        let sig = a.sign(b"x", 0).expect("sign");
        // Signature from key A must not verify under key B's public root.
        assert!(!b.public_key().verify(b"x", &sig));
        assert!(a.public_key().verify(b"x", &sig));
    }

    #[test]
    fn test_merkle_height_bounds() {
        assert!(MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 0, b"s").is_err());
        assert!(
            MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, MAX_MERKLE_HEIGHT + 1, b"s")
                .is_err()
        );
        assert!(MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 1, b"s").is_ok());
    }

    #[test]
    fn test_merkle_sign_out_of_range_leaf() {
        let scheme = MerkleSignatureScheme::from_seed(PqHashAlgorithm::Sha256, 2, b"s")
            .expect("scheme builds");
        assert!(scheme.sign(b"x", 4).is_err());
        assert!(scheme.sign(b"x", 3).is_ok());
    }
}
