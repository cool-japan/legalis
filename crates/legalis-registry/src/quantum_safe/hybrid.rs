//! Hybrid classical + post-quantum signature envelopes for registry records.
//!
//! During the migration to post-quantum cryptography, a *hybrid* construction
//! provides defence-in-depth: a record is protected by two independent
//! mechanisms so that a break of either one alone does not forge it. This module
//! binds:
//!
//! - a **classical** symmetric authentication tag (HMAC-SHA-256 over a
//!   length-prefixed commitment to the statute id, version and content digest),
//!   and
//! - the **post-quantum** hash-based [`SignedStatute`] signature.
//!
//! A configurable [`HybridPolicy`] decides which layers must pass for the
//! envelope to be accepted, letting deployments tighten from "either" to "both"
//! as they gain confidence in the PQ layer.
//!
//! # Threat model note
//!
//! The classical layer here is a *symmetric* MAC (a shared secret authenticates
//! and verifies). A classical *asymmetric* signature (RSA / ECDSA) is **not**
//! bundled, because no audited pure-Rust implementation is available without a
//! new heavy dependency; it is catalogued as deprecated/deferred in
//! [`super::agility`]. The PQ layer supplies the asymmetric, publicly verifiable
//! guarantee, while the QKD-derivable classical key (see [`super::qkd`]) supplies
//! an information-theoretically keyed integrity layer.

use super::signatures::{SignedStatute, content_digest};
use super::{constant_time_eq, hmac_sha256, now_rfc3339, to_hex};
use crate::{RegistryResult, StatuteEntry};
use serde::{Deserialize, Serialize};

/// Identifier of the classical MAC algorithm used by the hybrid envelope.
pub const CLASSICAL_ALGORITHM_ID: &str = "hmac-sha256";

/// Acceptance policy controlling which layers must verify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HybridPolicy {
    /// Both the classical and post-quantum layers must verify (strongest;
    /// defence-in-depth).
    RequireBoth,
    /// Only the post-quantum layer must verify (forward-secure against quantum
    /// adversaries even if the shared classical key leaks).
    RequireQuantum,
    /// Only the classical layer must verify (legacy interop; not quantum-safe).
    RequireClassical,
    /// Either layer verifying is sufficient (most permissive; useful during a
    /// staged rollout).
    RequireEither,
}

impl HybridPolicy {
    /// Resolves acceptance from the two layer outcomes.
    #[must_use]
    pub fn accepts(&self, classical_ok: bool, quantum_ok: bool) -> bool {
        match self {
            HybridPolicy::RequireBoth => classical_ok && quantum_ok,
            HybridPolicy::RequireQuantum => quantum_ok,
            HybridPolicy::RequireClassical => classical_ok,
            HybridPolicy::RequireEither => classical_ok || quantum_ok,
        }
    }
}

/// Builds the length-prefixed classical MAC message binding id, version and the
/// hex content digest.
fn classical_message(statute_id: &str, version: u32, content_hash_hex: &str) -> Vec<u8> {
    let mut message = Vec::with_capacity(statute_id.len() + content_hash_hex.len() + 24);
    message.extend_from_slice(&(statute_id.len() as u64).to_be_bytes());
    message.extend_from_slice(statute_id.as_bytes());
    message.extend_from_slice(&version.to_be_bytes());
    message.extend_from_slice(&(content_hash_hex.len() as u64).to_be_bytes());
    message.extend_from_slice(content_hash_hex.as_bytes());
    message
}

/// A hybrid classical + post-quantum signature over a statute record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridSignatureEnvelope {
    /// Identifier of the signed statute.
    pub statute_id: String,
    /// Version of the entry that was signed.
    pub version: u32,
    /// Classical MAC algorithm identifier (always [`CLASSICAL_ALGORITHM_ID`]).
    pub classical_algorithm: String,
    /// Classical HMAC-SHA-256 tag (lowercase hex).
    pub classical_tag: String,
    /// The post-quantum hash-based signature.
    pub quantum: SignedStatute,
    /// Acceptance policy for verification.
    pub policy: HybridPolicy,
    /// RFC 3339 timestamp of sealing.
    pub created_at: String,
}

impl HybridSignatureEnvelope {
    /// Seals a post-quantum [`SignedStatute`] into a hybrid envelope by adding a
    /// classical HMAC tag keyed by `classical_key`.
    #[must_use]
    pub fn seal(quantum: SignedStatute, classical_key: &[u8], policy: HybridPolicy) -> Self {
        let message =
            classical_message(&quantum.statute_id, quantum.version, &quantum.content_hash);
        let tag = hmac_sha256(classical_key, &message);
        Self {
            statute_id: quantum.statute_id.clone(),
            version: quantum.version,
            classical_algorithm: CLASSICAL_ALGORITHM_ID.to_string(),
            classical_tag: to_hex(&tag),
            quantum,
            policy,
            created_at: now_rfc3339(),
        }
    }

    /// Verifies the classical layer against `entry` and `classical_key`.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify_classical(
        &self,
        entry: &StatuteEntry,
        classical_key: &[u8],
    ) -> RegistryResult<bool> {
        if entry.statute.id != self.statute_id || entry.version != self.version {
            return Ok(false);
        }
        let digest = content_digest(entry)?;
        let message = classical_message(&self.statute_id, self.version, &to_hex(&digest));
        let expected = to_hex(&hmac_sha256(classical_key, &message));
        Ok(constant_time_eq(
            expected.as_bytes(),
            self.classical_tag.as_bytes(),
        ))
    }

    /// Verifies both layers and resolves acceptance under the embedded policy.
    ///
    /// # Errors
    ///
    /// Propagates canonicalization failures.
    pub fn verify(
        &self,
        entry: &StatuteEntry,
        classical_key: &[u8],
    ) -> RegistryResult<HybridVerification> {
        let classical_ok = self.verify_classical(entry, classical_key)?;
        let quantum_ok = self.quantum.verify(entry)?;
        Ok(HybridVerification {
            classical_ok,
            quantum_ok,
            policy: self.policy,
            accepted: self.policy.accepts(classical_ok, quantum_ok),
        })
    }

    /// Whether the post-quantum layer alone makes this envelope quantum-safe.
    #[must_use]
    pub fn is_quantum_safe(&self) -> bool {
        !matches!(self.policy, HybridPolicy::RequireClassical)
    }
}

/// The per-layer outcome of verifying a [`HybridSignatureEnvelope`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridVerification {
    /// Whether the classical MAC layer verified.
    pub classical_ok: bool,
    /// Whether the post-quantum signature layer verified.
    pub quantum_ok: bool,
    /// The policy that was applied.
    pub policy: HybridPolicy,
    /// Whether the envelope is accepted under the policy.
    pub accepted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum_safe::StatuteSigner;
    use legalis_core::{Effect, EffectType, Statute};

    fn entry(id: &str, title: &str) -> StatuteEntry {
        let statute = Statute::new(id, title, Effect::new(EffectType::Grant, "grant"));
        StatuteEntry::new(statute, "US")
    }

    fn sealed(policy: HybridPolicy, key: &[u8]) -> (HybridSignatureEnvelope, StatuteEntry) {
        let mut signer = StatuteSigner::from_seed([8u8; 32], 3).expect("signer");
        let item = entry("act-1", "An Act");
        let signed = signer.sign_entry(0, &item).expect("sign");
        (HybridSignatureEnvelope::seal(signed, key, policy), item)
    }

    #[test]
    fn test_policy_acceptance_matrix() {
        assert!(HybridPolicy::RequireBoth.accepts(true, true));
        assert!(!HybridPolicy::RequireBoth.accepts(true, false));
        assert!(HybridPolicy::RequireQuantum.accepts(false, true));
        assert!(!HybridPolicy::RequireQuantum.accepts(true, false));
        assert!(HybridPolicy::RequireClassical.accepts(true, false));
        assert!(HybridPolicy::RequireEither.accepts(false, true));
        assert!(!HybridPolicy::RequireEither.accepts(false, false));
    }

    #[test]
    fn test_hybrid_both_layers_verify() {
        let key = [0x11u8; 32];
        let (envelope, item) = sealed(HybridPolicy::RequireBoth, &key);
        assert_eq!(envelope.classical_algorithm, CLASSICAL_ALGORITHM_ID);
        let verification = envelope.verify(&item, &key).expect("verify");
        assert!(verification.classical_ok);
        assert!(verification.quantum_ok);
        assert!(verification.accepted);
        assert!(envelope.is_quantum_safe());
    }

    #[test]
    fn test_wrong_classical_key_fails_under_require_both() {
        let key = [0x11u8; 32];
        let (envelope, item) = sealed(HybridPolicy::RequireBoth, &key);
        let verification = envelope.verify(&item, &[0x22u8; 32]).expect("verify");
        assert!(!verification.classical_ok);
        assert!(verification.quantum_ok);
        // RequireBoth rejects when the classical layer fails.
        assert!(!verification.accepted);
    }

    #[test]
    fn test_require_quantum_tolerates_wrong_classical_key() {
        let key = [0x11u8; 32];
        let (envelope, item) = sealed(HybridPolicy::RequireQuantum, &key);
        // Even with the wrong classical key, the PQ layer carries acceptance.
        let verification = envelope.verify(&item, &[0x22u8; 32]).expect("verify");
        assert!(!verification.classical_ok);
        assert!(verification.quantum_ok);
        assert!(verification.accepted);
    }

    #[test]
    fn test_tampered_content_fails_both_layers() {
        let key = [0x11u8; 32];
        let (envelope, item) = sealed(HybridPolicy::RequireEither, &key);
        let mut tampered = item.clone();
        tampered.statute.title = "Tampered".to_string();
        let verification = envelope.verify(&tampered, &key).expect("verify");
        assert!(!verification.classical_ok);
        assert!(!verification.quantum_ok);
        assert!(!verification.accepted);
    }

    #[test]
    fn test_envelope_serde_roundtrip() {
        let key = [0x33u8; 32];
        let (envelope, item) = sealed(HybridPolicy::RequireBoth, &key);
        let json = serde_json::to_string(&envelope).expect("ser");
        let back: HybridSignatureEnvelope = serde_json::from_str(&json).expect("de");
        assert_eq!(envelope, back);
        assert!(back.verify(&item, &key).expect("verify").accepted);
    }
}
