//! Future-proof, long-term preservation formats and quantum-aware cryptography.
//!
//! This module group implements the **Quantum-Safe Format Migration** family of
//! features. It is concerned with keeping legal documents *verifiable and
//! readable for decades*, including in a world with cryptographically relevant
//! quantum computers. It is fully pure-Rust and `scirs2`-free, reusing only the
//! workspace's audited [`sha2`] crate for the underlying hash primitive.
//!
//! - **Cryptographic agility** ([`agility`]): an [`agility::AlgorithmRegistry`]
//!   of hash, signature and KEM algorithms (with classical/quantum security
//!   levels and life-cycle status), plus a versioned [`agility::CryptoEnvelope`]
//!   so the digest/signature scheme protecting a document is *pluggable* and can
//!   be *upgraded* in place without breaking older artifacts.
//! - **Quantum-resistant checksums** ([`checksum`]): large-output digests
//!   ([`checksum::ChecksumAlgorithm`]) built on SHA-512, SHA-512/256, iterated
//!   SHA-512 hardening, and a SHA-512‖SHA-256 concatenation combiner. Grover's
//!   algorithm only halves pre-image security, so a 512-bit digest still offers
//!   ~256-bit post-quantum pre-image resistance.
//! - **Hash-based signatures** ([`hash_sig`]): a self-contained Lamport
//!   one-time signature with a hash-committed public key, lifted to a many-time
//!   [`hash_sig::MerkleSigner`] via a Merkle tree (an XMSS-style construction).
//!   This is **not** a standardized, audited post-quantum scheme; lattice
//!   schemes (ML-DSA / ML-KEM) are *deferred* and registered as
//!   [`agility::AlgorithmStatus::Planned`].
//! - **Long-term preservation archives** ([`archive`]): a self-describing,
//!   BagIt-like container ([`archive::PreservationArchive`]) carrying a manifest,
//!   redundant fixity checksums, migration history and an optional post-quantum
//!   hash-based signature, plus pluggable [`archive::ArchivalStrategy`] presets
//!   and [`archive::ArchivalPlan`] for archival-strategy planning. It integrates
//!   with the crate's [`crate::FormatImporter`] / [`crate::FormatExporter`]
//!   pipeline as [`crate::LegalFormat::PreservationArchive`].
//!
//! All hashing in this module is domain-separated (see [`DOMAIN_SEP`]) to avoid
//! cross-protocol collisions, and every algorithm is deterministic, so artifacts
//! are byte-for-byte reproducible (signatures are derived from a caller-supplied
//! seed rather than ambient randomness, which keeps the crate `rand`-free).

pub mod agility;
pub mod archive;
pub mod checksum;
pub mod hash_sig;

use crate::{InteropError, InteropResult};
use sha2::{Digest, Sha256, Sha512, Sha512_256};

/// Domain-separation tag prefixed (with a context label) to all hashing in this
/// module, preventing cross-protocol collisions.
pub const DOMAIN_SEP: &[u8] = b"legalis.future-proof/v1";

/// Length, in bytes, of a SHA-256 digest.
pub const SHA256_BYTES: usize = 32;

/// Length, in bytes, of a SHA-512 digest.
pub const SHA512_BYTES: usize = 64;

/// Computes the 32-byte SHA-256 digest of `data`.
pub fn sha256(data: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(out.as_ref());
    bytes
}

/// Computes the 64-byte SHA-512 digest of `data`.
pub fn sha512(data: &[u8]) -> [u8; SHA512_BYTES] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut bytes = [0u8; SHA512_BYTES];
    bytes.copy_from_slice(out.as_ref());
    bytes
}

/// Computes the 32-byte SHA-512/256 digest of `data` (a truncated SHA-512 with a
/// distinct initialization vector; faster than SHA-256 on 64-bit hardware).
pub fn sha512_256(data: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha512_256::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(out.as_ref());
    bytes
}

/// Computes a domain-separated SHA-256 over a context label and one or more
/// byte segments. Each segment is length-prefixed (8-byte big-endian) so the
/// hash is unambiguous regardless of segment boundaries.
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
    let out = hasher.finalize();
    let mut bytes = [0u8; SHA256_BYTES];
    bytes.copy_from_slice(out.as_ref());
    bytes
}

/// Encodes bytes as a lowercase hex string.
pub fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

/// Decodes a lowercase or uppercase hex string into bytes.
pub fn from_hex(text: &str) -> InteropResult<Vec<u8>> {
    let trimmed = text.trim();
    if !trimmed.len().is_multiple_of(2) {
        return Err(InteropError::ParseError(
            "hex string has an odd number of characters".to_string(),
        ));
    }
    let bytes = trimmed.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let mut index = 0usize;
    while index + 1 < bytes.len() {
        let hi = hex_value(bytes[index])?;
        let lo = hex_value(bytes[index + 1])?;
        out.push((hi << 4) | lo);
        index += 2;
    }
    Ok(out)
}

/// Decodes a fixed-size hex string into a `[u8; N]`, erroring on the wrong
/// length.
pub fn from_hex_array<const N: usize>(text: &str) -> InteropResult<[u8; N]> {
    let bytes = from_hex(text)?;
    if bytes.len() != N {
        return Err(InteropError::ParseError(format!(
            "expected {} hex bytes, found {}",
            N,
            bytes.len()
        )));
    }
    let mut array = [0u8; N];
    array.copy_from_slice(&bytes);
    Ok(array)
}

fn hex_value(character: u8) -> InteropResult<u8> {
    match character {
        b'0'..=b'9' => Ok(character - b'0'),
        b'a'..=b'f' => Ok(character - b'a' + 10),
        b'A'..=b'F' => Ok(character - b'A' + 10),
        other => Err(InteropError::ParseError(format!(
            "invalid hex digit: {}",
            other as char
        ))),
    }
}

/// Constant-time byte-slice equality, returning `false` for length mismatches.
///
/// Used for digest and signature comparison so verification does not leak the
/// position of the first differing byte through timing.
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

/// Returns the current UTC time as an RFC 3339 timestamp.
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_vector() {
        // SHA-256("abc") test vector.
        let digest = sha256(b"abc");
        assert_eq!(
            to_hex(&digest),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_sha512_known_vector() {
        // SHA-512("abc") test vector.
        let digest = sha512(b"abc");
        assert_eq!(
            to_hex(&digest),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
        assert_eq!(digest.len(), SHA512_BYTES);
    }

    #[test]
    fn test_sha512_256_length_and_difference() {
        let truncated = sha512_256(b"abc");
        assert_eq!(truncated.len(), SHA256_BYTES);
        // SHA-512/256 must differ from a plain SHA-256 of the same input.
        assert_ne!(truncated, sha256(b"abc"));
    }

    #[test]
    fn test_hex_roundtrip() {
        let bytes: Vec<u8> = (0..=255u8).collect();
        let hex = to_hex(&bytes);
        assert_eq!(hex.len(), 512);
        let decoded = from_hex(&hex).expect("decode");
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_from_hex_rejects_bad_input() {
        assert!(from_hex("abc").is_err()); // odd length
        assert!(from_hex("zz").is_err()); // invalid digit
        assert!(from_hex_array::<4>("aabb").is_err()); // wrong length
        let array = from_hex_array::<2>("aabb").expect("array");
        assert_eq!(array, [0xaa, 0xbb]);
    }

    #[test]
    fn test_tagged_hash_is_unambiguous() {
        // Length-prefixing prevents segment-boundary collisions.
        let a = tagged_hash(b"ctx", &[b"ab", b"c"]);
        let b = tagged_hash(b"ctx", &[b"a", b"bc"]);
        let c = tagged_hash(b"other", &[b"ab", b"c"]);
        assert_ne!(a, b);
        assert_ne!(a, c);
        assert_eq!(a, tagged_hash(b"ctx", &[b"ab", b"c"]));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
        assert!(constant_time_eq(b"", b""));
    }
}
