//! Quantum random beacons.
//!
//! A randomness beacon emits a public, tamper-evident sequence of unpredictable
//! values that auditors can later *verify*. This module implements the full
//! beacon protocol in pure Rust:
//!
//! * each round commits to fresh entropy (`commitment = H(entropy)`) and chains
//!   its output into the previous one (`output = H(prev || entropy || index)`),
//!   yielding a hash chain that anyone can recompute and check;
//! * the entropy itself comes from a pluggable [`EntropySource`].
//!
//! The default [`SystemEntropySource`] draws from the OS CSPRNG (via the crate's
//! existing `rand` dependency), and [`SeededEntropySource`] is a deterministic
//! hash-based source for reproducible tests and offline verification.
//!
//! ## Deferred: true quantum entropy
//! A hardware **quantum** random number generator (QRNG) — or an external
//! verifiable quantum beacon such as a national metrology service — would simply
//! be *another* implementation of [`EntropySource`]. Binding to such hardware or
//! network service is intentionally **not** included here (it requires hardware
//! / live endpoints); the trait is the pluggable seam where it would attach. The
//! protocol, chaining and verification above are complete and source-agnostic.

use super::pq_hash::{PqHashAlgorithm, pq_hash, to_hex};
use crate::AuditResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BEACON_COMMIT_TAG: u8 = 0x60;
const BEACON_OUTPUT_TAG: u8 = 0x61;
const BEACON_SEED_TAG: u8 = 0x62;

/// A pluggable source of entropy for a [`QuantumRandomBeacon`].
///
/// Implementations include the OS CSPRNG ([`SystemEntropySource`]) and a
/// deterministic test source ([`SeededEntropySource`]). A real QRNG would
/// implement this trait (see module docs).
pub trait EntropySource {
    /// Fills `out` with fresh entropy.
    fn fill_entropy(&mut self, out: &mut [u8]) -> AuditResult<()>;

    /// Short identifier of the source (recorded with each round).
    fn source_name(&self) -> &str;
}

/// Entropy from the operating-system CSPRNG.
#[derive(Debug, Clone, Default)]
pub struct SystemEntropySource;

impl EntropySource for SystemEntropySource {
    fn fill_entropy(&mut self, out: &mut [u8]) -> AuditResult<()> {
        let mut offset = 0;
        while offset < out.len() {
            let block = rand::random::<[u8; 32]>();
            let take = (out.len() - offset).min(block.len());
            if let Some(slot) = out.get_mut(offset..offset + take) {
                slot.copy_from_slice(&block[..take]);
            }
            offset += take;
        }
        Ok(())
    }

    fn source_name(&self) -> &str {
        "system-csprng"
    }
}

/// A deterministic hash-based entropy source (for reproducible tests and
/// offline verification). Not cryptographically unpredictable to a party that
/// knows the seed — do not use for production randomness.
#[derive(Debug, Clone)]
pub struct SeededEntropySource {
    algorithm: PqHashAlgorithm,
    seed: Vec<u8>,
    counter: u64,
}

impl SeededEntropySource {
    /// Creates a deterministic source from a seed.
    pub fn new(algorithm: PqHashAlgorithm, seed: &[u8]) -> Self {
        Self {
            algorithm,
            seed: seed.to_vec(),
            counter: 0,
        }
    }
}

impl EntropySource for SeededEntropySource {
    fn fill_entropy(&mut self, out: &mut [u8]) -> AuditResult<()> {
        let mut offset = 0;
        while offset < out.len() {
            let mut buf = vec![BEACON_SEED_TAG];
            buf.extend_from_slice(&self.seed);
            buf.extend_from_slice(&self.counter.to_le_bytes());
            let block = pq_hash(self.algorithm, &buf);
            self.counter += 1;
            let take = (out.len() - offset).min(block.len());
            if let Some(slot) = out.get_mut(offset..offset + take) {
                slot.copy_from_slice(&block[..take]);
            }
            offset += take;
        }
        Ok(())
    }

    fn source_name(&self) -> &str {
        "seeded-deterministic"
    }
}

/// One published round of a beacon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeaconRound {
    /// Sequential round index (genesis round is 0).
    pub index: u64,
    /// When the round was produced.
    pub timestamp: DateTime<Utc>,
    /// Output of the previous round (or the genesis value for round 0).
    pub previous_output: Vec<u8>,
    /// Commitment to this round's entropy: `H(entropy)`.
    pub entropy_commitment: Vec<u8>,
    /// The revealed entropy.
    pub entropy: Vec<u8>,
    /// `H(previous_output || entropy || index)`.
    pub output: Vec<u8>,
    /// Name of the entropy source used.
    pub source: String,
}

impl BeaconRound {
    /// Output as a hex string.
    pub fn output_hex(&self) -> String {
        to_hex(&self.output)
    }
}

/// Genesis value for a beacon of a given algorithm.
fn genesis(algorithm: PqHashAlgorithm) -> Vec<u8> {
    pq_hash(algorithm, b"legalis-audit::quantum-beacon::genesis::v1")
}

/// Computes the commitment to `entropy`.
fn commitment(algorithm: PqHashAlgorithm, entropy: &[u8]) -> Vec<u8> {
    let mut buf = vec![BEACON_COMMIT_TAG];
    buf.extend_from_slice(entropy);
    pq_hash(algorithm, &buf)
}

/// Computes a round's output from its inputs.
fn round_output(
    algorithm: PqHashAlgorithm,
    previous_output: &[u8],
    entropy: &[u8],
    index: u64,
) -> Vec<u8> {
    let mut buf = vec![BEACON_OUTPUT_TAG];
    buf.extend_from_slice(previous_output);
    buf.extend_from_slice(entropy);
    buf.extend_from_slice(&index.to_le_bytes());
    pq_hash(algorithm, &buf)
}

/// A verifiable randomness beacon backed by a pluggable [`EntropySource`].
pub struct QuantumRandomBeacon {
    algorithm: PqHashAlgorithm,
    entropy_bytes: usize,
    source: Box<dyn EntropySource>,
    rounds: Vec<BeaconRound>,
}

impl QuantumRandomBeacon {
    /// Creates a beacon over `algorithm`, pulling `entropy_bytes` of entropy per
    /// round from `source`.
    pub fn new(
        algorithm: PqHashAlgorithm,
        entropy_bytes: usize,
        source: Box<dyn EntropySource>,
    ) -> Self {
        let bytes = entropy_bytes.max(1);
        Self {
            algorithm,
            entropy_bytes: bytes,
            source,
            rounds: Vec::new(),
        }
    }

    /// Convenience constructor using the system CSPRNG and a digest-sized
    /// entropy draw per round.
    pub fn with_system_source(algorithm: PqHashAlgorithm) -> Self {
        let bytes = algorithm.digest_len();
        Self::new(algorithm, bytes, Box::new(SystemEntropySource))
    }

    /// Produces and appends the next round, returning a reference to it.
    pub fn next_round(&mut self) -> AuditResult<&BeaconRound> {
        let mut entropy = vec![0u8; self.entropy_bytes];
        self.source.fill_entropy(&mut entropy)?;

        let index = self.rounds.len() as u64;
        let previous_output = match self.rounds.last() {
            Some(round) => round.output.clone(),
            None => genesis(self.algorithm),
        };
        let entropy_commitment = commitment(self.algorithm, &entropy);
        let output = round_output(self.algorithm, &previous_output, &entropy, index);

        self.rounds.push(BeaconRound {
            index,
            timestamp: Utc::now(),
            previous_output,
            entropy_commitment,
            entropy,
            output,
            source: self.source.source_name().to_string(),
        });
        // The push above guarantees a last element.
        match self.rounds.last() {
            Some(round) => Ok(round),
            None => Err(crate::AuditError::StorageError(
                "beacon round vanished after push".to_string(),
            )),
        }
    }

    /// The most recent beacon output, if any rounds have been produced.
    pub fn latest_output(&self) -> Option<&[u8]> {
        self.rounds.last().map(|round| round.output.as_slice())
    }

    /// All rounds produced so far.
    pub fn rounds(&self) -> &[BeaconRound] {
        &self.rounds
    }

    /// Number of rounds produced.
    pub fn len(&self) -> usize {
        self.rounds.len()
    }

    /// Whether no rounds have been produced.
    pub fn is_empty(&self) -> bool {
        self.rounds.is_empty()
    }

    /// Verifies the full beacon chain: each round's entropy commitment matches
    /// its revealed entropy, each output equals `H(prev || entropy || index)`,
    /// indices are sequential, and back-links are intact.
    pub fn verify_chain(&self) -> bool {
        verify_rounds(self.algorithm, &self.rounds)
    }
}

/// Verifies an arbitrary slice of [`BeaconRound`]s for a given algorithm.
pub fn verify_rounds(algorithm: PqHashAlgorithm, rounds: &[BeaconRound]) -> bool {
    let mut previous = genesis(algorithm);
    for (i, round) in rounds.iter().enumerate() {
        if round.index != i as u64 || round.previous_output != previous {
            return false;
        }
        if round.entropy_commitment != commitment(algorithm, &round.entropy) {
            return false;
        }
        let expected = round_output(
            algorithm,
            &round.previous_output,
            &round.entropy,
            round.index,
        );
        if round.output != expected {
            return false;
        }
        previous = round.output.clone();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_source_is_reproducible() {
        let mut a = SeededEntropySource::new(PqHashAlgorithm::Sha256, b"seed");
        let mut b = SeededEntropySource::new(PqHashAlgorithm::Sha256, b"seed");
        let mut buf_a = [0u8; 40];
        let mut buf_b = [0u8; 40];
        a.fill_entropy(&mut buf_a).expect("fill");
        b.fill_entropy(&mut buf_b).expect("fill");
        assert_eq!(buf_a, buf_b);
        assert_eq!(a.source_name(), "seeded-deterministic");
    }

    #[test]
    fn test_beacon_chain_builds_and_verifies() {
        let source = Box::new(SeededEntropySource::new(PqHashAlgorithm::Sha256, b"beacon"));
        let mut beacon = QuantumRandomBeacon::new(PqHashAlgorithm::Sha256, 32, source);
        for _ in 0..5 {
            beacon.next_round().expect("round");
        }
        assert_eq!(beacon.len(), 5);
        assert!(!beacon.is_empty());
        assert!(beacon.verify_chain());
        assert!(beacon.latest_output().is_some());
    }

    #[test]
    fn test_beacon_outputs_advance() {
        let source = Box::new(SeededEntropySource::new(PqHashAlgorithm::Sha512, b"adv"));
        let mut beacon = QuantumRandomBeacon::new(PqHashAlgorithm::Sha512, 16, source);
        let first = beacon.next_round().expect("round").output.clone();
        let second = beacon.next_round().expect("round").output.clone();
        assert_ne!(first, second);
    }

    #[test]
    fn test_beacon_detects_entropy_tamper() {
        let source = Box::new(SeededEntropySource::new(PqHashAlgorithm::Sha256, b"t"));
        let mut beacon = QuantumRandomBeacon::new(PqHashAlgorithm::Sha256, 32, source);
        for _ in 0..3 {
            beacon.next_round().expect("round");
        }
        assert!(beacon.verify_chain());
        // Tamper with a revealed entropy value: commitment check must fail.
        let mut rounds = beacon.rounds().to_vec();
        if let Some(round) = rounds.get_mut(1) {
            round.entropy = vec![0xff; round.entropy.len()];
        }
        assert!(!verify_rounds(PqHashAlgorithm::Sha256, &rounds));
    }

    #[test]
    fn test_beacon_detects_output_tamper() {
        let source = Box::new(SeededEntropySource::new(PqHashAlgorithm::Sha256, b"o"));
        let mut beacon = QuantumRandomBeacon::new(PqHashAlgorithm::Sha256, 32, source);
        for _ in 0..3 {
            beacon.next_round().expect("round");
        }
        let mut rounds = beacon.rounds().to_vec();
        if let Some(round) = rounds.get_mut(2) {
            round.output = vec![0u8; round.output.len()];
        }
        assert!(!verify_rounds(PqHashAlgorithm::Sha256, &rounds));
    }

    #[test]
    fn test_system_source_produces_distinct_rounds() {
        let mut beacon = QuantumRandomBeacon::with_system_source(PqHashAlgorithm::Sha256);
        let a = beacon.next_round().expect("round").output.clone();
        let b = beacon.next_round().expect("round").output.clone();
        assert_ne!(a, b);
        assert!(beacon.verify_chain());
    }
}
