//! Post-quantum hash primitives and post-quantum hash chains.
//!
//! Hash-based cryptography is the only signature/integrity family that is
//! *both* believed to be quantum-resistant *and* implementable in pure Rust
//! without elliptic-curve or lattice machinery — its security reduces solely to
//! the preimage / second-preimage resistance of an underlying cryptographic
//! hash. This module therefore provides clean-room, dependency-free
//! implementations of **SHA-256** and **SHA-512** (FIPS 180-4) and builds a
//! [`PqHashChain`] on top of them.
//!
//! Against a quantum adversary, Grover's algorithm only quadratically speeds up
//! preimage search, so an `m`-bit digest retains roughly `m/2` bits of
//! post-quantum preimage security. SHA-256 therefore offers ~128-bit and
//! SHA-512 ~256-bit post-quantum security, which is why the chain is
//! parameterised over a [`PqHashAlgorithm`] (crypto-agility — a core
//! post-quantum design principle) rather than hard-wiring one digest.
//!
//! These primitives deliberately *complement* the existing weak
//! [`crate::AuditRecord`] hash chain rather than replacing it; the
//! [`crate::quantum::hybrid`] layer combines both.

use crate::AuditRecord;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain-separation tag mixed into every post-quantum chain link.
const PQ_CHAIN_TAG: u8 = 0x10;

/// A cryptographic hash algorithm with a post-quantum-adequate output size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PqHashAlgorithm {
    /// SHA-256 (FIPS 180-4). 256-bit output → ~128-bit post-quantum security.
    Sha256,
    /// SHA-512 (FIPS 180-4). 512-bit output → ~256-bit post-quantum security.
    Sha512,
}

impl PqHashAlgorithm {
    /// Length in bytes of a digest produced by this algorithm.
    pub fn digest_len(self) -> usize {
        match self {
            PqHashAlgorithm::Sha256 => 32,
            PqHashAlgorithm::Sha512 => 64,
        }
    }

    /// Approximate post-quantum preimage security in bits (Grover-adjusted).
    pub fn quantum_security_bits(self) -> usize {
        match self {
            PqHashAlgorithm::Sha256 => 128,
            PqHashAlgorithm::Sha512 => 256,
        }
    }

    /// Human-readable algorithm name.
    pub fn name(self) -> &'static str {
        match self {
            PqHashAlgorithm::Sha256 => "SHA-256",
            PqHashAlgorithm::Sha512 => "SHA-512",
        }
    }
}

/// Dispatches a hash over `data` for the chosen algorithm.
pub fn pq_hash(algorithm: PqHashAlgorithm, data: &[u8]) -> Vec<u8> {
    match algorithm {
        PqHashAlgorithm::Sha256 => sha256(data).to_vec(),
        PqHashAlgorithm::Sha512 => sha512(data).to_vec(),
    }
}

/// Hashes `data` prefixed with a single domain-separation `tag` byte.
pub(crate) fn hash_tagged(algorithm: PqHashAlgorithm, tag: u8, data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(1 + data.len());
    buf.push(tag);
    buf.extend_from_slice(data);
    pq_hash(algorithm, &buf)
}

/// Hashes the concatenation of `left` and `right` under a domain-separation
/// `tag` (used for internal Merkle nodes).
pub(crate) fn hash_pair(algorithm: PqHashAlgorithm, tag: u8, left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(1 + left.len() + right.len());
    buf.push(tag);
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    pq_hash(algorithm, &buf)
}

/// A keyed, hash-based pseudo-random generator used to expand a compact secret
/// `seed` into the many secret values that hash-based signatures require.
///
/// `output = H(seed || label || idx1_le || idx2_le)`.
pub(crate) fn prg(
    algorithm: PqHashAlgorithm,
    seed: &[u8],
    label: u8,
    idx1: u64,
    idx2: u64,
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(seed.len() + 1 + 16);
    buf.extend_from_slice(seed);
    buf.push(label);
    buf.extend_from_slice(&idx1.to_le_bytes());
    buf.extend_from_slice(&idx2.to_le_bytes());
    pq_hash(algorithm, &buf)
}

/// Lower-case hexadecimal encoding (avoids pulling in the `hex` crate).
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from_digit((byte >> 4) as u32, 16).unwrap_or('0'));
        out.push(char::from_digit((byte & 0x0f) as u32, 16).unwrap_or('0'));
    }
    out
}

// ---------------------------------------------------------------------------
// SHA-256 (FIPS 180-4)
// ---------------------------------------------------------------------------

const SHA256_H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Computes the SHA-256 digest of `data`.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut h = SHA256_H0;

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for block in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w.iter_mut().enumerate().take(16) {
            let j = i * 4;
            *word = u32::from_be_bytes([block[j], block[j + 1], block[j + 2], block[j + 3]]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for t in 0..64 {
            let big_s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(big_s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[t])
                .wrapping_add(w[t]);
            let big_s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = big_s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// SHA-512 (FIPS 180-4)
// ---------------------------------------------------------------------------

const SHA512_H0: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

/// Computes the SHA-512 digest of `data`.
pub fn sha512(data: &[u8]) -> [u8; 64] {
    let mut h = SHA512_H0;

    let bit_len = (data.len() as u128).wrapping_mul(8);
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 128 != 112 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for block in msg.chunks_exact(128) {
        let mut w = [0u64; 80];
        for (i, word) in w.iter_mut().enumerate().take(16) {
            let j = i * 8;
            *word = u64::from_be_bytes([
                block[j],
                block[j + 1],
                block[j + 2],
                block[j + 3],
                block[j + 4],
                block[j + 5],
                block[j + 6],
                block[j + 7],
            ]);
        }
        for t in 16..80 {
            let s0 = w[t - 15].rotate_right(1) ^ w[t - 15].rotate_right(8) ^ (w[t - 15] >> 7);
            let s1 = w[t - 2].rotate_right(19) ^ w[t - 2].rotate_right(61) ^ (w[t - 2] >> 6);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for t in 0..80 {
            let big_s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(big_s1)
                .wrapping_add(ch)
                .wrapping_add(SHA512_K[t])
                .wrapping_add(w[t]);
            let big_s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = big_s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 64];
    for (i, word) in h.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// Post-quantum hash chain
// ---------------------------------------------------------------------------

/// Genesis (initialisation) value for an empty chain of a given algorithm.
fn genesis(algorithm: PqHashAlgorithm) -> Vec<u8> {
    pq_hash(algorithm, b"legalis-audit::pq-hash-chain::genesis::v1")
}

/// Builds the canonical byte preimage for a chain link.
///
/// Binds the previous link, the record id/timestamp, the record's *existing*
/// (classical) hash and its statute id. Re-using `record_hash` keeps the
/// preimage deterministic (it contains no unordered `HashMap`s) while still
/// transitively committing to the full record content.
fn link_input(previous: &[u8], record: &AuditRecord) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(PQ_CHAIN_TAG);
    buf.extend_from_slice(previous);
    buf.extend_from_slice(record.id.as_bytes());
    buf.extend_from_slice(&record.timestamp.timestamp().to_le_bytes());
    buf.extend_from_slice(record.record_hash.as_bytes());
    buf.extend_from_slice(record.statute_id.as_bytes());
    buf
}

/// A single link in a [`PqHashChain`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqChainLink {
    /// Identifier of the record this link covers.
    pub record_id: Uuid,
    /// Digest of the previous link (or the genesis value for the first link).
    pub previous: Vec<u8>,
    /// `H(previous || canonical(record))`.
    pub digest: Vec<u8>,
}

impl PqChainLink {
    /// Returns the link digest as a hex string.
    pub fn digest_hex(&self) -> String {
        to_hex(&self.digest)
    }
}

/// A tamper-evident hash chain over audit records using a post-quantum-adequate
/// cryptographic hash.
///
/// This is a stronger, parallel integrity structure to the crate's built-in
/// (non-cryptographic) record hash chain; it does not modify or replace it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqHashChain {
    /// Algorithm used for every link.
    pub algorithm: PqHashAlgorithm,
    /// Ordered links, one per record.
    pub links: Vec<PqChainLink>,
}

impl PqHashChain {
    /// Builds a post-quantum hash chain over `records` in order.
    pub fn from_records(records: &[AuditRecord], algorithm: PqHashAlgorithm) -> Self {
        let mut links = Vec::with_capacity(records.len());
        let mut previous = genesis(algorithm);
        for record in records {
            let input = link_input(&previous, record);
            let digest = pq_hash(algorithm, &input);
            links.push(PqChainLink {
                record_id: record.id,
                previous: previous.clone(),
                digest: digest.clone(),
            });
            previous = digest;
        }
        Self { algorithm, links }
    }

    /// Returns the head (tip) digest of the chain, or the genesis value if the
    /// chain is empty.
    pub fn head(&self) -> Vec<u8> {
        match self.links.last() {
            Some(link) => link.digest.clone(),
            None => genesis(self.algorithm),
        }
    }

    /// Returns the head digest as a hex string.
    pub fn head_hex(&self) -> String {
        to_hex(&self.head())
    }

    /// Recomputes the chain from `records` and verifies that every stored link
    /// matches (digest, ordering and back-link).
    pub fn verify(&self, records: &[AuditRecord]) -> bool {
        if records.len() != self.links.len() {
            return false;
        }
        let mut previous = genesis(self.algorithm);
        for (record, link) in records.iter().zip(self.links.iter()) {
            if link.record_id != record.id || link.previous != previous {
                return false;
            }
            let input = link_input(&previous, record);
            let digest = pq_hash(self.algorithm, &input);
            if digest != link.digest {
                return false;
            }
            previous = digest;
        }
        true
    }

    /// Number of links in the chain.
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Whether the chain has no links.
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn record(statute: &str) -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "ok".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_sha256_nist_vectors() {
        assert_eq!(
            to_hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            to_hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            to_hex(&sha256(
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
            )),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn test_sha512_nist_vectors() {
        assert_eq!(
            to_hex(&sha512(b"")),
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        );
        assert_eq!(
            to_hex(&sha512(b"abc")),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn test_pq_hash_dispatch_and_lengths() {
        assert_eq!(pq_hash(PqHashAlgorithm::Sha256, b"x").len(), 32);
        assert_eq!(pq_hash(PqHashAlgorithm::Sha512, b"x").len(), 64);
        assert_eq!(PqHashAlgorithm::Sha256.quantum_security_bits(), 128);
        assert_eq!(PqHashAlgorithm::Sha512.quantum_security_bits(), 256);
    }

    #[test]
    fn test_to_hex_roundtrip_shape() {
        assert_eq!(to_hex(&[0x00, 0x0f, 0xff]), "000fff");
    }

    #[test]
    fn test_pq_chain_build_and_verify() {
        let records: Vec<_> = (0..6).map(|i| record(&format!("s-{i}"))).collect();
        let chain = PqHashChain::from_records(&records, PqHashAlgorithm::Sha256);
        assert_eq!(chain.len(), 6);
        assert!(!chain.is_empty());
        assert!(chain.verify(&records));
        assert_eq!(chain.head().len(), 32);
    }

    #[test]
    fn test_pq_chain_detects_tamper() {
        let mut records: Vec<_> = (0..4).map(|i| record(&format!("s-{i}"))).collect();
        let chain = PqHashChain::from_records(&records, PqHashAlgorithm::Sha512);
        assert!(chain.verify(&records));
        // Mutate a record's stored hash → chain must reject.
        records[2].record_hash = "tampered".to_string();
        assert!(!chain.verify(&records));
    }

    #[test]
    fn test_pq_chain_detects_reorder() {
        let records: Vec<_> = (0..4).map(|i| record(&format!("s-{i}"))).collect();
        let chain = PqHashChain::from_records(&records, PqHashAlgorithm::Sha256);
        let mut reordered = records.clone();
        reordered.swap(0, 3);
        assert!(!chain.verify(&reordered));
    }

    #[test]
    fn test_empty_chain_head_is_genesis() {
        let chain = PqHashChain::from_records(&[], PqHashAlgorithm::Sha256);
        assert!(chain.is_empty());
        assert_eq!(chain.head(), genesis(PqHashAlgorithm::Sha256));
        assert!(chain.verify(&[]));
    }
}
