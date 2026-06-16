//! Hash-based digital signatures (Lamport one-time + Merkle many-time).
//!
//! Hash-based signatures rely only on the second-pre-image / collision
//! resistance of a hash function, properties that (unlike RSA and elliptic
//! curves) are **not** broken by Shor's algorithm. They are the most
//! conservative family of post-quantum signatures.
//!
//! This module implements, from scratch and self-contained:
//!
//! 1. **Lamport one-time signature (OTS)** over a 256-bit message digest. The
//!    secret key is derived deterministically from a 32-byte seed via a
//!    domain-separated PRF (so the crate needs no `rand` dependency), and the
//!    public key is compressed to a single 32-byte *fingerprint* by hashing all
//!    public elements. Each signature carries the revealed secret element and
//!    the sibling public element for every message-digest bit, which lets a
//!    verifier reconstruct the fingerprint and compare it to the known one.
//! 2. **Merkle many-time signature**, an XMSS-style construction: `2^height`
//!    independent OTS leaves are hashed into a Merkle tree whose root is the
//!    long-lived public key. Each signature includes the OTS signature plus the
//!    authentication path to the root, and the signer tracks used leaves to
//!    enforce one-time semantics.
//!
//! **This is not a standardized, audited PQ scheme** (it omits the WOTS+ /
//! few-time-signature optimisations of XMSS and SPHINCS+). It is a real,
//! correct hash-based OTS suitable for long-term archival integrity; lattice
//! schemes are deferred (see [`super::agility`]).

use super::{constant_time_eq, from_hex_array, sha256, tagged_hash, to_hex};
use crate::{InteropError, InteropResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Number of bits in the message digest signed by the OTS (SHA-256 → 256).
pub const OTS_BITS: usize = 256;

/// Maximum supported Merkle tree height (`2^height` one-time keys). Keygen cost
/// grows as `2^height`, so callers should keep this modest.
pub const MAX_MERKLE_HEIGHT: u8 = 16;

const CTX_SECRET: &[u8] = b"lamport-sk";
const CTX_PUBLIC_ELEM: &[u8] = b"lamport-pk-elem";
const CTX_FINGERPRINT: &[u8] = b"lamport-pk";
const CTX_LEAF_SEED: &[u8] = b"merkle-leaf-seed";
const CTX_LEAF: &[u8] = b"merkle-leaf";
const CTX_NODE: &[u8] = b"merkle-node";
const CTX_SEED: &[u8] = b"seed-from-bytes";

/// Derives a 32-byte seed from arbitrary secret material, so callers can key the
/// signer from a passphrase or other entropy without handling raw seed bytes.
pub fn seed_from_bytes(material: &[u8]) -> [u8; 32] {
    tagged_hash(CTX_SEED, &[material])
}

fn lamport_secret_element(seed: &[u8; 32], position: usize, bit: u8) -> [u8; 32] {
    tagged_hash(
        CTX_SECRET,
        &[seed, &(position as u32).to_be_bytes(), &[bit]],
    )
}

fn lamport_public_element(secret: &[u8; 32]) -> [u8; 32] {
    tagged_hash(CTX_PUBLIC_ELEM, &[secret])
}

/// Returns all `2 * OTS_BITS` public elements in canonical order
/// (`index = position * 2 + bit`).
fn lamport_public_elements(seed: &[u8; 32]) -> Vec<[u8; 32]> {
    let mut elements = Vec::with_capacity(OTS_BITS * 2);
    for position in 0..OTS_BITS {
        for bit in 0..2u8 {
            let secret = lamport_secret_element(seed, position, bit);
            elements.push(lamport_public_element(&secret));
        }
    }
    elements
}

fn fingerprint_of_elements(elements: &[[u8; 32]]) -> [u8; 32] {
    let refs: Vec<&[u8]> = elements.iter().map(|element| element.as_slice()).collect();
    tagged_hash(CTX_FINGERPRINT, &refs)
}

fn lamport_fingerprint(seed: &[u8; 32]) -> [u8; 32] {
    fingerprint_of_elements(&lamport_public_elements(seed))
}

/// Extracts bit `index` (LSB-first within each byte) from a 32-byte digest.
fn digest_bit(digest: &[u8; 32], index: usize) -> u8 {
    (digest[index / 8] >> (index % 8)) & 1
}

/// A Lamport one-time key pair, identified by a 32-byte seed.
#[derive(Debug, Clone)]
pub struct LamportKeyPair {
    seed: [u8; 32],
    fingerprint: [u8; 32],
}

impl LamportKeyPair {
    /// Builds a key pair deterministically from a 32-byte seed.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let fingerprint = lamport_fingerprint(&seed);
        Self { seed, fingerprint }
    }

    /// The compressed (32-byte) public key fingerprint.
    pub fn public_fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    /// The public key fingerprint as lowercase hex.
    pub fn public_fingerprint_hex(&self) -> String {
        to_hex(&self.fingerprint)
    }

    /// Signs `message`. The key must be used for **at most one** message.
    pub fn sign(&self, message: &[u8]) -> LamportSignature {
        let digest = sha256(message);
        let mut revealed = Vec::with_capacity(OTS_BITS);
        let mut siblings = Vec::with_capacity(OTS_BITS);
        for position in 0..OTS_BITS {
            let bit = digest_bit(&digest, position);
            let secret = lamport_secret_element(&self.seed, position, bit);
            let sibling =
                lamport_public_element(&lamport_secret_element(&self.seed, position, 1 - bit));
            revealed.push(to_hex(&secret));
            siblings.push(to_hex(&sibling));
        }
        LamportSignature { revealed, siblings }
    }
}

/// A Lamport one-time signature: for each digest bit, the revealed secret
/// element and the sibling public element.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LamportSignature {
    /// Revealed secret elements (hex), one per message-digest bit.
    pub revealed: Vec<String>,
    /// Sibling public elements (hex) for the non-revealed bits.
    pub siblings: Vec<String>,
}

/// Reconstructs the public-key fingerprint implied by `signature` over
/// `message`. A correct fingerprint can only be produced with knowledge of the
/// secret pre-images, so comparing it to a trusted fingerprint verifies the
/// signature.
pub fn lamport_reconstruct_fingerprint(
    message: &[u8],
    signature: &LamportSignature,
) -> InteropResult<[u8; 32]> {
    if signature.revealed.len() != OTS_BITS || signature.siblings.len() != OTS_BITS {
        return Err(InteropError::ValidationError(format!(
            "Lamport signature must have {OTS_BITS} elements per side"
        )));
    }
    let digest = sha256(message);
    let mut elements = vec![[0u8; 32]; OTS_BITS * 2];
    for position in 0..OTS_BITS {
        let bit = digest_bit(&digest, position) as usize;
        let revealed = from_hex_array::<32>(&signature.revealed[position])?;
        let sibling = from_hex_array::<32>(&signature.siblings[position])?;
        elements[position * 2 + bit] = lamport_public_element(&revealed);
        elements[position * 2 + (1 - bit)] = sibling;
    }
    Ok(fingerprint_of_elements(&elements))
}

/// Verifies a Lamport signature against a trusted 32-byte fingerprint.
pub fn lamport_verify(
    message: &[u8],
    signature: &LamportSignature,
    fingerprint: &[u8; 32],
) -> bool {
    match lamport_reconstruct_fingerprint(message, signature) {
        Ok(reconstructed) => constant_time_eq(&reconstructed, fingerprint),
        Err(_) => false,
    }
}

fn derive_leaf_seed(master_seed: &[u8; 32], index: u32) -> [u8; 32] {
    tagged_hash(CTX_LEAF_SEED, &[master_seed, &index.to_be_bytes()])
}

fn leaf_node(index: u32, fingerprint: &[u8; 32]) -> [u8; 32] {
    tagged_hash(CTX_LEAF, &[&index.to_be_bytes(), fingerprint])
}

fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    tagged_hash(CTX_NODE, &[left, right])
}

fn build_levels(leaves: Vec<[u8; 32]>) -> Vec<Vec<[u8; 32]>> {
    let mut levels: Vec<Vec<[u8; 32]>> = Vec::new();
    let mut current = leaves;
    while current.len() > 1 {
        let mut next = Vec::with_capacity(current.len() / 2);
        let mut index = 0;
        while index + 1 < current.len() {
            next.push(node_hash(&current[index], &current[index + 1]));
            index += 2;
        }
        levels.push(current);
        current = next;
    }
    levels.push(current);
    levels
}

/// The long-lived public key of a Merkle signature scheme: the tree root and
/// its height.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerklePublicKey {
    /// Merkle root (lowercase hex of 32 bytes).
    pub root: String,
    /// Tree height (`2^height` one-time leaves).
    pub height: u8,
}

impl MerklePublicKey {
    /// Decodes the root into raw bytes.
    pub fn root_bytes(&self) -> InteropResult<[u8; 32]> {
        from_hex_array::<32>(&self.root)
    }
}

/// A Merkle (many-time) signature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleSignature {
    /// Index of the one-time leaf used.
    pub index: u32,
    /// Tree height the signature was produced under.
    pub height: u8,
    /// The underlying one-time signature.
    pub one_time: LamportSignature,
    /// Authentication path (sibling hashes, hex) from the leaf to the root.
    pub auth_path: Vec<String>,
}

/// A stateful Merkle signer holding the secret master seed and the precomputed
/// tree, tracking which one-time leaves have been consumed.
#[derive(Debug, Clone)]
pub struct MerkleSigner {
    master_seed: [u8; 32],
    height: u8,
    levels: Vec<Vec<[u8; 32]>>,
    used: BTreeSet<u32>,
}

impl MerkleSigner {
    /// Builds a signer from a master seed and tree height.
    ///
    /// `height` must be in `1..=MAX_MERKLE_HEIGHT`. Keygen hashes every leaf's
    /// one-time public key, so cost grows as `2^height`.
    pub fn from_seed(master_seed: [u8; 32], height: u8) -> InteropResult<Self> {
        if height == 0 || height > MAX_MERKLE_HEIGHT {
            return Err(InteropError::ValidationError(format!(
                "Merkle height must be in 1..={MAX_MERKLE_HEIGHT}, got {height}"
            )));
        }
        let leaf_count = 1u32 << height;
        let mut leaves = Vec::with_capacity(leaf_count as usize);
        for index in 0..leaf_count {
            let leaf_seed = derive_leaf_seed(&master_seed, index);
            let fingerprint = lamport_fingerprint(&leaf_seed);
            leaves.push(leaf_node(index, &fingerprint));
        }
        Ok(Self {
            master_seed,
            height,
            levels: build_levels(leaves),
            used: BTreeSet::new(),
        })
    }

    /// Tree height.
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Number of one-time leaves (`2^height`).
    pub fn leaf_count(&self) -> u32 {
        1u32 << self.height
    }

    /// Raw 32-byte Merkle root.
    pub fn root(&self) -> [u8; 32] {
        self.levels[self.height as usize][0]
    }

    /// The scheme's public key.
    pub fn public_key(&self) -> MerklePublicKey {
        MerklePublicKey {
            root: to_hex(&self.root()),
            height: self.height,
        }
    }

    /// Returns `true` if leaf `index` has already been used to sign.
    pub fn is_used(&self, index: u32) -> bool {
        self.used.contains(&index)
    }

    /// Number of remaining one-time leaves.
    pub fn remaining(&self) -> u32 {
        self.leaf_count() - self.used.len() as u32
    }

    fn auth_path(&self, index: u32) -> Vec<String> {
        let mut path = Vec::with_capacity(self.height as usize);
        let mut position = index as usize;
        for level in 0..self.height as usize {
            let sibling = position ^ 1;
            path.push(to_hex(&self.levels[level][sibling]));
            position /= 2;
        }
        path
    }

    /// Signs `message` with one-time leaf `index`.
    ///
    /// Errors if `index` is out of range or has already been used.
    pub fn sign(&mut self, index: u32, message: &[u8]) -> InteropResult<MerkleSignature> {
        if index >= self.leaf_count() {
            return Err(InteropError::ValidationError(format!(
                "leaf index {index} out of range (have {} leaves)",
                self.leaf_count()
            )));
        }
        if self.used.contains(&index) {
            return Err(InteropError::UnsupportedFeature(format!(
                "leaf {index} already used; hash-based one-time keys must not be reused"
            )));
        }
        let leaf_seed = derive_leaf_seed(&self.master_seed, index);
        let keypair = LamportKeyPair::from_seed(leaf_seed);
        let one_time = keypair.sign(message);
        let auth_path = self.auth_path(index);
        self.used.insert(index);
        Ok(MerkleSignature {
            index,
            height: self.height,
            one_time,
            auth_path,
        })
    }
}

/// Verifies a Merkle signature against a public key.
pub fn merkle_verify(
    message: &[u8],
    signature: &MerkleSignature,
    public_key: &MerklePublicKey,
) -> bool {
    if signature.height != public_key.height
        || signature.auth_path.len() != signature.height as usize
        || signature.index >= (1u32 << signature.height)
    {
        return false;
    }
    let leaf_fingerprint = match lamport_reconstruct_fingerprint(message, &signature.one_time) {
        Ok(fingerprint) => fingerprint,
        Err(_) => return false,
    };
    let mut node = leaf_node(signature.index, &leaf_fingerprint);
    let mut position = signature.index as usize;
    for sibling_hex in &signature.auth_path {
        let sibling = match from_hex_array::<32>(sibling_hex) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        node = if position & 1 == 0 {
            node_hash(&node, &sibling)
        } else {
            node_hash(&sibling, &node)
        };
        position /= 2;
    }
    let root = match public_key.root_bytes() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    constant_time_eq(&node, &root)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(label: u8) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0] = label;
        bytes[31] = label.wrapping_add(7);
        bytes
    }

    #[test]
    fn test_lamport_sign_verify_roundtrip() {
        let keypair = LamportKeyPair::from_seed(seed(1));
        let signature = keypair.sign(b"preserve this statute");
        assert_eq!(signature.revealed.len(), OTS_BITS);
        assert_eq!(signature.siblings.len(), OTS_BITS);
        assert!(lamport_verify(
            b"preserve this statute",
            &signature,
            &keypair.public_fingerprint()
        ));
    }

    #[test]
    fn test_lamport_rejects_wrong_message_and_tampering() {
        let keypair = LamportKeyPair::from_seed(seed(2));
        let signature = keypair.sign(b"original");
        let fingerprint = keypair.public_fingerprint();
        assert!(!lamport_verify(b"different", &signature, &fingerprint));

        let mut tampered = signature.clone();
        tampered.revealed[0] = to_hex(&[0u8; 32]);
        assert!(!lamport_verify(b"original", &tampered, &fingerprint));

        // A wrong fingerprint never verifies.
        let other = LamportKeyPair::from_seed(seed(3));
        assert!(!lamport_verify(
            b"original",
            &signature,
            &other.public_fingerprint()
        ));
    }

    #[test]
    fn test_lamport_is_deterministic() {
        let a = LamportKeyPair::from_seed(seed(4));
        let b = LamportKeyPair::from_seed(seed(4));
        assert_eq!(a.public_fingerprint(), b.public_fingerprint());
        assert_eq!(a.sign(b"msg"), b.sign(b"msg"));
        // Different seeds give different fingerprints.
        let c = LamportKeyPair::from_seed(seed(5));
        assert_ne!(a.public_fingerprint(), c.public_fingerprint());
    }

    #[test]
    fn test_seed_from_bytes_is_stable() {
        assert_eq!(
            seed_from_bytes(b"passphrase"),
            seed_from_bytes(b"passphrase")
        );
        assert_ne!(seed_from_bytes(b"passphrase"), seed_from_bytes(b"other"));
    }

    #[test]
    fn test_merkle_sign_verify_multiple_leaves() {
        let mut signer = MerkleSigner::from_seed(seed(6), 3).expect("signer");
        assert_eq!(signer.leaf_count(), 8);
        let public_key = signer.public_key();
        for index in [0u32, 3, 7] {
            let message = format!("archive revision {index}");
            let signature = signer.sign(index, message.as_bytes()).expect("sign");
            assert_eq!(signature.auth_path.len(), 3);
            assert!(merkle_verify(message.as_bytes(), &signature, &public_key));
            // Wrong message must fail.
            assert!(!merkle_verify(b"forged", &signature, &public_key));
        }
    }

    #[test]
    fn test_merkle_one_time_enforcement_and_bounds() {
        let mut signer = MerkleSigner::from_seed(seed(7), 2).expect("signer");
        assert_eq!(signer.remaining(), 4);
        let _ = signer.sign(1, b"first").expect("first use");
        assert!(signer.is_used(1));
        assert_eq!(signer.remaining(), 3);
        // Re-using a leaf is rejected.
        assert!(signer.sign(1, b"second").is_err());
        // Out-of-range index is rejected.
        assert!(signer.sign(4, b"oob").is_err());
        // Invalid heights are rejected.
        assert!(MerkleSigner::from_seed(seed(7), 0).is_err());
        assert!(MerkleSigner::from_seed(seed(7), MAX_MERKLE_HEIGHT + 1).is_err());
    }

    #[test]
    fn test_merkle_rejects_tampered_path_and_wrong_root() {
        let mut signer = MerkleSigner::from_seed(seed(8), 3).expect("signer");
        let public_key = signer.public_key();
        let signature = signer.sign(2, b"payload").expect("sign");
        assert!(merkle_verify(b"payload", &signature, &public_key));

        let mut tampered = signature.clone();
        tampered.auth_path[0] = to_hex(&[0xffu8; 32]);
        assert!(!merkle_verify(b"payload", &tampered, &public_key));

        let mut wrong_index = signature.clone();
        wrong_index.index = 5;
        assert!(!merkle_verify(b"payload", &wrong_index, &public_key));

        let other_root = MerklePublicKey {
            root: to_hex(&[0u8; 32]),
            height: 3,
        };
        assert!(!merkle_verify(b"payload", &signature, &other_root));
    }

    #[test]
    fn test_merkle_signature_serde_roundtrip() {
        let mut signer = MerkleSigner::from_seed(seed(9), 2).expect("signer");
        let public_key = signer.public_key();
        let signature = signer.sign(0, b"serialize me").expect("sign");
        let json = serde_json::to_string(&signature).expect("serialize");
        let back: MerkleSignature = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(signature, back);
        assert!(merkle_verify(b"serialize me", &back, &public_key));
    }
}
