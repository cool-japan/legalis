//! A BB84-style quantum key distribution (QKD) model expressed as data
//! structures.
//!
//! This is a **simulation / model**, not a hardware driver: it reproduces the
//! information flow of the BB84 protocol (Bennett & Brassard, 1984) so the
//! registry can reason about, test, and derive shared key material for the
//! [`super::hybrid`] classical layer. It is fully deterministic from a 32-byte
//! seed (no `rand` dependency): all of Alice's bits/bases, Bob's measurement
//! bases and any eavesdropper choices are expanded from the seed via a
//! domain-separated PRF, so a session is byte-for-byte reproducible.
//!
//! The modelled flow:
//! 1. Alice picks a random bit and a random preparation [`Basis`] per photon.
//! 2. Bob picks a random measurement basis per photon. When his basis matches
//!    Alice's he recovers her bit; otherwise the outcome is random.
//! 3. **Sifting**: the parties keep only the positions where their bases agreed.
//! 4. **Error estimation**: a sample of the sifted bits is sacrificed to
//!    estimate the quantum bit-error rate (QBER). A QBER above the abort
//!    threshold signals eavesdropping and the key is rejected.
//! 5. **Key**: the surviving sifted bits are privacy-amplified (hashed) into a
//!    32-byte symmetric key.
//!
//! An optional intercept-resend [`Bb84Config::eavesdropper`] models Eve, who
//! measures and resends in a random basis, injecting the characteristic ~25%
//! error rate on the sifted bits that BB84 is designed to detect.
//!
//! This complements (does not implement) the lattice KEM ML-KEM, which is
//! deferred in [`super::agility`].

use super::{tagged_hash, to_hex};
use serde::{Deserialize, Serialize};

/// The standard BB84 QBER abort threshold (~11%): above this, eavesdropping is
/// assumed and the key is discarded.
pub const DEFAULT_ABORT_THRESHOLD: f64 = 0.11;

const CTX_ALICE_BIT: &[u8] = b"qkd-alice-bit";
const CTX_ALICE_BASIS: &[u8] = b"qkd-alice-basis";
const CTX_BOB_BASIS: &[u8] = b"qkd-bob-basis";
const CTX_BOB_RANDOM: &[u8] = b"qkd-bob-random";
const CTX_EVE_BASIS: &[u8] = b"qkd-eve-basis";
const CTX_EVE_RANDOM: &[u8] = b"qkd-eve-random";
const CTX_KEY_MATERIAL: &[u8] = b"qkd-key-material";

/// A polarization measurement basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Basis {
    /// Rectilinear basis (+): horizontal / vertical.
    Rectilinear,
    /// Diagonal basis (x): 45° / 135°.
    Diagonal,
}

impl Basis {
    fn from_bit(bit: u8) -> Self {
        if bit & 1 == 0 {
            Basis::Rectilinear
        } else {
            Basis::Diagonal
        }
    }
}

/// Deterministically derives a single PRF bit for photon `index` under a context.
fn prf_bit(seed: &[u8; 32], context: &[u8], index: usize) -> u8 {
    tagged_hash(context, &[seed, &(index as u64).to_be_bytes()])[0] & 1
}

/// Configuration for a BB84 session.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bb84Config {
    /// Number of photons (qubits) Alice transmits.
    pub photon_count: usize,
    /// Fraction of the sifted bits sacrificed to estimate the QBER (clamped to
    /// `0.0..=1.0`).
    pub sample_fraction: f64,
    /// Whether an intercept-resend eavesdropper (Eve) is present.
    pub eavesdropper: bool,
    /// QBER above which the key is aborted as compromised.
    pub abort_threshold: f64,
}

impl Default for Bb84Config {
    fn default() -> Self {
        Self {
            photon_count: 256,
            sample_fraction: 0.25,
            eavesdropper: false,
            abort_threshold: DEFAULT_ABORT_THRESHOLD,
        }
    }
}

impl Bb84Config {
    /// Creates a config with a given photon count, default sampling, no
    /// eavesdropper.
    #[must_use]
    pub fn with_photons(photon_count: usize) -> Self {
        Self {
            photon_count,
            ..Self::default()
        }
    }

    /// Enables or disables the modelled eavesdropper.
    #[must_use]
    pub fn with_eavesdropper(mut self, present: bool) -> Self {
        self.eavesdropper = present;
        self
    }

    /// Sets the QBER sampling fraction (clamped to `0.0..=1.0`).
    #[must_use]
    pub fn with_sample_fraction(mut self, fraction: f64) -> Self {
        self.sample_fraction = fraction.clamp(0.0, 1.0);
        self
    }
}

/// The outcome of the BB84 error-estimation / eavesdropper-detection phase.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EavesdropAssessment {
    /// Number of sifted bits compared during estimation.
    pub sample_size: usize,
    /// Number of disagreements observed in the sample.
    pub mismatches: usize,
    /// Estimated quantum bit-error rate (`mismatches / sample_size`).
    pub estimated_qber: f64,
    /// The abort threshold that was applied.
    pub abort_threshold: f64,
    /// Whether the QBER exceeded the threshold (key rejected).
    pub aborted: bool,
    /// Whether an eavesdropper was modelled in this run (ground truth).
    pub eavesdropper_modelled: bool,
}

impl EavesdropAssessment {
    /// Whether the channel is considered secure (not aborted).
    #[must_use]
    pub fn is_secure(&self) -> bool {
        !self.aborted
    }
}

/// A fully expanded BB84 session: the raw bits/bases, the sifted positions, the
/// eavesdropper assessment and the derived shared key bits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bb84Session {
    /// The configuration used.
    pub config: Bb84Config,
    /// Alice's prepared bits, one per photon.
    pub alice_bits: Vec<u8>,
    /// Alice's preparation bases.
    pub alice_bases: Vec<Basis>,
    /// Bob's measurement bases.
    pub bob_bases: Vec<Basis>,
    /// Bob's measured bits.
    pub bob_bits: Vec<u8>,
    /// Indices where Alice's and Bob's bases agreed (the sifted positions).
    pub sifted_indices: Vec<usize>,
    /// Sifted indices sacrificed to estimate the QBER.
    pub sample_indices: Vec<usize>,
    /// The eavesdropper assessment.
    pub assessment: EavesdropAssessment,
    /// Alice's surviving (post-sample) sifted bits — the raw shared key.
    pub key_bits: Vec<u8>,
}

impl Bb84Session {
    /// Runs a deterministic BB84 session from a 32-byte `seed` and `config`.
    #[must_use]
    pub fn simulate(seed: [u8; 32], config: Bb84Config) -> Self {
        let count = config.photon_count;
        let mut alice_bits = Vec::with_capacity(count);
        let mut alice_bases = Vec::with_capacity(count);
        let mut bob_bases = Vec::with_capacity(count);
        let mut bob_bits = Vec::with_capacity(count);

        for index in 0..count {
            let a_bit = prf_bit(&seed, CTX_ALICE_BIT, index);
            let a_basis = Basis::from_bit(prf_bit(&seed, CTX_ALICE_BASIS, index));
            let b_basis = Basis::from_bit(prf_bit(&seed, CTX_BOB_BASIS, index));

            let b_bit = if config.eavesdropper {
                Self::eavesdropped_outcome(&seed, index, a_bit, a_basis, b_basis)
            } else if a_basis == b_basis {
                a_bit
            } else {
                prf_bit(&seed, CTX_BOB_RANDOM, index)
            };

            alice_bits.push(a_bit);
            alice_bases.push(a_basis);
            bob_bases.push(b_basis);
            bob_bits.push(b_bit);
        }

        let sifted_indices: Vec<usize> = (0..count)
            .filter(|&i| alice_bases[i] == bob_bases[i])
            .collect();

        let sample_count = ((sifted_indices.len() as f64) * config.sample_fraction.clamp(0.0, 1.0))
            .floor() as usize;
        let sample_indices: Vec<usize> =
            sifted_indices.iter().take(sample_count).copied().collect();

        let mismatches = sample_indices
            .iter()
            .filter(|&&i| alice_bits[i] != bob_bits[i])
            .count();
        let estimated_qber = if sample_indices.is_empty() {
            0.0
        } else {
            mismatches as f64 / sample_indices.len() as f64
        };
        let aborted = estimated_qber > config.abort_threshold;

        let key_bits: Vec<u8> = sifted_indices
            .iter()
            .skip(sample_count)
            .map(|&i| alice_bits[i])
            .collect();

        Bb84Session {
            config,
            alice_bits,
            alice_bases,
            bob_bases,
            bob_bits,
            sifted_indices,
            sample_indices,
            assessment: EavesdropAssessment {
                sample_size: sample_count,
                mismatches,
                estimated_qber,
                abort_threshold: config.abort_threshold,
                aborted,
                eavesdropper_modelled: config.eavesdropper,
            },
            key_bits,
        }
    }

    /// Models Bob's measured bit under an intercept-resend eavesdropper.
    fn eavesdropped_outcome(
        seed: &[u8; 32],
        index: usize,
        a_bit: u8,
        a_basis: Basis,
        b_basis: Basis,
    ) -> u8 {
        let eve_basis = Basis::from_bit(prf_bit(seed, CTX_EVE_BASIS, index));
        // Eve measures Alice's qubit: correct bit if she guessed the basis,
        // otherwise a random outcome. She resends in her own basis.
        let eve_bit = if eve_basis == a_basis {
            a_bit
        } else {
            prf_bit(seed, CTX_EVE_RANDOM, index)
        };
        // Bob measures Eve's resent qubit (prepared in eve_basis with eve_bit).
        if b_basis == eve_basis {
            eve_bit
        } else {
            prf_bit(seed, CTX_BOB_RANDOM, index)
        }
    }

    /// Number of positions that survived sifting.
    #[must_use]
    pub fn sifted_len(&self) -> usize {
        self.sifted_indices.len()
    }

    /// Number of raw key bits (post-sampling).
    #[must_use]
    pub fn key_len(&self) -> usize {
        self.key_bits.len()
    }

    /// Whether any key material was produced.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.key_bits.is_empty()
    }

    /// Bob's view of the surviving key bits (post-sample sifted positions).
    ///
    /// In a noiseless, eavesdropper-free run this equals [`Bb84Session::key_bits`];
    /// any difference reveals tampering or basis-mismatch noise.
    #[must_use]
    pub fn bob_key_bits(&self) -> Vec<u8> {
        let sample_count = self.sample_indices.len();
        self.sifted_indices
            .iter()
            .skip(sample_count)
            .map(|&i| self.bob_bits[i])
            .collect()
    }

    /// Whether Alice's and Bob's surviving key bits agree exactly.
    #[must_use]
    pub fn keys_match(&self) -> bool {
        self.key_bits == self.bob_key_bits()
    }

    /// Privacy-amplifies the surviving key bits into a 32-byte symmetric key,
    /// suitable for keying the [`super::hybrid`] classical layer.
    ///
    /// Returns `None` if the session was aborted or produced no key bits.
    #[must_use]
    pub fn derive_key_material(&self) -> Option<[u8; 32]> {
        if self.assessment.aborted || self.key_bits.is_empty() {
            return None;
        }
        Some(tagged_hash(CTX_KEY_MATERIAL, &[&self.key_bits]))
    }

    /// The derived key material as lowercase hex, if any.
    #[must_use]
    pub fn key_material_hex(&self) -> Option<String> {
        self.derive_key_material().map(|key| to_hex(&key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed(label: u8) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0] = label;
        bytes[17] = label.wrapping_mul(3);
        bytes
    }

    #[test]
    fn test_session_is_deterministic() {
        let config = Bb84Config::with_photons(128);
        let a = Bb84Session::simulate(seed(1), config);
        let b = Bb84Session::simulate(seed(1), config);
        assert_eq!(a, b);
        let c = Bb84Session::simulate(seed(2), config);
        assert_ne!(a.alice_bits, c.alice_bits);
    }

    #[test]
    fn test_noiseless_run_agrees_and_is_secure() {
        let config = Bb84Config::with_photons(256);
        let session = Bb84Session::simulate(seed(3), config);
        // Roughly half the photons survive sifting (bases agree ~50%).
        assert!(session.sifted_len() > 0);
        assert!(session.sifted_len() < 256);
        // Without an eavesdropper, the QBER is zero and the keys agree exactly.
        assert_eq!(session.assessment.estimated_qber, 0.0);
        assert!(session.assessment.is_secure());
        assert!(session.keys_match());
        assert!(!session.is_empty());
    }

    #[test]
    fn test_eavesdropper_raises_qber_and_aborts() {
        let config = Bb84Config::with_photons(512).with_eavesdropper(true);
        let session = Bb84Session::simulate(seed(4), config);
        assert!(session.assessment.eavesdropper_modelled);
        // Intercept-resend injects ~25% error on the sifted bits — well above the
        // ~11% abort threshold.
        assert!(session.assessment.estimated_qber > config.abort_threshold);
        assert!(session.assessment.aborted);
        assert!(!session.assessment.is_secure());
        // An aborted session yields no usable key material.
        assert!(session.derive_key_material().is_none());
    }

    #[test]
    fn test_key_material_derivation_and_hex() {
        let config = Bb84Config::with_photons(256);
        let session = Bb84Session::simulate(seed(5), config);
        let key = session.derive_key_material().expect("key material");
        assert_eq!(key.len(), 32);
        assert_eq!(session.key_material_hex().expect("hex").len(), 64);
        // Deterministic: same session derives the same key.
        let again = Bb84Session::simulate(seed(5), config)
            .derive_key_material()
            .expect("again");
        assert_eq!(key, again);
    }

    #[test]
    fn test_config_builders_clamp() {
        let config = Bb84Config::with_photons(10)
            .with_sample_fraction(5.0)
            .with_eavesdropper(true);
        assert_eq!(config.sample_fraction, 1.0);
        assert!(config.eavesdropper);
        let serialized = serde_json::to_string(&config).expect("ser");
        let back: Bb84Config = serde_json::from_str(&serialized).expect("de");
        assert_eq!(config, back);
    }
}
