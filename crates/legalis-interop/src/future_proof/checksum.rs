//! Quantum-resistant checksums via large-output hashing.
//!
//! Grover's quantum search reduces the cost of a pre-image attack on an `n`-bit
//! hash from `2^n` to `2^(n/2)`. A 256-bit digest therefore retains only ~128
//! bits of post-quantum pre-image security, while a 512-bit digest retains
//! ~256 bits. This module exposes a pluggable [`ChecksumAlgorithm`] favouring
//! large outputs, plus:
//!
//! - **Iterated SHA-512** — repeated application for additional hardening (and a
//!   small work-factor against precomputation), still 512-bit output.
//! - **Concatenation combiner** `SHA-512(x) ‖ SHA-256(x)` — a 768-bit digest
//!   that stays collision-resistant as long as *either* component hash is, which
//!   provides defence-in-depth / cryptographic agility against a future break of
//!   one function.
//!
//! Each [`Checksum`] records the algorithm and the lowercase-hex digest and can
//! re-verify itself against fresh bytes in constant time.

use super::{constant_time_eq, sha256, sha512, sha512_256, to_hex};
use serde::{Deserialize, Serialize};

/// A pluggable, quantum-aware checksum algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChecksumAlgorithm {
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

impl ChecksumAlgorithm {
    /// Computes the raw digest bytes for `data`.
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            ChecksumAlgorithm::Sha256 => sha256(data).to_vec(),
            ChecksumAlgorithm::Sha512 => sha512(data).to_vec(),
            ChecksumAlgorithm::Sha512Trunc256 => sha512_256(data).to_vec(),
            ChecksumAlgorithm::IteratedSha512 { rounds } => {
                let total = (*rounds).max(1);
                let mut state = sha512(data);
                for _ in 1..total {
                    state = sha512(&state);
                }
                state.to_vec()
            }
            ChecksumAlgorithm::ConcatSha512Sha256 => {
                let mut combined = Vec::with_capacity(super::SHA512_BYTES + super::SHA256_BYTES);
                combined.extend_from_slice(&sha512(data));
                combined.extend_from_slice(&sha256(data));
                combined
            }
        }
    }

    /// Digest size in bits.
    pub fn digest_bits(&self) -> usize {
        match self {
            ChecksumAlgorithm::Sha256 | ChecksumAlgorithm::Sha512Trunc256 => 256,
            ChecksumAlgorithm::Sha512 | ChecksumAlgorithm::IteratedSha512 { .. } => 512,
            ChecksumAlgorithm::ConcatSha512Sha256 => 768,
        }
    }

    /// Approximate post-quantum pre-image security in bits (`digest_bits / 2`,
    /// the Grover bound). The combiner is capped at its strongest component.
    pub fn quantum_preimage_bits(&self) -> u32 {
        match self {
            ChecksumAlgorithm::ConcatSha512Sha256 => 256,
            other => (other.digest_bits() as u32) / 2,
        }
    }

    /// Returns `true` if the algorithm offers at least 128-bit post-quantum
    /// pre-image security.
    pub fn is_quantum_resistant(&self) -> bool {
        self.quantum_preimage_bits() >= 128
    }

    /// A stable, human-readable identifier including parameters (used for BagIt
    /// manifest file names, where the round count matters).
    pub fn canonical_id(&self) -> String {
        match self {
            ChecksumAlgorithm::IteratedSha512 { rounds } => {
                format!("iterated-sha-512-r{}", (*rounds).max(1))
            }
            other => other.family_id().to_string(),
        }
    }

    /// The parameter-free family identifier, used as the
    /// [`super::agility::AlgorithmRegistry`] key.
    pub fn family_id(&self) -> &'static str {
        match self {
            ChecksumAlgorithm::Sha256 => "sha-256",
            ChecksumAlgorithm::Sha512 => "sha-512",
            ChecksumAlgorithm::Sha512Trunc256 => "sha-512_256",
            ChecksumAlgorithm::IteratedSha512 { .. } => "iterated-sha-512",
            ChecksumAlgorithm::ConcatSha512Sha256 => "concat-sha-512+sha-256",
        }
    }
}

/// A computed checksum: an algorithm plus its lowercase-hex digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checksum {
    /// Algorithm used to produce [`Checksum::digest`].
    pub algorithm: ChecksumAlgorithm,
    /// Lowercase-hex digest.
    pub digest: String,
}

impl Checksum {
    /// Computes a checksum of `data` using `algorithm`.
    pub fn compute(algorithm: ChecksumAlgorithm, data: &[u8]) -> Self {
        Self {
            algorithm,
            digest: to_hex(&algorithm.digest(data)),
        }
    }

    /// Re-computes the digest of `data` and compares it in constant time against
    /// the stored value.
    pub fn verify(&self, data: &[u8]) -> bool {
        let expected = to_hex(&self.algorithm.digest(data));
        constant_time_eq(self.digest.as_bytes(), expected.as_bytes())
    }

    /// Digest size in bits.
    pub fn digest_bits(&self) -> usize {
        self.algorithm.digest_bits()
    }

    /// Stable identifier of the underlying algorithm.
    pub fn algorithm_id(&self) -> String {
        self.algorithm.canonical_id()
    }
}

/// Computes a redundant set of checksums of `data`, one per algorithm.
///
/// Storing fixity under several independent algorithms is a core long-term
/// preservation practice: it survives the deprecation (or break) of any single
/// hash function.
pub fn compute_set(algorithms: &[ChecksumAlgorithm], data: &[u8]) -> Vec<Checksum> {
    algorithms
        .iter()
        .map(|algorithm| Checksum::compute(*algorithm, data))
        .collect()
}

/// Verifies every checksum in `set` against `data`, returning the canonical ids
/// of any that fail (empty slice means all passed).
pub fn verify_set(set: &[Checksum], data: &[u8]) -> Vec<String> {
    set.iter()
        .filter(|checksum| !checksum.verify(data))
        .map(Checksum::algorithm_id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_and_verify_each_algorithm() {
        let data = b"long-term preservation payload";
        let algorithms = [
            ChecksumAlgorithm::Sha256,
            ChecksumAlgorithm::Sha512,
            ChecksumAlgorithm::Sha512Trunc256,
            ChecksumAlgorithm::IteratedSha512 { rounds: 4 },
            ChecksumAlgorithm::ConcatSha512Sha256,
        ];
        for algorithm in algorithms {
            let checksum = Checksum::compute(algorithm, data);
            assert!(checksum.verify(data), "{}", algorithm.canonical_id());
            assert!(
                !checksum.verify(b"tampered"),
                "{}",
                algorithm.canonical_id()
            );
            assert_eq!(checksum.digest.len(), algorithm.digest_bits() / 4);
        }
    }

    #[test]
    fn test_digest_bits_and_quantum_levels() {
        assert_eq!(ChecksumAlgorithm::Sha256.digest_bits(), 256);
        assert_eq!(ChecksumAlgorithm::Sha512.digest_bits(), 512);
        assert_eq!(ChecksumAlgorithm::ConcatSha512Sha256.digest_bits(), 768);
        assert_eq!(ChecksumAlgorithm::Sha256.quantum_preimage_bits(), 128);
        assert_eq!(ChecksumAlgorithm::Sha512.quantum_preimage_bits(), 256);
        assert!(ChecksumAlgorithm::Sha512.is_quantum_resistant());
        assert!(ChecksumAlgorithm::ConcatSha512Sha256.is_quantum_resistant());
        assert!(ChecksumAlgorithm::Sha256.is_quantum_resistant());
    }

    #[test]
    fn test_iterated_sha512_depends_on_rounds() {
        let data = b"hardening";
        let one = Checksum::compute(ChecksumAlgorithm::IteratedSha512 { rounds: 1 }, data);
        let many = Checksum::compute(ChecksumAlgorithm::IteratedSha512 { rounds: 8 }, data);
        // A single round equals a plain SHA-512.
        assert_eq!(one.digest, to_hex(&sha512(data)));
        // More rounds produce a different, deterministic digest.
        assert_ne!(one.digest, many.digest);
        assert_eq!(
            many.digest,
            Checksum::compute(ChecksumAlgorithm::IteratedSha512 { rounds: 8 }, data).digest
        );
        // rounds = 0 is clamped to 1.
        let zero = Checksum::compute(ChecksumAlgorithm::IteratedSha512 { rounds: 0 }, data);
        assert_eq!(zero.digest, one.digest);
    }

    #[test]
    fn test_concat_combiner_layout() {
        let data = b"defence in depth";
        let combiner = ChecksumAlgorithm::ConcatSha512Sha256.digest(data);
        assert_eq!(
            combiner.len(),
            super::super::SHA512_BYTES + super::super::SHA256_BYTES
        );
        assert_eq!(&combiner[..super::super::SHA512_BYTES], &sha512(data));
        assert_eq!(&combiner[super::super::SHA512_BYTES..], &sha256(data));
    }

    #[test]
    fn test_compute_and_verify_set() {
        let data = b"redundant fixity";
        let algorithms = [
            ChecksumAlgorithm::Sha512,
            ChecksumAlgorithm::ConcatSha512Sha256,
        ];
        let set = compute_set(&algorithms, data);
        assert_eq!(set.len(), 2);
        assert!(verify_set(&set, data).is_empty());
        let failures = verify_set(&set, b"corrupted");
        assert_eq!(failures.len(), 2);
        assert!(failures.contains(&"sha-512".to_string()));
    }

    #[test]
    fn test_checksum_serde_roundtrip() {
        let checksum = Checksum::compute(ChecksumAlgorithm::IteratedSha512 { rounds: 3 }, b"x");
        let json = serde_json::to_string(&checksum).expect("serialize");
        let back: Checksum = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(checksum, back);
        assert!(back.verify(b"x"));
    }
}
