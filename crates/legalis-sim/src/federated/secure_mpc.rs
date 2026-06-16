//! Secure multi-party computation primitives for federated simulation.
//!
//! This submodule implements information-theoretically secure **additive secret
//! sharing** over a finite field, together with a [`SecureAggregator`] that
//! performs secure aggregation (the summation step of federated averaging)
//! without any single party learning another party's input.
//!
//! Real values are mapped into the field with a signed fixed-point
//! [`FieldEncoder`]; the field is the Mersenne prime `2^61 - 1`, which keeps all
//! modular arithmetic exact inside `u128` accumulators while leaving ample head
//! room for summing many encoded vectors.

use crate::error::{SimResult, SimulationError};
use rand::RngExt;
use serde::{Deserialize, Serialize};

/// Prime modulus of the field used for secret sharing (`2^61 - 1`, a Mersenne prime).
pub const FIELD_PRIME: u64 = 2_305_843_009_213_693_951;

/// Modular subtraction `(a - b) mod FIELD_PRIME`, assuming `a, b < FIELD_PRIME`.
fn mod_sub(a: u64, b: u64) -> u64 {
    ((a as u128 + FIELD_PRIME as u128 - b as u128) % FIELD_PRIME as u128) as u64
}

/// Signed fixed-point encoder mapping `f64` values onto field elements.
///
/// A value `v` is encoded as `round(v * scale) mod p`. Negative values map onto
/// the upper half of the field, so that decoding interprets any element greater
/// than `p / 2` as negative. This makes addition in the field correspond to
/// addition of the original (signed) fixed-point numbers, provided the magnitude
/// of the true scaled sum stays below `p / 2`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FieldEncoder {
    /// Fixed-point scaling factor (precision is `1 / scale`).
    scale: f64,
}

impl FieldEncoder {
    /// Creates a new encoder with the given fixed-point `scale` (must be positive).
    pub fn new(scale: f64) -> SimResult<Self> {
        if scale <= 0.0 || !scale.is_finite() {
            return Err(SimulationError::InvalidParameter(
                "fixed-point scale must be positive and finite".to_string(),
            ));
        }
        Ok(Self { scale })
    }

    /// Returns the fixed-point scale.
    pub fn scale(&self) -> f64 {
        self.scale
    }

    /// Encodes a real value as a field element.
    pub fn encode(&self, value: f64) -> u64 {
        let scaled = (value * self.scale).round() as i128;
        scaled.rem_euclid(FIELD_PRIME as i128) as u64
    }

    /// Decodes a field element back into a real value.
    pub fn decode(&self, element: u64) -> f64 {
        let p = FIELD_PRIME as i128;
        let signed = if element as i128 > p / 2 {
            element as i128 - p
        } else {
            element as i128
        };
        signed as f64 / self.scale
    }
}

/// A single additive secret share held by one party.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretShare {
    /// Identifier of the party that holds this share.
    pub party_id: usize,
    /// The share value, an element of the field.
    pub value: u64,
}

/// Additive secret sharing over the field [`FIELD_PRIME`].
///
/// A secret `s` is split into `n` shares `s_0, ..., s_{n-1}` whose sum modulo the
/// field prime equals `s`. Any subset of fewer than `n` shares is uniformly
/// random and reveals nothing about `s`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AdditiveSecretSharing {
    /// Number of parties among which secrets are shared.
    num_parties: usize,
}

impl AdditiveSecretSharing {
    /// Creates a sharing scheme for `num_parties` parties (at least two).
    pub fn new(num_parties: usize) -> SimResult<Self> {
        if num_parties < 2 {
            return Err(SimulationError::InvalidParameter(
                "additive secret sharing requires at least 2 parties".to_string(),
            ));
        }
        Ok(Self { num_parties })
    }

    /// Returns the number of parties.
    pub fn num_parties(&self) -> usize {
        self.num_parties
    }

    /// Splits `secret` into one share per party using uniformly random shares.
    pub fn share<R: RngExt>(&self, secret: u64, rng: &mut R) -> Vec<SecretShare> {
        let secret = secret % FIELD_PRIME;
        let mut shares = Vec::with_capacity(self.num_parties);
        let mut acc: u128 = 0;

        // The first n-1 shares are uniformly random field elements.
        for party_id in 0..self.num_parties - 1 {
            let value: u64 = rng.random_range(0..FIELD_PRIME);
            acc = (acc + value as u128) % FIELD_PRIME as u128;
            shares.push(SecretShare { party_id, value });
        }

        // The final share is chosen so that all shares sum to the secret.
        let last = mod_sub(secret, acc as u64);
        shares.push(SecretShare {
            party_id: self.num_parties - 1,
            value: last,
        });

        shares
    }

    /// Reconstructs a secret from a complete set of shares (sum modulo the prime).
    pub fn reconstruct(&self, shares: &[SecretShare]) -> u64 {
        let mut acc: u128 = 0;
        for share in shares {
            acc = (acc + share.value as u128) % FIELD_PRIME as u128;
        }
        acc as u64
    }
}

/// Secure aggregator computing the sum of per-party inputs without revealing them.
///
/// The aggregator simulates the standard additive secure-aggregation protocol:
/// every party splits its (encoded) input into one share per party and sends each
/// share to its destination; every party then sums the shares it received into a
/// *partial sum*; finally the partial sums are combined into the field sum, which
/// decodes to the aggregate. No party's individual input is ever exposed — only
/// the final aggregate.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SecureAggregator {
    /// Number of participating parties.
    num_parties: usize,
    /// Fixed-point encoder shared by all parties.
    encoder: FieldEncoder,
    /// Underlying secret-sharing scheme.
    sharing: AdditiveSecretSharing,
}

impl SecureAggregator {
    /// Creates a secure aggregator for `num_parties` parties with fixed-point `scale`.
    pub fn new(num_parties: usize, scale: f64) -> SimResult<Self> {
        Ok(Self {
            num_parties,
            encoder: FieldEncoder::new(scale)?,
            sharing: AdditiveSecretSharing::new(num_parties)?,
        })
    }

    /// Returns the number of parties.
    pub fn num_parties(&self) -> usize {
        self.num_parties
    }

    /// Runs the secure-aggregation protocol over one scalar per party.
    ///
    /// Returns the field element representing the (encoded) sum.
    fn secure_sum<R: RngExt>(&self, values: &[f64], rng: &mut R) -> u64 {
        // partials[j] is the partial sum accumulated by party j.
        let mut partials = vec![0u128; self.num_parties];

        for &value in values {
            let secret = self.encoder.encode(value);
            let shares = self.sharing.share(secret, rng);
            for share in &shares {
                partials[share.party_id] =
                    (partials[share.party_id] + share.value as u128) % FIELD_PRIME as u128;
            }
        }

        let mut total: u128 = 0;
        for partial in partials {
            total = (total + partial) % FIELD_PRIME as u128;
        }
        total as u64
    }

    /// Securely aggregates one scalar per party, returning their sum.
    pub fn aggregate_scalars<R: RngExt>(&self, values: &[f64], rng: &mut R) -> SimResult<f64> {
        if values.len() != self.num_parties {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} scalar inputs, got {}",
                self.num_parties,
                values.len()
            )));
        }
        Ok(self.encoder.decode(self.secure_sum(values, rng)))
    }

    /// Securely aggregates one vector per party, returning their element-wise sum.
    pub fn aggregate_vectors<R: RngExt>(
        &self,
        vectors: &[Vec<f64>],
        rng: &mut R,
    ) -> SimResult<Vec<f64>> {
        if vectors.len() != self.num_parties {
            return Err(SimulationError::InvalidParameter(format!(
                "expected {} vector inputs, got {}",
                self.num_parties,
                vectors.len()
            )));
        }

        let dim = vectors.first().map(Vec::len).unwrap_or(0);
        if vectors.iter().any(|v| v.len() != dim) {
            return Err(SimulationError::InvalidParameter(
                "all party vectors must have the same dimension".to_string(),
            ));
        }

        let mut result = Vec::with_capacity(dim);
        for k in 0..dim {
            let column: Vec<f64> = vectors.iter().map(|v| v[k]).collect();
            result.push(self.encoder.decode(self.secure_sum(&column, rng)));
        }
        Ok(result)
    }

    /// Securely computes the mean of one scalar per party.
    pub fn secure_mean<R: RngExt>(&self, values: &[f64], rng: &mut R) -> SimResult<f64> {
        let sum = self.aggregate_scalars(values, rng)?;
        Ok(sum / self.num_parties as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn test_field_encoder_roundtrip() {
        let encoder = FieldEncoder::new(65536.0).unwrap();
        for &value in &[0.0, 1.0, -1.0, 3.5, -2.25, 1000.5, -500.25] {
            let decoded = encoder.decode(encoder.encode(value));
            assert!((decoded - value).abs() < 1e-3, "value {value} -> {decoded}");
        }
    }

    #[test]
    fn test_field_encoder_rejects_bad_scale() {
        assert!(FieldEncoder::new(0.0).is_err());
        assert!(FieldEncoder::new(-1.0).is_err());
        assert!(FieldEncoder::new(f64::NAN).is_err());
    }

    #[test]
    fn test_secret_sharing_reconstruct() {
        let mut rng = StdRng::seed_from_u64(7);
        let sharing = AdditiveSecretSharing::new(5).unwrap();
        let secret = 123_456_789u64;
        let shares = sharing.share(secret, &mut rng);
        assert_eq!(shares.len(), 5);
        assert_eq!(sharing.reconstruct(&shares), secret % FIELD_PRIME);
    }

    #[test]
    fn test_secret_shares_hide_secret() {
        let mut rng = StdRng::seed_from_u64(11);
        let sharing = AdditiveSecretSharing::new(4).unwrap();
        let secret = 42u64;
        let shares = sharing.share(secret, &mut rng);
        // No individual share equals the (tiny) secret; the first n-1 are random.
        let revealing = shares.iter().filter(|s| s.value == secret).count();
        assert_eq!(revealing, 0);
    }

    #[test]
    fn test_sharing_requires_two_parties() {
        assert!(AdditiveSecretSharing::new(1).is_err());
        assert!(AdditiveSecretSharing::new(0).is_err());
        assert!(AdditiveSecretSharing::new(2).is_ok());
    }

    #[test]
    fn test_secure_aggregate_scalars() {
        let mut rng = StdRng::seed_from_u64(99);
        let values = vec![10.0, 20.5, -5.25, 7.75];
        let aggregator = SecureAggregator::new(values.len(), 65536.0).unwrap();
        let secure = aggregator.aggregate_scalars(&values, &mut rng).unwrap();
        let plain: f64 = values.iter().sum();
        assert!(
            (secure - plain).abs() < 1e-2,
            "secure {secure} vs plain {plain}"
        );
    }

    #[test]
    fn test_secure_aggregate_vectors() {
        let mut rng = StdRng::seed_from_u64(123);
        let vectors = vec![
            vec![1.0, 2.0, 3.0],
            vec![0.5, -1.0, 4.0],
            vec![-2.0, 3.5, 1.0],
        ];
        let aggregator = SecureAggregator::new(vectors.len(), 65536.0).unwrap();
        let secure = aggregator.aggregate_vectors(&vectors, &mut rng).unwrap();
        let expected = [1.0 + 0.5 - 2.0, 2.0 - 1.0 + 3.5, 3.0 + 4.0 + 1.0];
        for (s, e) in secure.iter().zip(expected.iter()) {
            assert!((s - e).abs() < 1e-2, "secure {s} vs expected {e}");
        }
    }

    #[test]
    fn test_secure_mean() {
        let mut rng = StdRng::seed_from_u64(321);
        let values = vec![4.0, 8.0, 12.0, 16.0];
        let aggregator = SecureAggregator::new(values.len(), 65536.0).unwrap();
        let mean = aggregator.secure_mean(&values, &mut rng).unwrap();
        assert!((mean - 10.0).abs() < 1e-2);
    }

    #[test]
    fn test_secure_aggregate_wrong_length_err() {
        let mut rng = StdRng::seed_from_u64(5);
        let aggregator = SecureAggregator::new(3, 65536.0).unwrap();
        assert!(aggregator.aggregate_scalars(&[1.0, 2.0], &mut rng).is_err());
        assert!(
            aggregator
                .aggregate_vectors(&[vec![1.0], vec![2.0]], &mut rng)
                .is_err()
        );
    }

    #[test]
    fn test_secure_aggregate_mismatched_vector_dim_err() {
        let mut rng = StdRng::seed_from_u64(8);
        let aggregator = SecureAggregator::new(2, 65536.0).unwrap();
        let vectors = vec![vec![1.0, 2.0], vec![3.0]];
        assert!(aggregator.aggregate_vectors(&vectors, &mut rng).is_err());
    }
}
