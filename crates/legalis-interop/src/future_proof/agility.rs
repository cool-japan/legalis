//! Cryptographic agility: an algorithm registry and a versioned crypto envelope.
//!
//! "Cryptographic agility" is the ability to change the cryptographic primitives
//! protecting an artifact without redesigning the artifact. This module provides
//! the two pieces needed for long-lived legal documents:
//!
//! - [`AlgorithmRegistry`] — a catalogue of digest, signature and KEM algorithms
//!   annotated with classical/quantum security levels and a life-cycle
//!   [`AlgorithmStatus`]. It can answer "is this algorithm still safe?", "what
//!   should I migrate to?" and "which post-quantum algorithms are planned?".
//! - [`CryptoEnvelope`] — a versioned header naming the digest and signature
//!   algorithms that protect a payload. Because the envelope is self-describing,
//!   a verifier always knows which scheme to use, and [`CryptoEnvelope::upgraded`]
//!   can rewrite a weak envelope to recommended algorithms in place.
//!
//! Lattice schemes (ML-DSA / ML-KEM / SLH-DSA) are registered with
//! [`AlgorithmStatus::Planned`] and are intentionally **not** implemented here,
//! to avoid pulling in heavy non-pure-Rust dependencies; the implemented
//! post-quantum option is the hash-based scheme in [`super::hash_sig`].

use super::checksum::ChecksumAlgorithm;
use super::now_rfc3339;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Current version of the [`CryptoEnvelope`] header format.
pub const CURRENT_ENVELOPE_VERSION: u32 = 1;

/// The category an algorithm belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlgorithmKind {
    /// Hash / checksum function.
    Digest,
    /// Digital signature scheme.
    Signature,
    /// Key-encapsulation mechanism.
    Kem,
}

/// Life-cycle status of an algorithm for long-term preservation purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlgorithmStatus {
    /// Preferred for new artifacts.
    Recommended,
    /// Safe to verify and acceptable for new artifacts, but not preferred.
    Acceptable,
    /// Must be migrated away from; do not use for new artifacts.
    Deprecated,
    /// Known and reserved, but not yet implemented in this crate.
    Planned,
    /// Implemented but experimental; not for production preservation.
    Experimental,
}

/// A catalogue entry describing one cryptographic algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorithmDescriptor {
    /// Stable registry identifier.
    pub id: String,
    /// Algorithm category.
    pub kind: AlgorithmKind,
    /// Human-readable name.
    pub display_name: String,
    /// Pre-image / forgery security against a classical adversary, in bits.
    pub classical_security_bits: u32,
    /// Security against a quantum adversary (Grover/Shor accounted), in bits.
    pub quantum_security_bits: u32,
    /// Whether the algorithm retains usable strength against a quantum adversary.
    pub quantum_resistant: bool,
    /// Life-cycle status.
    pub status: AlgorithmStatus,
    /// Free-form note (rationale, standard reference, deferral reason).
    pub note: String,
}

impl AlgorithmDescriptor {
    /// Returns `true` if this algorithm should be migrated away from (deprecated
    /// or not quantum-resistant).
    pub fn needs_migration(&self) -> bool {
        matches!(self.status, AlgorithmStatus::Deprecated) || !self.quantum_resistant
    }
}

/// A pluggable, post-quantum-aware signature scheme identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureScheme {
    /// Hash-based Merkle signature with Lamport one-time leaves over SHA-256.
    /// Implemented in [`super::hash_sig`].
    HashMerkleLamportSha256,
    /// ML-DSA (CRYSTALS-Dilithium), FIPS 204. Planned; deferred.
    MlDsa65,
    /// SLH-DSA (SPHINCS+), FIPS 205. Planned; deferred.
    SlhDsaSha2128s,
}

impl SignatureScheme {
    /// The stable registry identifier.
    pub fn canonical_id(&self) -> &'static str {
        match self {
            SignatureScheme::HashMerkleLamportSha256 => "hash-merkle-lamport-sha256",
            SignatureScheme::MlDsa65 => "ml-dsa-65",
            SignatureScheme::SlhDsaSha2128s => "slh-dsa-sha2-128s",
        }
    }

    /// Whether this crate actually implements signing/verification for the
    /// scheme. Only the hash-based scheme is implemented; lattice/stateless
    /// schemes are deferred.
    pub fn is_implemented(&self) -> bool {
        matches!(self, SignatureScheme::HashMerkleLamportSha256)
    }

    /// All listed schemes are post-quantum.
    pub fn is_quantum_resistant(&self) -> bool {
        true
    }

    /// Parses a scheme from its canonical identifier.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "hash-merkle-lamport-sha256" => Some(SignatureScheme::HashMerkleLamportSha256),
            "ml-dsa-65" => Some(SignatureScheme::MlDsa65),
            "slh-dsa-sha2-128s" => Some(SignatureScheme::SlhDsaSha2128s),
            _ => None,
        }
    }
}

/// A catalogue of cryptographic algorithms with their security properties.
#[derive(Debug, Clone, Default)]
pub struct AlgorithmRegistry {
    entries: BTreeMap<String, AlgorithmDescriptor>,
}

impl AlgorithmRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry pre-populated with the algorithms this crate knows
    /// about (implemented and planned).
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        let digests = [
            (
                ChecksumAlgorithm::Sha256,
                "SHA-256",
                256,
                128,
                AlgorithmStatus::Acceptable,
                "256-bit; ~128-bit post-quantum pre-image security (Grover).",
            ),
            (
                ChecksumAlgorithm::Sha512,
                "SHA-512",
                512,
                256,
                AlgorithmStatus::Recommended,
                "512-bit; ~256-bit post-quantum pre-image security.",
            ),
            (
                ChecksumAlgorithm::Sha512Trunc256,
                "SHA-512/256",
                256,
                128,
                AlgorithmStatus::Acceptable,
                "Truncated SHA-512; fast on 64-bit hosts.",
            ),
            (
                ChecksumAlgorithm::IteratedSha512 { rounds: 1 },
                "Iterated SHA-512",
                512,
                256,
                AlgorithmStatus::Recommended,
                "Iterated SHA-512 adds a tunable work factor; 512-bit output.",
            ),
            (
                ChecksumAlgorithm::ConcatSha512Sha256,
                "SHA-512 ‖ SHA-256",
                512,
                256,
                AlgorithmStatus::Recommended,
                "Concatenation combiner; collision-resistant if either component is.",
            ),
        ];
        for (algorithm, name, classical, quantum, status, note) in digests {
            registry.register(AlgorithmDescriptor {
                id: algorithm.family_id().to_string(),
                kind: AlgorithmKind::Digest,
                display_name: name.to_string(),
                classical_security_bits: classical,
                quantum_security_bits: quantum,
                quantum_resistant: quantum >= 128,
                status,
                note: note.to_string(),
            });
        }

        // A genuinely broken legacy digest, registered so the agility layer can
        // detect and migrate away from it. It is metadata only (this crate does
        // not compute SHA-1).
        registry.register(AlgorithmDescriptor {
            id: "sha-1".to_string(),
            kind: AlgorithmKind::Digest,
            display_name: "SHA-1".to_string(),
            classical_security_bits: 80,
            quantum_security_bits: 0,
            quantum_resistant: false,
            status: AlgorithmStatus::Deprecated,
            note: "Collision-broken (SHAttered, 2017); never use for new fixity.".to_string(),
        });

        registry.register(AlgorithmDescriptor {
            id: SignatureScheme::HashMerkleLamportSha256
                .canonical_id()
                .to_string(),
            kind: AlgorithmKind::Signature,
            display_name: "Hash-based Merkle (Lamport/SHA-256)".to_string(),
            classical_security_bits: 256,
            quantum_security_bits: 128,
            quantum_resistant: true,
            status: AlgorithmStatus::Recommended,
            note: "Stateful hash-based signature; one signature per Merkle leaf.".to_string(),
        });
        registry.register(AlgorithmDescriptor {
            id: SignatureScheme::MlDsa65.canonical_id().to_string(),
            kind: AlgorithmKind::Signature,
            display_name: "ML-DSA-65 (CRYSTALS-Dilithium)".to_string(),
            classical_security_bits: 192,
            quantum_security_bits: 192,
            quantum_resistant: true,
            status: AlgorithmStatus::Planned,
            note: "FIPS 204 lattice signature; deferred (no pure-Rust impl bundled).".to_string(),
        });
        registry.register(AlgorithmDescriptor {
            id: SignatureScheme::SlhDsaSha2128s.canonical_id().to_string(),
            kind: AlgorithmKind::Signature,
            display_name: "SLH-DSA-SHA2-128s (SPHINCS+)".to_string(),
            classical_security_bits: 128,
            quantum_security_bits: 128,
            quantum_resistant: true,
            status: AlgorithmStatus::Planned,
            note: "FIPS 205 stateless hash-based signature; deferred.".to_string(),
        });
        registry.register(AlgorithmDescriptor {
            id: "ed25519".to_string(),
            kind: AlgorithmKind::Signature,
            display_name: "Ed25519".to_string(),
            classical_security_bits: 128,
            quantum_security_bits: 0,
            quantum_resistant: false,
            status: AlgorithmStatus::Deprecated,
            note: "Elliptic-curve signature; broken by Shor's algorithm. Migrate to a PQ scheme."
                .to_string(),
        });
        registry.register(AlgorithmDescriptor {
            id: "ml-kem-768".to_string(),
            kind: AlgorithmKind::Kem,
            display_name: "ML-KEM-768 (CRYSTALS-Kyber)".to_string(),
            classical_security_bits: 192,
            quantum_security_bits: 192,
            quantum_resistant: true,
            status: AlgorithmStatus::Planned,
            note: "FIPS 203 key-encapsulation mechanism; deferred.".to_string(),
        });
        registry
    }

    /// Registers (or replaces) a descriptor.
    pub fn register(&mut self, descriptor: AlgorithmDescriptor) {
        self.entries.insert(descriptor.id.clone(), descriptor);
    }

    /// Looks up a descriptor by identifier.
    pub fn get(&self, id: &str) -> Option<&AlgorithmDescriptor> {
        self.entries.get(id)
    }

    /// Whether an identifier is registered.
    pub fn contains(&self, id: &str) -> bool {
        self.entries.contains_key(id)
    }

    /// Number of registered algorithms.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// All descriptors of a given kind, in identifier order.
    pub fn list(&self, kind: AlgorithmKind) -> Vec<&AlgorithmDescriptor> {
        self.entries
            .values()
            .filter(|descriptor| descriptor.kind == kind)
            .collect()
    }

    /// All algorithms still awaiting implementation.
    pub fn planned(&self) -> Vec<&AlgorithmDescriptor> {
        self.entries
            .values()
            .filter(|descriptor| descriptor.status == AlgorithmStatus::Planned)
            .collect()
    }

    /// Whether the identified algorithm is registered and quantum-resistant.
    pub fn is_quantum_resistant(&self, id: &str) -> bool {
        self.get(id).map(|d| d.quantum_resistant).unwrap_or(false)
    }

    /// The recommended algorithm of a kind: the quantum-resistant,
    /// [`AlgorithmStatus::Recommended`] descriptor with the highest quantum
    /// security level.
    pub fn recommended(&self, kind: AlgorithmKind) -> Option<&AlgorithmDescriptor> {
        self.entries
            .values()
            .filter(|d| {
                d.kind == kind
                    && d.quantum_resistant
                    && matches!(d.status, AlgorithmStatus::Recommended)
            })
            .max_by_key(|d| d.quantum_security_bits)
    }

    /// Suggests a migration target for `id`: if it is unknown, deprecated, or
    /// not quantum-resistant, returns the recommended algorithm of its kind.
    /// Returns `None` when no migration is needed.
    pub fn migration_target(&self, id: &str) -> Option<String> {
        match self.get(id) {
            Some(descriptor) if !descriptor.needs_migration() => None,
            Some(descriptor) => self.recommended(descriptor.kind).map(|d| d.id.clone()),
            // Unknown algorithm: default to the recommended signature/digest is
            // ambiguous, so only suggest when the kind can be inferred. We try
            // digest then signature.
            None => self
                .recommended(AlgorithmKind::Digest)
                .or_else(|| self.recommended(AlgorithmKind::Signature))
                .map(|d| d.id.clone()),
        }
    }
}

/// A named bundle of a digest algorithm and an optional signature scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptoSuite {
    /// Suite name.
    pub name: String,
    /// Digest algorithm family identifier.
    pub digest: String,
    /// Signature scheme identifier, if the suite is signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl CryptoSuite {
    /// The recommended post-quantum suite: a SHA-512‖SHA-256 combiner digest and
    /// the hash-based Merkle signature.
    pub fn quantum_safe_v1() -> Self {
        Self {
            name: "legalis-pq-v1".to_string(),
            digest: ChecksumAlgorithm::ConcatSha512Sha256
                .family_id()
                .to_string(),
            signature: Some(
                SignatureScheme::HashMerkleLamportSha256
                    .canonical_id()
                    .to_string(),
            ),
        }
    }

    /// A legacy classical suite (SHA-1 + Ed25519). Useful only to demonstrate
    /// migration; both the digest and the signature scheme are obsolete.
    pub fn classical_v1() -> Self {
        Self {
            name: "legalis-classical-v1".to_string(),
            digest: "sha-1".to_string(),
            signature: Some("ed25519".to_string()),
        }
    }

    /// Builds a fresh [`CryptoEnvelope`] for this suite, timestamped now.
    pub fn to_envelope(&self) -> CryptoEnvelope {
        CryptoEnvelope {
            envelope_version: CURRENT_ENVELOPE_VERSION,
            suite: self.name.clone(),
            digest_algorithm: self.digest.clone(),
            signature_scheme: self.signature.clone(),
            created_at: now_rfc3339(),
            notes: Vec::new(),
        }
    }
}

/// A versioned, self-describing header naming the algorithms that protect a
/// payload, enabling in-place cryptographic upgrades.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptoEnvelope {
    /// Version of this envelope format.
    pub envelope_version: u32,
    /// Name of the crypto suite in effect.
    pub suite: String,
    /// Digest algorithm family identifier protecting payload fixity.
    pub digest_algorithm: String,
    /// Signature scheme identifier, if the payload is signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_scheme: Option<String>,
    /// RFC 3339 creation/upgrade timestamp.
    pub created_at: String,
    /// Free-form notes, including a migration audit trail.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

impl CryptoEnvelope {
    /// Whether every algorithm named by the envelope is registered and
    /// quantum-resistant.
    pub fn is_quantum_resistant(&self, registry: &AlgorithmRegistry) -> bool {
        if !registry.is_quantum_resistant(&self.digest_algorithm) {
            return false;
        }
        match &self.signature_scheme {
            Some(scheme) => registry.is_quantum_resistant(scheme),
            None => true,
        }
    }

    /// Returns descriptions of any weak or unknown algorithms the envelope names.
    pub fn weaknesses(&self, registry: &AlgorithmRegistry) -> Vec<String> {
        let mut issues = Vec::new();
        match registry.get(&self.digest_algorithm) {
            None => issues.push(format!(
                "unknown digest algorithm '{}'",
                self.digest_algorithm
            )),
            Some(descriptor) if descriptor.needs_migration() => issues.push(format!(
                "digest '{}' should be migrated",
                self.digest_algorithm
            )),
            Some(_) => {}
        }
        if let Some(scheme) = &self.signature_scheme {
            match registry.get(scheme) {
                None => issues.push(format!("unknown signature scheme '{scheme}'")),
                Some(descriptor) if descriptor.needs_migration() => {
                    issues.push(format!("signature '{scheme}' should be migrated"))
                }
                Some(_) => {}
            }
        }
        issues
    }

    /// Produces an upgraded envelope: any weak digest/signature is replaced by
    /// the registry's recommended algorithm, the version is bumped, and an audit
    /// note is appended. Strong envelopes are returned essentially unchanged
    /// (only re-timestamped and version-bumped).
    pub fn upgraded(&self, registry: &AlgorithmRegistry) -> CryptoEnvelope {
        let mut next = self.clone();
        next.envelope_version = self.envelope_version + 1;
        if registry.migration_target(&self.digest_algorithm).is_some()
            && let Some(recommended) = registry.recommended(AlgorithmKind::Digest)
        {
            next.notes.push(format!(
                "digest upgraded {} -> {}",
                self.digest_algorithm, recommended.id
            ));
            next.digest_algorithm = recommended.id.clone();
        }
        if let Some(scheme) = self.signature_scheme.clone()
            && registry.migration_target(&scheme).is_some()
            && let Some(recommended) = registry.recommended(AlgorithmKind::Signature)
        {
            next.notes.push(format!(
                "signature upgraded {} -> {}",
                scheme, recommended.id
            ));
            next.signature_scheme = Some(recommended.id.clone());
        }
        next.created_at = now_rfc3339();
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_defaults_present() {
        let registry = AlgorithmRegistry::with_defaults();
        assert!(!registry.is_empty());
        assert!(registry.contains("sha-512"));
        assert!(registry.contains("concat-sha-512+sha-256"));
        assert!(registry.contains("hash-merkle-lamport-sha256"));
        assert!(registry.contains("ml-dsa-65"));
        assert!(registry.contains("ml-kem-768"));
        assert!(registry.is_quantum_resistant("sha-512"));
        assert!(!registry.is_quantum_resistant("ed25519"));
        assert!(!registry.is_quantum_resistant("does-not-exist"));
        let digest = registry.get("sha-512").expect("sha-512");
        assert_eq!(digest.kind, AlgorithmKind::Digest);
        assert_eq!(digest.quantum_security_bits, 256);
    }

    #[test]
    fn test_recommended_and_migration_target() {
        let registry = AlgorithmRegistry::with_defaults();
        let digest = registry.recommended(AlgorithmKind::Digest).expect("digest");
        assert!(digest.quantum_resistant);
        assert_eq!(digest.status, AlgorithmStatus::Recommended);
        assert_eq!(digest.quantum_security_bits, 256);

        let signature = registry
            .recommended(AlgorithmKind::Signature)
            .expect("signature");
        assert_eq!(signature.id, "hash-merkle-lamport-sha256");

        // Deprecated/non-PQ algorithm yields a migration target.
        assert_eq!(
            registry.migration_target("ed25519").as_deref(),
            Some("hash-merkle-lamport-sha256")
        );
        // Strong algorithm needs none.
        assert!(registry.migration_target("sha-512").is_none());
        // Unknown algorithm yields a sensible default.
        assert!(registry.migration_target("unknown-hash").is_some());
    }

    #[test]
    fn test_planned_schemes_listed_but_not_implemented() {
        let registry = AlgorithmRegistry::with_defaults();
        let planned: Vec<&str> = registry.planned().iter().map(|d| d.id.as_str()).collect();
        assert!(planned.contains(&"ml-dsa-65"));
        assert!(planned.contains(&"ml-kem-768"));
        assert!(planned.contains(&"slh-dsa-sha2-128s"));

        assert!(SignatureScheme::HashMerkleLamportSha256.is_implemented());
        assert!(!SignatureScheme::MlDsa65.is_implemented());
        assert!(SignatureScheme::MlDsa65.is_quantum_resistant());
        assert_eq!(
            SignatureScheme::from_id("ml-dsa-65"),
            Some(SignatureScheme::MlDsa65)
        );
        assert_eq!(SignatureScheme::from_id("nonexistent"), None);
    }

    #[test]
    fn test_envelope_quantum_resistance_check() {
        let registry = AlgorithmRegistry::with_defaults();
        let pq = CryptoSuite::quantum_safe_v1().to_envelope();
        assert!(pq.is_quantum_resistant(&registry));
        assert!(pq.weaknesses(&registry).is_empty());

        let classical = CryptoSuite::classical_v1().to_envelope();
        assert!(!classical.is_quantum_resistant(&registry));
        assert!(!classical.weaknesses(&registry).is_empty());
    }

    #[test]
    fn test_envelope_upgrade_replaces_weak_algorithms() {
        let registry = AlgorithmRegistry::with_defaults();
        let classical = CryptoSuite::classical_v1().to_envelope();
        assert_eq!(classical.envelope_version, CURRENT_ENVELOPE_VERSION);

        let upgraded = classical.upgraded(&registry);
        assert_eq!(upgraded.envelope_version, CURRENT_ENVELOPE_VERSION + 1);
        assert!(upgraded.is_quantum_resistant(&registry));
        assert_ne!(upgraded.digest_algorithm, classical.digest_algorithm);
        assert_eq!(
            upgraded.signature_scheme.as_deref(),
            Some("hash-merkle-lamport-sha256")
        );
        assert!(upgraded.notes.iter().any(|note| note.contains("upgraded")));

        // Upgrading an already-strong envelope keeps its algorithms.
        let strong = CryptoSuite::quantum_safe_v1().to_envelope();
        let strong_upgraded = strong.upgraded(&registry);
        assert_eq!(strong_upgraded.digest_algorithm, strong.digest_algorithm);
        assert_eq!(strong_upgraded.signature_scheme, strong.signature_scheme);
    }

    #[test]
    fn test_custom_registration() {
        let mut registry = AlgorithmRegistry::new();
        assert!(registry.is_empty());
        registry.register(AlgorithmDescriptor {
            id: "custom-digest".to_string(),
            kind: AlgorithmKind::Digest,
            display_name: "Custom".to_string(),
            classical_security_bits: 512,
            quantum_security_bits: 256,
            quantum_resistant: true,
            status: AlgorithmStatus::Experimental,
            note: String::new(),
        });
        assert_eq!(registry.len(), 1);
        assert!(registry.contains("custom-digest"));
        assert_eq!(registry.list(AlgorithmKind::Digest).len(), 1);
        assert_eq!(registry.list(AlgorithmKind::Signature).len(), 0);
    }
}
