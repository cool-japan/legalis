//! Cross-border digital notarization for ports.
//!
//! A [`DigitalNotary`] is bound to a jurisdiction and holds private key material.
//! It produces [`NotarySignature`]s — *signatures as data* — over a document
//! hash (typically a [`crate::blockchain::PortingLedgerRecord`]'s content hash).
//! A [`CrossBorderNotarization`] aggregates signatures and is considered
//! *complete* only once notaries from **both** the source and target
//! jurisdictions have attested, modelling the two-sided acknowledgement a
//! statute needs when it crosses a border.
//!
//! # Attestation scheme
//!
//! Attestations are keyed-hash commitments: a seal is
//! `SHA-256(secret ‖ document_hash)`, and the notary's public [`PartyId`] is a
//! one-way function of the same secret. This is self-contained and
//! tamper-evident — altering the document hash invalidates every seal — but it is
//! *symmetric*: authenticity verification requires the signing key, so it is
//! performed by the issuing [`NotaryRegistry`]. Substituting a public-key
//! signature scheme (Ed25519/secp256k1) only changes how a seal is produced and
//! verified; the [`NotarySignature`] data shape and all callers are unaffected.
//! That asymmetric binding is intentionally deferred.

use super::{PartyId, current_timestamp, sha256_parts};
use crate::PortingError;
use crate::blockchain::PortingLedgerRecord;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

type NotaryResult<T> = Result<T, PortingError>;

/// The attestation scheme used to produce a [`NotarySignature`] seal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureScheme {
    /// `SHA-256(secret ‖ document_hash)` keyed-hash commitment.
    KeyedSha256,
}

/// Computes a notary seal over a document hash with the notary's secret.
fn compute_seal(secret: &[u8], document_hash: &str) -> String {
    sha256_parts(&[secret, document_hash.as_bytes()])
}

/// A notary attestation, shareable as data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotarySignature {
    /// The signing notary's public identifier.
    pub signer: String,
    /// The jurisdiction the signing notary represents.
    pub jurisdiction: String,
    /// The document hash that was attested.
    pub document_hash: String,
    /// The keyed-hash seal binding the notary to the document hash.
    pub seal: String,
    /// UNIX timestamp (seconds) the attestation was produced.
    pub timestamp: u64,
    /// The scheme used to produce [`NotarySignature::seal`].
    pub scheme: SignatureScheme,
}

impl NotarySignature {
    /// Whether this signature was made over `document_hash` (tamper check that
    /// needs no key).
    pub fn covers(&self, document_hash: &str) -> bool {
        self.document_hash == document_hash
    }
}

/// A jurisdiction-bound digital notary holding private key material.
///
/// `Debug` is implemented manually to keep the secret out of formatted output.
#[derive(Clone)]
pub struct DigitalNotary {
    name: String,
    jurisdiction: String,
    id: PartyId,
    secret: Vec<u8>,
}

impl std::fmt::Debug for DigitalNotary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DigitalNotary")
            .field("name", &self.name)
            .field("jurisdiction", &self.jurisdiction)
            .field("id", &self.id)
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl DigitalNotary {
    /// Creates a notary for `jurisdiction` from `secret` key material. The public
    /// [`PartyId`] is derived one-way from the secret.
    pub fn new(
        name: impl Into<String>,
        jurisdiction: impl Into<String>,
        secret: impl Into<Vec<u8>>,
    ) -> Self {
        let secret = secret.into();
        let id = PartyId::from_key(&secret);
        Self {
            name: name.into(),
            jurisdiction: jurisdiction.into(),
            id,
            secret,
        }
    }

    /// The notary's display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The jurisdiction this notary represents.
    pub fn jurisdiction(&self) -> &str {
        &self.jurisdiction
    }

    /// The notary's public identifier.
    pub fn id(&self) -> &PartyId {
        &self.id
    }

    /// Produces an attestation over `document_hash`.
    pub fn attest(&self, document_hash: impl Into<String>) -> NotarySignature {
        let document_hash = document_hash.into();
        let seal = compute_seal(&self.secret, &document_hash);
        NotarySignature {
            signer: self.id.as_str().to_string(),
            jurisdiction: self.jurisdiction.clone(),
            document_hash,
            seal,
            timestamp: current_timestamp(),
            scheme: SignatureScheme::KeyedSha256,
        }
    }

    /// Verifies that `signature` is a genuine attestation by this notary over
    /// `document_hash` (authenticity + binding).
    pub fn verify(&self, signature: &NotarySignature, document_hash: &str) -> bool {
        signature.signer == self.id.as_str()
            && signature.covers(document_hash)
            && signature.scheme == SignatureScheme::KeyedSha256
            && signature.seal == compute_seal(&self.secret, document_hash)
    }
}

/// Completeness of a cross-border notarization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotarizationStatus {
    /// No notary has attested yet.
    Unsigned,
    /// At least one — but not both — border jurisdictions have attested.
    Partial,
    /// Both the source and target jurisdictions have attested.
    Complete,
}

/// An aggregation of notary attestations over a single document, tracking
/// cross-border completeness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossBorderNotarization {
    /// The logical document being notarized (e.g. a ported statute id).
    pub document_id: String,
    /// The document hash every signature must cover.
    pub document_hash: String,
    /// Source jurisdiction code.
    pub source_jurisdiction: String,
    /// Target jurisdiction code.
    pub target_jurisdiction: String,
    /// Collected attestations.
    pub signatures: Vec<NotarySignature>,
    /// UNIX timestamp (seconds) the notarization was opened.
    pub created_at: u64,
}

impl CrossBorderNotarization {
    /// Opens an empty notarization for a document on a `source -> target`
    /// corridor.
    pub fn new(
        document_id: impl Into<String>,
        document_hash: impl Into<String>,
        source_jurisdiction: impl Into<String>,
        target_jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            document_id: document_id.into(),
            document_hash: document_hash.into(),
            source_jurisdiction: source_jurisdiction.into(),
            target_jurisdiction: target_jurisdiction.into(),
            signatures: Vec::new(),
            created_at: current_timestamp(),
        }
    }

    /// Opens a notarization keyed to a ledger record's content hash and corridor.
    pub fn for_record(record: &PortingLedgerRecord) -> Self {
        Self::new(
            record.original_id.clone(),
            record.content_hash.clone(),
            record.source_jurisdiction.clone(),
            record.target_jurisdiction.clone(),
        )
    }

    /// Adds an attestation.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the signature does not cover this
    /// notarization's document hash (a binding violation).
    pub fn add_signature(&mut self, signature: NotarySignature) -> NotaryResult<()> {
        if !signature.covers(&self.document_hash) {
            return Err(PortingError::InvalidInput(format!(
                "notary: signature by '{}' covers a different document hash",
                signature.signer
            )));
        }
        self.signatures.push(signature);
        Ok(())
    }

    /// The distinct jurisdictions that have attested.
    pub fn signer_jurisdictions(&self) -> BTreeSet<String> {
        self.signatures
            .iter()
            .map(|s| s.jurisdiction.clone())
            .collect()
    }

    /// The current cross-border completeness status.
    pub fn status(&self) -> NotarizationStatus {
        let jurisdictions = self.signer_jurisdictions();
        let source = jurisdictions.contains(&self.source_jurisdiction);
        let target = jurisdictions.contains(&self.target_jurisdiction);
        match (source, target) {
            (true, true) => NotarizationStatus::Complete,
            (false, false) => {
                if self.signatures.is_empty() {
                    NotarizationStatus::Unsigned
                } else {
                    NotarizationStatus::Partial
                }
            }
            _ => NotarizationStatus::Partial,
        }
    }

    /// Whether both border jurisdictions have attested.
    pub fn is_complete(&self) -> bool {
        self.status() == NotarizationStatus::Complete
    }

    /// Keyless tamper check: every signature must cover `document_hash` and it
    /// must equal this notarization's recorded hash.
    pub fn verify_binding(&self, document_hash: &str) -> bool {
        self.document_hash == document_hash
            && self.signatures.iter().all(|s| s.covers(document_hash))
    }
}

/// A registry that issues and verifies notaries across jurisdictions.
///
/// The registry is the trusted key custodian: it holds notary secrets privately
/// so it can both attest on a notary's behalf and verify attestations. External
/// code only ever sees public [`PartyId`]s and [`NotarySignature`] data.
#[derive(Debug, Default, Clone)]
pub struct NotaryRegistry {
    notaries: HashMap<String, DigitalNotary>,
}

impl NotaryRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a notary for `jurisdiction` from secret key material, returning
    /// its public identifier.
    pub fn register(
        &mut self,
        name: impl Into<String>,
        jurisdiction: impl Into<String>,
        secret: impl Into<Vec<u8>>,
    ) -> PartyId {
        let notary = DigitalNotary::new(name, jurisdiction, secret);
        let id = notary.id().clone();
        self.notaries.insert(id.as_str().to_string(), notary);
        id
    }

    /// Number of registered notaries.
    pub fn len(&self) -> usize {
        self.notaries.len()
    }

    /// Whether the registry has no notaries.
    pub fn is_empty(&self) -> bool {
        self.notaries.is_empty()
    }

    /// Looks up a registered notary by id.
    pub fn notary(&self, id: &PartyId) -> Option<&DigitalNotary> {
        self.notaries.get(id.as_str())
    }

    /// Attests `document_hash` on behalf of the notary identified by `id`.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if no notary is registered for `id`.
    pub fn attest(
        &self,
        id: &PartyId,
        document_hash: impl Into<String>,
    ) -> NotaryResult<NotarySignature> {
        let notary = self.notaries.get(id.as_str()).ok_or_else(|| {
            PortingError::InvalidInput(format!("notary: no notary registered for '{id}'"))
        })?;
        Ok(notary.attest(document_hash))
    }

    /// Verifies a single attestation against the registered signing notary.
    pub fn verify_signature(&self, signature: &NotarySignature, document_hash: &str) -> bool {
        self.notaries
            .get(&signature.signer)
            .map(|notary| notary.verify(signature, document_hash))
            .unwrap_or(false)
    }

    /// Verifies a complete notarization: binding, every signature's authenticity,
    /// and that both border jurisdictions have attested.
    pub fn verify_notarization(&self, notarization: &CrossBorderNotarization) -> bool {
        notarization.verify_binding(&notarization.document_hash)
            && notarization.is_complete()
            && notarization
                .signatures
                .iter()
                .all(|s| self.verify_signature(s, &notarization.document_hash))
    }

    /// High-level convenience: opens a notarization for a ledger record and has
    /// notaries from both the source and target jurisdictions attest it.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if either notary is unregistered, or
    /// does not represent the jurisdiction it is being asked to notarize for.
    pub fn cross_border_notarize(
        &self,
        record: &PortingLedgerRecord,
        source_notary: &PartyId,
        target_notary: &PartyId,
    ) -> NotaryResult<CrossBorderNotarization> {
        let source = self.notaries.get(source_notary.as_str()).ok_or_else(|| {
            PortingError::InvalidInput(format!(
                "notary: no notary registered for '{source_notary}'"
            ))
        })?;
        let target = self.notaries.get(target_notary.as_str()).ok_or_else(|| {
            PortingError::InvalidInput(format!(
                "notary: no notary registered for '{target_notary}'"
            ))
        })?;
        if source.jurisdiction() != record.source_jurisdiction {
            return Err(PortingError::InvalidInput(format!(
                "notary: source notary represents '{}', not the corridor source '{}'",
                source.jurisdiction(),
                record.source_jurisdiction
            )));
        }
        if target.jurisdiction() != record.target_jurisdiction {
            return Err(PortingError::InvalidInput(format!(
                "notary: target notary represents '{}', not the corridor target '{}'",
                target.jurisdiction(),
                record.target_jurisdiction
            )));
        }
        let mut notarization = CrossBorderNotarization::for_record(record);
        notarization.add_signature(source.attest(record.content_hash.clone()))?;
        notarization.add_signature(target.attest(record.content_hash.clone()))?;
        Ok(notarization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::ledger::PortingLedgerRecord;
    use crate::{ChangeType, PortedStatute, PortingChange};
    use legalis_core::{Effect, EffectType, Statute};
    use legalis_i18n::Locale;

    fn record(source: &str, target: &str) -> PortingLedgerRecord {
        let ported = PortedStatute {
            original_id: "doc-1".to_string(),
            statute: Statute::new("t-1", "T", Effect::new(EffectType::Grant, "B")),
            changes: vec![PortingChange {
                change_type: ChangeType::Translation,
                description: "d".to_string(),
                original: None,
                adapted: None,
                reason: "r".to_string(),
            }],
            locale: Locale::new("en").with_country(target),
            compatibility_score: 0.88,
        };
        PortingLedgerRecord::from_ported(&ported, source, target, "reg").expect("record")
    }

    #[test]
    fn test_notary_id_is_one_way_of_secret() {
        let a = DigitalNotary::new("Tokyo", "JP", b"secret-jp".to_vec());
        let b = DigitalNotary::new("Tokyo", "JP", b"secret-jp".to_vec());
        let c = DigitalNotary::new("Berlin", "DE", b"secret-de".to_vec());
        assert_eq!(a.id(), b.id());
        assert_ne!(a.id(), c.id());
        assert!(a.id().as_str().starts_with("0x"));
    }

    #[test]
    fn test_debug_redacts_secret() {
        let notary = DigitalNotary::new("Tokyo", "JP", b"top-secret".to_vec());
        let printed = format!("{notary:?}");
        assert!(printed.contains("<redacted>"));
        assert!(!printed.contains("top-secret"));
    }

    #[test]
    fn test_attest_and_verify() {
        let notary = DigitalNotary::new("Tokyo", "JP", b"k".to_vec());
        let sig = notary.attest("abc123");
        assert!(notary.verify(&sig, "abc123"));
        assert!(!notary.verify(&sig, "different"));
    }

    #[test]
    fn test_verify_rejects_foreign_notary() {
        let signer = DigitalNotary::new("Tokyo", "JP", b"k1".to_vec());
        let other = DigitalNotary::new("Osaka", "JP", b"k2".to_vec());
        let sig = signer.attest("hash");
        assert!(!other.verify(&sig, "hash"));
    }

    #[test]
    fn test_tampered_seal_fails_verification() {
        let notary = DigitalNotary::new("Tokyo", "JP", b"k".to_vec());
        let mut sig = notary.attest("hash");
        sig.seal = "deadbeef".to_string();
        assert!(!notary.verify(&sig, "hash"));
    }

    #[test]
    fn test_notarization_status_progression() {
        let rec = record("JP", "US");
        let mut notz = CrossBorderNotarization::for_record(&rec);
        assert_eq!(notz.status(), NotarizationStatus::Unsigned);

        let jp = DigitalNotary::new("Tokyo", "JP", b"jp".to_vec());
        notz.add_signature(jp.attest(rec.content_hash.clone()))
            .expect("add jp");
        assert_eq!(notz.status(), NotarizationStatus::Partial);
        assert!(!notz.is_complete());

        let us = DigitalNotary::new("DC", "US", b"us".to_vec());
        notz.add_signature(us.attest(rec.content_hash.clone()))
            .expect("add us");
        assert_eq!(notz.status(), NotarizationStatus::Complete);
        assert!(notz.is_complete());
        assert_eq!(notz.signer_jurisdictions().len(), 2);
    }

    #[test]
    fn test_same_border_does_not_complete() {
        let rec = record("JP", "US");
        let mut notz = CrossBorderNotarization::for_record(&rec);
        let jp1 = DigitalNotary::new("Tokyo", "JP", b"jp1".to_vec());
        let jp2 = DigitalNotary::new("Osaka", "JP", b"jp2".to_vec());
        notz.add_signature(jp1.attest(rec.content_hash.clone()))
            .expect("a1");
        notz.add_signature(jp2.attest(rec.content_hash.clone()))
            .expect("a2");
        assert_eq!(notz.status(), NotarizationStatus::Partial);
    }

    #[test]
    fn test_add_signature_rejects_wrong_document() {
        let rec = record("JP", "US");
        let mut notz = CrossBorderNotarization::for_record(&rec);
        let jp = DigitalNotary::new("Tokyo", "JP", b"jp".to_vec());
        let bad = jp.attest("some-other-hash");
        assert!(notz.add_signature(bad).is_err());
    }

    #[test]
    fn test_verify_binding_detects_tamper() {
        let rec = record("JP", "US");
        let mut notz = CrossBorderNotarization::for_record(&rec);
        let jp = DigitalNotary::new("Tokyo", "JP", b"jp".to_vec());
        notz.add_signature(jp.attest(rec.content_hash.clone()))
            .expect("add");
        assert!(notz.verify_binding(&rec.content_hash));
        assert!(!notz.verify_binding("tampered-hash"));
    }

    #[test]
    fn test_registry_register_attest_verify() {
        let mut registry = NotaryRegistry::new();
        assert!(registry.is_empty());
        let jp = registry.register("Tokyo", "JP", b"jp-secret".to_vec());
        assert_eq!(registry.len(), 1);
        assert!(registry.notary(&jp).is_some());
        let sig = registry.attest(&jp, "hash-1").expect("attest");
        assert!(registry.verify_signature(&sig, "hash-1"));
        assert!(!registry.verify_signature(&sig, "hash-2"));
    }

    #[test]
    fn test_registry_attest_unknown_notary() {
        let registry = NotaryRegistry::new();
        let phantom = PartyId::from_label("0xunknown");
        assert!(registry.attest(&phantom, "h").is_err());
    }

    #[test]
    fn test_registry_verify_unknown_signer_is_false() {
        let registry = NotaryRegistry::new();
        let outsider = DigitalNotary::new("X", "JP", b"x".to_vec());
        let sig = outsider.attest("h");
        assert!(!registry.verify_signature(&sig, "h"));
    }

    #[test]
    fn test_cross_border_notarize_happy_path() {
        let rec = record("JP", "US");
        let mut registry = NotaryRegistry::new();
        let jp = registry.register("Tokyo", "JP", b"jp".to_vec());
        let us = registry.register("DC", "US", b"us".to_vec());
        let notz = registry
            .cross_border_notarize(&rec, &jp, &us)
            .expect("notarize");
        assert!(notz.is_complete());
        assert!(registry.verify_notarization(&notz));
    }

    #[test]
    fn test_cross_border_notarize_wrong_jurisdiction() {
        let rec = record("JP", "US");
        let mut registry = NotaryRegistry::new();
        let jp = registry.register("Tokyo", "JP", b"jp".to_vec());
        let de = registry.register("Berlin", "DE", b"de".to_vec());
        // DE notary cannot stand in for the US side.
        assert!(registry.cross_border_notarize(&rec, &jp, &de).is_err());
    }

    #[test]
    fn test_verify_notarization_requires_completeness() {
        let rec = record("JP", "US");
        let mut registry = NotaryRegistry::new();
        let jp = registry.register("Tokyo", "JP", b"jp".to_vec());
        let mut notz = CrossBorderNotarization::for_record(&rec);
        notz.add_signature(registry.attest(&jp, rec.content_hash.clone()).expect("a"))
            .expect("add");
        // Only one border has signed -> not verifiable as complete.
        assert!(!registry.verify_notarization(&notz));
    }

    #[test]
    fn test_notarization_serde_roundtrip() {
        let rec = record("JP", "US");
        let mut registry = NotaryRegistry::new();
        let jp = registry.register("Tokyo", "JP", b"jp".to_vec());
        let us = registry.register("DC", "US", b"us".to_vec());
        let notz = registry
            .cross_border_notarize(&rec, &jp, &us)
            .expect("notarize");
        let json = serde_json::to_string(&notz).expect("ser");
        let back: CrossBorderNotarization = serde_json::from_str(&json).expect("de");
        assert_eq!(notz, back);
        assert!(registry.verify_notarization(&back));
    }
}
