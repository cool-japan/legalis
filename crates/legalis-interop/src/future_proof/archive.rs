//! Long-term preservation archive: a self-describing, BagIt-like container.
//!
//! A [`PreservationArchive`] bundles a legal corpus with everything a future
//! reader needs to trust and migrate it:
//!
//! - a **manifest** of descriptive metadata (title, agent, dates, counts);
//! - the **payload** as lossless [`StructuredStatute`] provenance records;
//! - redundant **fixity** checksums under several quantum-aware algorithms;
//! - a **migration history** recording every format conversion applied;
//! - an optional post-quantum **hash-based signature**; and
//! - a versioned **crypto envelope** describing the protecting algorithms so the
//!   scheme can be upgraded in place.
//!
//! The container serialises to a single self-describing JSON document (the
//! canonical form used by the [`crate::FormatImporter`] / [`crate::FormatExporter`]
//! pipeline) and can additionally be projected to / parsed from a classic
//! [BagIt](https://datatracker.ietf.org/doc/html/rfc8493) file set
//! (`bagit.txt`, `bag-info.txt`, `manifest-<alg>.txt`, `data/`).
//!
//! [`ArchivalStrategy`] presets and [`ArchivalPlan`] provide the
//! "format archival strategies" layer: a policy object (which digests, whether
//! to sign, retention) plus a dry-run plan with quantum-resistance warnings.

use super::agility::{AlgorithmRegistry, CryptoEnvelope, CryptoSuite, SignatureScheme};
use super::checksum::{Checksum, ChecksumAlgorithm, compute_set, verify_set};
use super::hash_sig::{MerklePublicKey, MerkleSignature, MerkleSigner, merkle_verify};
use super::now_rfc3339;
use crate::formats_nextgen::{StructuredStatute, build_structured};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use chrono::{Duration, Utc};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Schema identifier for the preservation archive format.
pub const SCHEMA: &str = "legalis.preservation-archive/v1";

/// BagIt version produced by [`PreservationArchive::to_bagit_files`].
pub const BAGIT_VERSION: &str = "1.0";

/// Name of the software agent recorded in archives.
const SOURCE_SOFTWARE: &str = "legalis-interop";

/// Serialises the structured payload to deterministic, canonical bytes (the
/// bytes fixity and signatures are computed over).
fn payload_bytes(payload: &[StructuredStatute]) -> InteropResult<Vec<u8>> {
    serde_json::to_vec(payload).map_err(|error| {
        InteropError::SerializationError(format!("failed to canonicalise payload: {error}"))
    })
}

/// Descriptive metadata for the archive (maps to BagIt `bag-info.txt`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveManifest {
    /// Human-readable archive title.
    pub title: String,
    /// Producing software name.
    pub source_software: String,
    /// Producing software version.
    pub source_version: String,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// Number of statutes in the payload.
    pub statute_count: usize,
    /// Size of the canonical payload in bytes.
    pub payload_bytes: usize,
    /// Archival profile name (from the [`ArchivalStrategy`]).
    pub profile: String,
    /// Additional free-form metadata.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// A record of one format migration applied during the archive's lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Source format identifier.
    pub from_format: String,
    /// Target format identifier.
    pub to_format: String,
    /// RFC 3339 timestamp of the migration.
    pub performed_at: String,
    /// Tool that performed the migration.
    pub tool: String,
    /// Free-form note (rationale, fidelity remarks).
    pub note: String,
}

impl MigrationRecord {
    /// Creates a migration record stamped with the current time.
    pub fn new(
        from_format: impl Into<String>,
        to_format: impl Into<String>,
        tool: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            from_format: from_format.into(),
            to_format: to_format.into(),
            performed_at: now_rfc3339(),
            tool: tool.into(),
            note: note.into(),
        }
    }
}

/// An embedded cryptographic signature over the canonical payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveSignature {
    /// Signature scheme identifier.
    pub scheme: String,
    /// Public key needed to verify the signature.
    pub public_key: MerklePublicKey,
    /// The hash-based signature value.
    pub signature: MerkleSignature,
    /// A checksum of the exact bytes that were signed (redundant integrity).
    pub signed_digest: Checksum,
}

/// Result of re-checking an archive's fixity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixityReport {
    /// Number of checksums verified.
    pub checked: usize,
    /// Canonical identifiers of any checksums that failed.
    pub failures: Vec<String>,
}

impl FixityReport {
    /// Whether every checksum verified.
    pub fn is_valid(&self) -> bool {
        self.failures.is_empty()
    }
}

/// A self-describing, BagIt-like long-term preservation container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreservationArchive {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// BagIt version of the projected file set.
    pub bagit_version: String,
    /// Descriptive metadata.
    pub manifest: ArchiveManifest,
    /// Versioned crypto envelope describing protecting algorithms.
    pub crypto: CryptoEnvelope,
    /// Lossless statute provenance.
    pub payload: Vec<StructuredStatute>,
    /// Redundant fixity checksums over the canonical payload.
    pub fixity: Vec<Checksum>,
    /// Migration history.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub migrations: Vec<MigrationRecord>,
    /// Optional post-quantum signature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<ArchiveSignature>,
}

impl PreservationArchive {
    /// Builds an unsigned archive from a statute corpus and a strategy.
    pub fn build(statutes: &[Statute], strategy: &ArchivalStrategy) -> InteropResult<Self> {
        let payload = build_structured(statutes);
        let canonical = payload_bytes(&payload)?;
        let algorithms = strategy.effective_algorithms();
        let fixity = compute_set(&algorithms, &canonical);
        let suite = CryptoSuite {
            name: strategy.profile.clone(),
            digest: strategy.strongest_digest().family_id().to_string(),
            signature: strategy
                .signature_scheme
                .map(|s| s.canonical_id().to_string()),
        };
        let manifest = ArchiveManifest {
            title: format!("Legalis preservation archive ({} statutes)", statutes.len()),
            source_software: SOURCE_SOFTWARE.to_string(),
            source_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: now_rfc3339(),
            statute_count: statutes.len(),
            payload_bytes: canonical.len(),
            profile: strategy.profile.clone(),
            extra: BTreeMap::new(),
        };
        Ok(Self {
            schema: SCHEMA.to_string(),
            bagit_version: BAGIT_VERSION.to_string(),
            manifest,
            crypto: suite.to_envelope(),
            payload,
            fixity,
            migrations: Vec::new(),
            signature: None,
        })
    }

    /// The canonical payload bytes (fixity / signature input).
    pub fn canonical_payload_bytes(&self) -> InteropResult<Vec<u8>> {
        payload_bytes(&self.payload)
    }

    /// Reconstructs the underlying statutes.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.payload
            .iter()
            .map(StructuredStatute::to_statute)
            .collect()
    }

    /// Re-verifies all fixity checksums against the current payload.
    pub fn verify_fixity(&self) -> InteropResult<FixityReport> {
        let canonical = self.canonical_payload_bytes()?;
        let failures = verify_set(&self.fixity, &canonical);
        Ok(FixityReport {
            checked: self.fixity.len(),
            failures,
        })
    }

    /// Appends a migration record.
    pub fn record_migration(&mut self, record: MigrationRecord) {
        self.migrations.push(record);
    }

    /// Signs the archive's canonical payload with the given Merkle signer/leaf.
    ///
    /// Only the implemented hash-based scheme is accepted; planned lattice
    /// schemes return [`InteropError::UnsupportedFeature`].
    pub fn sign(
        &mut self,
        signer: &mut MerkleSigner,
        leaf_index: u32,
        scheme: SignatureScheme,
    ) -> InteropResult<()> {
        if !scheme.is_implemented() {
            return Err(InteropError::UnsupportedFeature(format!(
                "signature scheme '{}' is planned/deferred; use 'hash-merkle-lamport-sha256'",
                scheme.canonical_id()
            )));
        }
        let canonical = self.canonical_payload_bytes()?;
        let signature = signer.sign(leaf_index, &canonical)?;
        let signed_digest = Checksum::compute(ChecksumAlgorithm::Sha512, &canonical);
        self.crypto.signature_scheme = Some(scheme.canonical_id().to_string());
        self.signature = Some(ArchiveSignature {
            scheme: scheme.canonical_id().to_string(),
            public_key: signer.public_key(),
            signature,
            signed_digest,
        });
        Ok(())
    }

    /// Verifies the embedded signature (errors if the archive is unsigned or the
    /// scheme is not implemented).
    pub fn verify_signature(&self) -> InteropResult<bool> {
        let signature = self
            .signature
            .as_ref()
            .ok_or_else(|| InteropError::ValidationError("archive is not signed".to_string()))?;
        let scheme = SignatureScheme::from_id(&signature.scheme).ok_or_else(|| {
            InteropError::UnsupportedFeature(format!(
                "unknown signature scheme '{}'",
                signature.scheme
            ))
        })?;
        if !scheme.is_implemented() {
            return Err(InteropError::UnsupportedFeature(format!(
                "signature scheme '{}' is deferred and cannot be verified",
                signature.scheme
            )));
        }
        let canonical = self.canonical_payload_bytes()?;
        if !signature.signed_digest.verify(&canonical) {
            return Ok(false);
        }
        Ok(merkle_verify(
            &canonical,
            &signature.signature,
            &signature.public_key,
        ))
    }

    /// Whether the archive carries a signature.
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    /// Whether all protecting algorithms are quantum-resistant per `registry`.
    pub fn is_quantum_resistant(&self, registry: &AlgorithmRegistry) -> bool {
        self.crypto.is_quantum_resistant(registry)
            && self
                .fixity
                .iter()
                .all(|c| c.algorithm.is_quantum_resistant())
    }

    /// Serialises to pretty JSON (the canonical interchange form).
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("failed to serialise archive: {error}"))
        })
    }

    /// Parses from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("failed to parse archive JSON: {error}"))
        })
    }

    /// Projects the archive to a classic BagIt file set (path -> contents).
    pub fn to_bagit_files(&self) -> InteropResult<BTreeMap<String, String>> {
        let canonical = self.canonical_payload_bytes()?;
        let payload_len = canonical.len();
        let data_file = String::from_utf8(canonical).map_err(|error| {
            InteropError::SerializationError(format!("payload is not valid UTF-8: {error}"))
        })?;
        let mut files = BTreeMap::new();
        files.insert(
            "bagit.txt".to_string(),
            format!("BagIt-Version: {BAGIT_VERSION}\nTag-File-Character-Encoding: UTF-8\n"),
        );
        files.insert(
            "bag-info.txt".to_string(),
            format!(
                "External-Identifier: {}\nExternal-Description: {}\nBagging-Date: {}\n\
                 Source-Organization: {}\nBag-Software-Agent: {} {}\nPayload-Oxum: {}.1\n\
                 Legalis-Statute-Count: {}\nLegalis-Profile: {}\n",
                self.schema,
                self.manifest.title,
                self.manifest.created_at,
                self.manifest.source_software,
                self.manifest.source_software,
                self.manifest.source_version,
                payload_len,
                self.manifest.statute_count,
                self.manifest.profile,
            ),
        );
        files.insert("data/statutes.json".to_string(), data_file);
        for checksum in &self.fixity {
            files.insert(
                format!("manifest-{}.txt", checksum.algorithm_id()),
                format!("{}  data/statutes.json\n", checksum.digest),
            );
        }
        // Full lossless container, retained as a tag file for round-tripping.
        files.insert("legalis-archive.json".to_string(), self.to_json()?);
        let mut tagmanifest = String::new();
        for name in ["bag-info.txt", "bagit.txt", "legalis-archive.json"] {
            if let Some(content) = files.get(name) {
                let digest = Checksum::compute(ChecksumAlgorithm::Sha512, content.as_bytes());
                tagmanifest.push_str(&format!("{}  {}\n", digest.digest, name));
            }
        }
        files.insert("tagmanifest-sha-512.txt".to_string(), tagmanifest);
        Ok(files)
    }

    /// Reconstructs an archive from a BagIt file set produced by
    /// [`PreservationArchive::to_bagit_files`].
    pub fn from_bagit_files(files: &BTreeMap<String, String>) -> InteropResult<Self> {
        let container = files.get("legalis-archive.json").ok_or_else(|| {
            InteropError::ParseError("BagIt set is missing 'legalis-archive.json'".to_string())
        })?;
        Self::from_json(container)
    }
}

/// A long-term preservation policy: which digests to compute, whether to sign,
/// and how long to retain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivalStrategy {
    /// Profile name (also used as the crypto-suite name).
    pub profile: String,
    /// Fixity checksum algorithms to compute (redundant by design).
    pub checksum_algorithms: Vec<ChecksumAlgorithm>,
    /// Signature scheme to apply, if signing is desired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_scheme: Option<SignatureScheme>,
    /// Merkle tree height to use when signing.
    pub merkle_height: u8,
    /// Intended retention period, in years.
    pub retention_years: u32,
    /// Whether the payload is normalised to the canonical provenance form.
    pub normalize_to_canonical: bool,
}

impl Default for ArchivalStrategy {
    fn default() -> Self {
        Self::standard()
    }
}

impl ArchivalStrategy {
    /// A minimal strategy: a single SHA-256 fixity digest, unsigned.
    pub fn minimal() -> Self {
        Self {
            profile: "minimal".to_string(),
            checksum_algorithms: vec![ChecksumAlgorithm::Sha256],
            signature_scheme: None,
            merkle_height: 4,
            retention_years: 7,
            normalize_to_canonical: true,
        }
    }

    /// The default strategy: redundant SHA-256 + SHA-512 fixity, unsigned.
    pub fn standard() -> Self {
        Self {
            profile: "standard".to_string(),
            checksum_algorithms: vec![ChecksumAlgorithm::Sha256, ChecksumAlgorithm::Sha512],
            signature_scheme: None,
            merkle_height: 4,
            retention_years: 30,
            normalize_to_canonical: true,
        }
    }

    /// The maximum-security strategy: SHA-512, an iterated SHA-512, and the
    /// SHA-512‖SHA-256 combiner, plus a hash-based signature.
    pub fn maximum_security() -> Self {
        Self {
            profile: "maximum-security".to_string(),
            checksum_algorithms: vec![
                ChecksumAlgorithm::Sha512,
                ChecksumAlgorithm::IteratedSha512 { rounds: 4 },
                ChecksumAlgorithm::ConcatSha512Sha256,
            ],
            signature_scheme: Some(SignatureScheme::HashMerkleLamportSha256),
            merkle_height: 6,
            retention_years: 100,
            normalize_to_canonical: true,
        }
    }

    /// Returns the configured algorithms, defaulting to SHA-512 if empty.
    pub fn effective_algorithms(&self) -> Vec<ChecksumAlgorithm> {
        if self.checksum_algorithms.is_empty() {
            vec![ChecksumAlgorithm::Sha512]
        } else {
            self.checksum_algorithms.clone()
        }
    }

    /// Returns the strongest configured digest by quantum security.
    pub fn strongest_digest(&self) -> ChecksumAlgorithm {
        self.effective_algorithms()
            .into_iter()
            .max_by_key(|algorithm| algorithm.quantum_preimage_bits())
            .unwrap_or(ChecksumAlgorithm::Sha512)
    }

    /// Produces a dry-run [`ArchivalPlan`] for a corpus, including
    /// quantum-resistance warnings, without building the archive.
    pub fn plan(&self, statutes: &[Statute]) -> InteropResult<ArchivalPlan> {
        let payload = build_structured(statutes);
        let estimated_payload_bytes = payload_bytes(&payload)?.len();
        let registry = AlgorithmRegistry::with_defaults();
        let algorithms = self.effective_algorithms();

        let mut warnings = Vec::new();
        for algorithm in &algorithms {
            if !algorithm.is_quantum_resistant() {
                warnings.push(format!(
                    "fixity algorithm '{}' is not quantum-resistant",
                    algorithm.canonical_id()
                ));
            }
        }
        let mut signing_quantum_safe = true;
        if let Some(scheme) = self.signature_scheme {
            if !scheme.is_implemented() {
                warnings.push(format!(
                    "signature scheme '{}' is deferred and cannot be applied yet",
                    scheme.canonical_id()
                ));
            }
            if !registry.is_quantum_resistant(scheme.canonical_id()) {
                signing_quantum_safe = false;
                warnings.push(format!(
                    "signature scheme '{}' is not quantum-resistant",
                    scheme.canonical_id()
                ));
            }
        }
        if !self.normalize_to_canonical {
            warnings.push(
                "normalize_to_canonical is disabled; original-format fidelity is not guaranteed"
                    .to_string(),
            );
        }

        let quantum_resistant = algorithms
            .iter()
            .all(ChecksumAlgorithm::is_quantum_resistant)
            && signing_quantum_safe;
        let review_interval_years = i64::from((self.retention_years / 3).max(1));
        let recommended_review = (Utc::now() + Duration::days(review_interval_years * 365))
            .date_naive()
            .format("%Y-%m-%d")
            .to_string();

        Ok(ArchivalPlan {
            profile: self.profile.clone(),
            statute_count: statutes.len(),
            estimated_payload_bytes,
            checksum_algorithms: algorithms
                .iter()
                .map(ChecksumAlgorithm::canonical_id)
                .collect(),
            signing_enabled: self.signature_scheme.is_some(),
            signature_scheme: self.signature_scheme.map(|s| s.canonical_id().to_string()),
            retention_years: self.retention_years,
            recommended_review,
            quantum_resistant,
            warnings,
        })
    }
}

/// A dry-run description of what an [`ArchivalStrategy`] would produce.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchivalPlan {
    /// Profile name.
    pub profile: String,
    /// Number of statutes.
    pub statute_count: usize,
    /// Estimated canonical payload size in bytes.
    pub estimated_payload_bytes: usize,
    /// Canonical ids of the fixity algorithms.
    pub checksum_algorithms: Vec<String>,
    /// Whether signing is configured.
    pub signing_enabled: bool,
    /// The signature scheme id, if configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_scheme: Option<String>,
    /// Retention period in years.
    pub retention_years: u32,
    /// Recommended date (YYYY-MM-DD) for the next preservation review.
    pub recommended_review: String,
    /// Whether the plan is fully quantum-resistant.
    pub quantum_resistant: bool,
    /// Warnings about the strategy.
    pub warnings: Vec<String>,
}

/// Importer for the preservation archive format.
#[derive(Debug, Default)]
pub struct PreservationArchiveImporter;

impl PreservationArchiveImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for PreservationArchiveImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::PreservationArchive
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let archive = PreservationArchive::from_json(source)?;
        let mut report =
            ConversionReport::new(LegalFormat::PreservationArchive, LegalFormat::Legalis);

        let fixity = archive.verify_fixity()?;
        if !fixity.is_valid() {
            report.add_warning(format!(
                "fixity verification failed for: {}",
                fixity.failures.join(", ")
            ));
        }
        if archive.is_signed() {
            match archive.verify_signature() {
                Ok(true) => {}
                Ok(false) => report.add_warning("archive signature is INVALID"),
                Err(error) => {
                    report.add_warning(format!("archive signature could not be verified: {error}"))
                }
            }
        }

        let statutes = archive.to_statutes();
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(source)
            .ok()
            .and_then(|value| {
                value
                    .get("schema")
                    .and_then(|schema| schema.as_str())
                    .map(|schema| schema == SCHEMA)
            })
            .unwrap_or(false)
    }
}

/// Exporter for the preservation archive format.
#[derive(Debug, Clone)]
pub struct PreservationArchiveExporter {
    strategy: ArchivalStrategy,
    signer_seed: Option<[u8; 32]>,
    leaf_index: u32,
}

impl PreservationArchiveExporter {
    /// Creates an exporter with the default (standard) strategy, unsigned.
    pub fn new() -> Self {
        Self {
            strategy: ArchivalStrategy::standard(),
            signer_seed: None,
            leaf_index: 0,
        }
    }

    /// Sets the archival strategy.
    pub fn with_strategy(mut self, strategy: ArchivalStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Enables signing with the given 32-byte master seed and leaf index. The
    /// Merkle height is taken from the strategy.
    pub fn with_signer(mut self, seed: [u8; 32], leaf_index: u32) -> Self {
        self.signer_seed = Some(seed);
        self.leaf_index = leaf_index;
        self
    }
}

impl Default for PreservationArchiveExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for PreservationArchiveExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::PreservationArchive
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut archive = PreservationArchive::build(statutes, &self.strategy)?;
        let mut report =
            ConversionReport::new(LegalFormat::Legalis, LegalFormat::PreservationArchive);
        report.statutes_converted = statutes.len();

        if let Some(seed) = self.signer_seed {
            let scheme = self
                .strategy
                .signature_scheme
                .unwrap_or(SignatureScheme::HashMerkleLamportSha256);
            if scheme.is_implemented() {
                let mut signer = MerkleSigner::from_seed(seed, self.strategy.merkle_height)?;
                archive.sign(&mut signer, self.leaf_index, scheme)?;
            } else {
                report.add_warning(format!(
                    "signature scheme '{}' is deferred; archive left unsigned",
                    scheme.canonical_id()
                ));
            }
        } else if self.strategy.signature_scheme.is_some() {
            report.add_warning(
                "strategy requests signing but no signer seed was provided; archive left unsigned",
            );
        }

        let json = archive.to_json()?;
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::future_proof::hash_sig::{MerkleSigner, seed_from_bytes};
    use crate::{LegalConverter, LegalFormat};
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statutes() -> Vec<Statute> {
        vec![
            Statute::new(
                "voting-rights",
                "Voting Rights",
                Effect::new(EffectType::Grant, "Grant the right to vote"),
            )
            .with_jurisdiction("US")
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_applies_to("Citizen"),
            Statute::new(
                "tax-duty",
                "Tax Duty",
                Effect::new(EffectType::Obligation, "Pay income tax"),
            )
            .with_jurisdiction("US")
            .with_precondition(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 10_000,
            }),
        ]
    }

    #[test]
    fn test_build_and_verify_fixity() {
        let archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::standard()).expect("build");
        assert_eq!(archive.schema, SCHEMA);
        assert_eq!(archive.manifest.statute_count, 2);
        assert_eq!(archive.fixity.len(), 2);
        let report = archive.verify_fixity().expect("fixity");
        assert!(report.is_valid());
        assert_eq!(report.checked, 2);

        let reconstructed = archive.to_statutes();
        assert_eq!(reconstructed.len(), 2);
        assert_eq!(reconstructed[0].id, "voting-rights");
        assert_eq!(reconstructed[1].jurisdiction.as_deref(), Some("US"));
    }

    #[test]
    fn test_fixity_detects_payload_tampering() {
        let mut archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::maximum_security())
                .expect("build");
        assert!(archive.verify_fixity().expect("fixity").is_valid());
        // Mutate the payload after fixity was computed.
        archive.payload[0].title = "Tampered".to_string();
        let report = archive.verify_fixity().expect("fixity");
        assert!(!report.is_valid());
        assert_eq!(report.failures.len(), 3);
    }

    #[test]
    fn test_sign_and_verify_signature() {
        let mut archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::standard()).expect("build");
        let mut signer =
            MerkleSigner::from_seed(seed_from_bytes(b"archive-key"), 3).expect("signer");
        archive
            .sign(&mut signer, 0, SignatureScheme::HashMerkleLamportSha256)
            .expect("sign");
        assert!(archive.is_signed());
        assert!(archive.verify_signature().expect("verify"));
        assert_eq!(
            archive.crypto.signature_scheme.as_deref(),
            Some("hash-merkle-lamport-sha256")
        );

        // Tampering invalidates the signature.
        archive.payload[0].effect_description = "altered".to_string();
        assert!(!archive.verify_signature().expect("verify"));
    }

    #[test]
    fn test_deferred_scheme_is_rejected() {
        let mut archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::standard()).expect("build");
        let mut signer = MerkleSigner::from_seed(seed_from_bytes(b"k"), 2).expect("signer");
        let result = archive.sign(&mut signer, 0, SignatureScheme::MlDsa65);
        assert!(matches!(result, Err(InteropError::UnsupportedFeature(_))));
        assert!(!archive.is_signed());
    }

    #[test]
    fn test_json_roundtrip_preserves_signature() {
        let mut archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::standard()).expect("build");
        let mut signer = MerkleSigner::from_seed(seed_from_bytes(b"json-key"), 2).expect("signer");
        archive
            .sign(&mut signer, 1, SignatureScheme::HashMerkleLamportSha256)
            .expect("sign");
        let json = archive.to_json().expect("to_json");
        let back = PreservationArchive::from_json(&json).expect("from_json");
        assert_eq!(archive, back);
        assert!(back.verify_signature().expect("verify"));
        assert!(back.verify_fixity().expect("fixity").is_valid());
    }

    #[test]
    fn test_migration_history() {
        let mut archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::minimal()).expect("build");
        archive.record_migration(MigrationRecord::new(
            "akoma-ntoso",
            "preservation-archive",
            "legalis-interop",
            "normalised legacy XML to canonical provenance",
        ));
        assert_eq!(archive.migrations.len(), 1);
        let json = archive.to_json().expect("json");
        let back = PreservationArchive::from_json(&json).expect("parse");
        assert_eq!(back.migrations[0].from_format, "akoma-ntoso");
    }

    #[test]
    fn test_bagit_roundtrip_on_filesystem() {
        let archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::standard()).expect("build");
        let files = archive.to_bagit_files().expect("bagit");
        assert!(files.contains_key("bagit.txt"));
        assert!(files.contains_key("bag-info.txt"));
        assert!(files.contains_key("data/statutes.json"));
        assert!(files.contains_key("manifest-sha-512.txt"));

        // Write the bag to a temp directory and read it back.
        let dir = std::env::temp_dir().join(format!("legalis-bagit-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (name, content) in &files {
            let path = dir.join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("mkdir");
            }
            std::fs::write(&path, content).expect("write");
        }
        let mut loaded = BTreeMap::new();
        for name in files.keys() {
            let content = std::fs::read_to_string(dir.join(name)).expect("read");
            loaded.insert(name.clone(), content);
        }
        let restored = PreservationArchive::from_bagit_files(&loaded).expect("from bagit");
        assert_eq!(restored, archive);
        assert!(restored.verify_fixity().expect("fixity").is_valid());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_strategy_presets_and_plan() {
        assert_eq!(ArchivalStrategy::default(), ArchivalStrategy::standard());
        assert_eq!(ArchivalStrategy::minimal().checksum_algorithms.len(), 1);
        assert_eq!(
            ArchivalStrategy::maximum_security()
                .checksum_algorithms
                .len(),
            3
        );

        let plan = ArchivalStrategy::maximum_security()
            .plan(&statutes())
            .expect("plan");
        assert_eq!(plan.statute_count, 2);
        assert!(plan.estimated_payload_bytes > 0);
        assert!(plan.signing_enabled);
        assert!(plan.quantum_resistant);
        assert!(plan.warnings.is_empty());
        assert_eq!(plan.recommended_review.len(), 10);

        // A strategy requesting a deferred scheme produces a warning.
        let mut deferred = ArchivalStrategy::standard();
        deferred.signature_scheme = Some(SignatureScheme::MlDsa65);
        let plan = deferred.plan(&statutes()).expect("plan");
        assert!(plan.warnings.iter().any(|w| w.contains("deferred")));
    }

    #[test]
    fn test_quantum_resistance_flag() {
        let registry = AlgorithmRegistry::with_defaults();
        let archive =
            PreservationArchive::build(&statutes(), &ArchivalStrategy::maximum_security())
                .expect("build");
        assert!(archive.is_quantum_resistant(&registry));
    }

    #[test]
    fn test_importer_exporter_trait_roundtrip() {
        let exporter = PreservationArchiveExporter::new();
        let importer = PreservationArchiveImporter::new();
        let (json, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 2);
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"other\"}"));

        let (imported, import_report) = importer.import(&json).expect("import");
        assert_eq!(import_report.statutes_converted, 2);
        assert_eq!(imported.len(), 2);
        // A clean archive imports without fixity/signature warnings.
        assert!(import_report.warnings.is_empty());
    }

    #[test]
    fn test_signed_exporter_imports_cleanly() {
        let exporter = PreservationArchiveExporter::new()
            .with_strategy(ArchivalStrategy::maximum_security())
            .with_signer(seed_from_bytes(b"exporter-seed"), 0);
        let (json, _) = exporter.export(&statutes()).expect("export");
        let archive = PreservationArchive::from_json(&json).expect("parse");
        assert!(archive.is_signed());
        assert!(archive.verify_signature().expect("verify"));

        let importer = PreservationArchiveImporter::new();
        let (_, report) = importer.import(&json).expect("import");
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_converter_level_integration() {
        let mut converter = LegalConverter::new();
        assert!(
            converter
                .supported_exports()
                .contains(&LegalFormat::PreservationArchive)
        );
        assert!(
            converter
                .supported_imports()
                .contains(&LegalFormat::PreservationArchive)
        );

        let (json, export_report) = converter
            .export(&statutes(), LegalFormat::PreservationArchive)
            .expect("export");
        assert_eq!(export_report.statutes_converted, 2);

        let (imported, import_report) = converter
            .import(&json, LegalFormat::PreservationArchive)
            .expect("import");
        assert_eq!(import_report.statutes_converted, 2);
        assert_eq!(imported.len(), 2);

        // Auto-detection picks the schema-tagged archive.
        let (auto, auto_report) = converter.auto_import(&json).expect("auto");
        assert_eq!(
            auto_report.source_format,
            Some(LegalFormat::PreservationArchive)
        );
        assert_eq!(auto.len(), 2);
    }
}
