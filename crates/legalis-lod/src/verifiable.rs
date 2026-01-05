//! Verifiable Credentials for legal data.
//!
//! This module implements W3C Verifiable Credentials standard for legal knowledge,
//! enabling trustable and cryptographically verifiable legal information.

use crate::did::Did;
use crate::{LodError, LodResult, RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Verifiable Credential for legal data.
///
/// Following W3C Verifiable Credentials Data Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    /// JSON-LD context
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// Credential ID
    pub id: Option<String>,
    /// Credential type
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    /// Issuer DID
    pub issuer: String,
    /// Issuance date
    #[serde(rename = "issuanceDate")]
    pub issuance_date: chrono::DateTime<chrono::Utc>,
    /// Expiration date
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Credential subject
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,
    /// Proof (digital signature)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Proof>,
}

impl VerifiableCredential {
    /// Creates a new verifiable credential.
    pub fn new(issuer: impl Into<String>, subject: CredentialSubject) -> Self {
        Self {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_string(),
                "https://legalis.dev/credentials/v1".to_string(),
            ],
            id: None,
            credential_type: vec!["VerifiableCredential".to_string()],
            issuer: issuer.into(),
            issuance_date: chrono::Utc::now(),
            expiration_date: None,
            credential_subject: subject,
            proof: None,
        }
    }

    /// Sets the credential ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a credential type.
    pub fn with_type(mut self, credential_type: impl Into<String>) -> Self {
        self.credential_type.push(credential_type.into());
        self
    }

    /// Sets the expiration date.
    pub fn with_expiration(mut self, expiration: chrono::DateTime<chrono::Utc>) -> Self {
        self.expiration_date = Some(expiration);
        self
    }

    /// Adds a proof.
    pub fn with_proof(mut self, proof: Proof) -> Self {
        self.proof = Some(proof);
        self
    }

    /// Checks if the credential is expired.
    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expiration_date {
            chrono::Utc::now() > exp
        } else {
            false
        }
    }

    /// Verifies the credential.
    pub fn verify(&self) -> bool {
        // In a real implementation, this would verify the cryptographic proof
        // For now, we just check basic validity
        !self.is_expired() && self.proof.is_some()
    }

    /// Exports to JSON.
    pub fn to_json(&self) -> LodResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| LodError::SerializationError(e.to_string()))
    }

    /// Imports from JSON.
    pub fn from_json(json: &str) -> LodResult<Self> {
        serde_json::from_str(json).map_err(|e| LodError::SerializationError(e.to_string()))
    }

    /// Converts to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let subject = self
            .id
            .clone()
            .unwrap_or_else(|| "_:credential".to_string());

        // Type
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(
                "https://www.w3.org/2018/credentials#VerifiableCredential".to_string(),
            ),
        });

        // Issuer
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "https://www.w3.org/2018/credentials#issuer".to_string(),
            object: RdfValue::Uri(self.issuer.clone()),
        });

        // Issuance date
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "https://www.w3.org/2018/credentials#issuanceDate".to_string(),
            object: RdfValue::datetime(self.issuance_date),
        });

        // Expiration date
        if let Some(exp) = self.expiration_date {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "https://www.w3.org/2018/credentials#expirationDate".to_string(),
                object: RdfValue::datetime(exp),
            });
        }

        // Credential subject
        let subject_id = self.credential_subject.id.clone();
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "https://www.w3.org/2018/credentials#credentialSubject".to_string(),
            object: RdfValue::Uri(subject_id.clone()),
        });

        // Subject claims
        for (key, value) in &self.credential_subject.claims {
            triples.push(Triple {
                subject: subject_id.clone(),
                predicate: key.clone(),
                object: RdfValue::string(value),
            });
        }

        triples
    }
}

/// Credential subject (the entity the credential is about).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    /// Subject identifier (DID or URI)
    pub id: String,
    /// Claims about the subject
    #[serde(flatten)]
    pub claims: HashMap<String, String>,
}

impl CredentialSubject {
    /// Creates a new credential subject.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            claims: HashMap::new(),
        }
    }

    /// Adds a claim.
    pub fn add_claim(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.claims.insert(key.into(), value.into());
    }

    /// Gets a claim value.
    pub fn get_claim(&self, key: &str) -> Option<&String> {
        self.claims.get(key)
    }
}

/// Cryptographic proof for a verifiable credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    /// Proof type (e.g., "Ed25519Signature2020")
    #[serde(rename = "type")]
    pub proof_type: String,
    /// Creation timestamp
    pub created: chrono::DateTime<chrono::Utc>,
    /// Verification method
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,
    /// Proof purpose
    #[serde(rename = "proofPurpose")]
    pub proof_purpose: String,
    /// Signature value
    #[serde(rename = "proofValue")]
    pub proof_value: String,
}

impl Proof {
    /// Creates a new proof.
    pub fn new(
        proof_type: impl Into<String>,
        verification_method: impl Into<String>,
        proof_value: impl Into<String>,
    ) -> Self {
        Self {
            proof_type: proof_type.into(),
            created: chrono::Utc::now(),
            verification_method: verification_method.into(),
            proof_purpose: "assertionMethod".to_string(),
            proof_value: proof_value.into(),
        }
    }
}

/// Verifiable Presentation - a collection of credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiablePresentation {
    /// JSON-LD context
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// Presentation type
    #[serde(rename = "type")]
    pub presentation_type: Vec<String>,
    /// Holder DID
    pub holder: String,
    /// Verifiable credentials
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<VerifiableCredential>,
    /// Proof
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<Proof>,
}

impl VerifiablePresentation {
    /// Creates a new verifiable presentation.
    pub fn new(holder: impl Into<String>) -> Self {
        Self {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            presentation_type: vec!["VerifiablePresentation".to_string()],
            holder: holder.into(),
            verifiable_credential: Vec::new(),
            proof: None,
        }
    }

    /// Adds a credential.
    pub fn add_credential(&mut self, credential: VerifiableCredential) {
        self.verifiable_credential.push(credential);
    }

    /// Adds a proof.
    pub fn with_proof(mut self, proof: Proof) -> Self {
        self.proof = Some(proof);
        self
    }

    /// Verifies all credentials in the presentation.
    pub fn verify(&self) -> bool {
        self.verifiable_credential.iter().all(|vc| vc.verify())
    }

    /// Exports to JSON.
    pub fn to_json(&self) -> LodResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| LodError::SerializationError(e.to_string()))
    }
}

/// Legal statute credential builder.
pub struct LegalStatuteCredential;

impl LegalStatuteCredential {
    /// Creates a credential for a legal statute.
    pub fn create(
        issuer_did: &Did,
        statute_uri: &str,
        title: &str,
        jurisdiction: Option<&str>,
    ) -> VerifiableCredential {
        let mut subject = CredentialSubject::new(statute_uri);
        subject.add_claim("title", title);
        subject.add_claim("type", "LegalStatute");

        if let Some(j) = jurisdiction {
            subject.add_claim("jurisdiction", j);
        }

        VerifiableCredential::new(issuer_did.to_string(), subject)
            .with_type("LegalStatuteCredential")
    }
}

/// Credential registry for storing and retrieving credentials.
#[derive(Debug, Default)]
pub struct CredentialRegistry {
    credentials: HashMap<String, VerifiableCredential>,
}

impl CredentialRegistry {
    /// Creates a new credential registry.
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
        }
    }

    /// Registers a credential.
    pub fn register(&mut self, credential: VerifiableCredential) -> LodResult<String> {
        let id = credential
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        self.credentials.insert(id.clone(), credential);
        Ok(id)
    }

    /// Retrieves a credential by ID.
    pub fn get(&self, id: &str) -> Option<&VerifiableCredential> {
        self.credentials.get(id)
    }

    /// Lists all credential IDs.
    pub fn list_ids(&self) -> Vec<String> {
        self.credentials.keys().cloned().collect()
    }

    /// Filters credentials by issuer.
    pub fn filter_by_issuer(&self, issuer: &str) -> Vec<&VerifiableCredential> {
        self.credentials
            .values()
            .filter(|vc| vc.issuer == issuer)
            .collect()
    }

    /// Filters valid (non-expired) credentials.
    pub fn filter_valid(&self) -> Vec<&VerifiableCredential> {
        self.credentials
            .values()
            .filter(|vc| !vc.is_expired())
            .collect()
    }

    /// Returns the number of credentials.
    pub fn count(&self) -> usize {
        self.credentials.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_subject() {
        let mut subject = CredentialSubject::new("did:example:123");
        subject.add_claim("name", "Test Statute");
        subject.add_claim("jurisdiction", "US");

        assert_eq!(subject.get_claim("name"), Some(&"Test Statute".to_string()));
        assert_eq!(subject.claims.len(), 2);
    }

    #[test]
    fn test_verifiable_credential() {
        let subject = CredentialSubject::new("did:example:statute:123");
        let vc = VerifiableCredential::new("did:example:issuer", subject)
            .with_id("https://example.org/credentials/1")
            .with_type("LegalStatuteCredential");

        assert_eq!(vc.id, Some("https://example.org/credentials/1".to_string()));
        assert!(
            vc.credential_type
                .contains(&"LegalStatuteCredential".to_string())
        );
    }

    #[test]
    fn test_credential_expiration() {
        let subject = CredentialSubject::new("did:example:123");
        let past = chrono::Utc::now() - chrono::Duration::days(1);

        let vc = VerifiableCredential::new("did:example:issuer", subject).with_expiration(past);

        assert!(vc.is_expired());
    }

    #[test]
    fn test_credential_with_proof() {
        let subject = CredentialSubject::new("did:example:123");
        let proof = Proof::new(
            "Ed25519Signature2020",
            "did:example:issuer#key-1",
            "base64signature",
        );

        let vc = VerifiableCredential::new("did:example:issuer", subject).with_proof(proof);

        assert!(vc.proof.is_some());
    }

    #[test]
    fn test_credential_json() {
        let subject = CredentialSubject::new("did:example:123");
        let vc = VerifiableCredential::new("did:example:issuer", subject);

        let json = vc.to_json().unwrap();
        let parsed = VerifiableCredential::from_json(&json).unwrap();

        assert_eq!(vc.issuer, parsed.issuer);
    }

    #[test]
    fn test_verifiable_presentation() {
        let mut vp = VerifiablePresentation::new("did:example:holder");

        let subject = CredentialSubject::new("did:example:123");
        let vc = VerifiableCredential::new("did:example:issuer", subject);

        vp.add_credential(vc);

        assert_eq!(vp.verifiable_credential.len(), 1);
    }

    #[test]
    fn test_legal_statute_credential() {
        let issuer = Did::new("web", "legalis.dev");
        let vc = LegalStatuteCredential::create(
            &issuer,
            "https://legalis.dev/statute/123",
            "Test Statute",
            Some("US"),
        );

        assert!(
            vc.credential_type
                .contains(&"LegalStatuteCredential".to_string())
        );
        assert_eq!(
            vc.credential_subject.get_claim("title"),
            Some(&"Test Statute".to_string())
        );
    }

    #[test]
    fn test_credential_registry() {
        let mut registry = CredentialRegistry::new();

        let subject = CredentialSubject::new("did:example:123");
        let vc = VerifiableCredential::new("did:example:issuer", subject).with_id("cred-1");

        let id = registry.register(vc).unwrap();
        assert_eq!(id, "cred-1");
        assert_eq!(registry.count(), 1);

        let retrieved = registry.get(&id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_registry_filter_by_issuer() {
        let mut registry = CredentialRegistry::new();

        let subject1 = CredentialSubject::new("did:example:1");
        let vc1 = VerifiableCredential::new("did:example:issuer1", subject1).with_id("cred-1");

        let subject2 = CredentialSubject::new("did:example:2");
        let vc2 = VerifiableCredential::new("did:example:issuer2", subject2).with_id("cred-2");

        registry.register(vc1).unwrap();
        registry.register(vc2).unwrap();

        let filtered = registry.filter_by_issuer("did:example:issuer1");
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_registry_filter_valid() {
        let mut registry = CredentialRegistry::new();

        let subject1 = CredentialSubject::new("did:example:1");
        let future = chrono::Utc::now() + chrono::Duration::days(30);
        let vc1 = VerifiableCredential::new("did:example:issuer", subject1)
            .with_id("cred-1")
            .with_expiration(future);

        let subject2 = CredentialSubject::new("did:example:2");
        let past = chrono::Utc::now() - chrono::Duration::days(1);
        let vc2 = VerifiableCredential::new("did:example:issuer", subject2)
            .with_id("cred-2")
            .with_expiration(past);

        registry.register(vc1).unwrap();
        registry.register(vc2).unwrap();

        let valid = registry.filter_valid();
        assert_eq!(valid.len(), 1);
    }

    #[test]
    fn test_credential_to_triples() {
        let subject = CredentialSubject::new("did:example:statute:123");
        let vc = VerifiableCredential::new("did:example:issuer", subject)
            .with_id("https://example.org/credentials/1");

        let triples = vc.to_triples();
        assert!(!triples.is_empty());

        // Check for credential type
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));

        // Check for issuer
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "https://www.w3.org/2018/credentials#issuer")
        );
    }

    #[test]
    fn test_presentation_verify() {
        let mut vp = VerifiablePresentation::new("did:example:holder");

        let subject = CredentialSubject::new("did:example:123");
        let proof = Proof::new(
            "Ed25519Signature2020",
            "did:example:issuer#key-1",
            "signature",
        );

        let vc = VerifiableCredential::new("did:example:issuer", subject).with_proof(proof);

        vp.add_credential(vc);

        assert!(vp.verify());
    }

    #[test]
    fn test_proof_creation() {
        let proof = Proof::new(
            "Ed25519Signature2020",
            "did:example:issuer#key-1",
            "base64signature",
        );

        assert_eq!(proof.proof_type, "Ed25519Signature2020");
        assert_eq!(proof.proof_purpose, "assertionMethod");
    }
}
