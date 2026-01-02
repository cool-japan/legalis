//! Decentralized Identifier (DID) support for legal resources.
//!
//! This module implements DID methods for legal knowledge graphs,
//! enabling decentralized identity and verifiable credentials.

use crate::{LodError, LodResult, RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DID (Decentralized Identifier) representation.
///
/// Format: did:method:identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Did {
    /// DID method (e.g., "key", "web", "ipld")
    pub method: String,
    /// Method-specific identifier
    pub identifier: String,
}

impl Did {
    /// Creates a new DID.
    pub fn new(method: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            identifier: identifier.into(),
        }
    }

    /// Parses a DID string.
    pub fn parse(did: &str) -> LodResult<Self> {
        let parts: Vec<&str> = did.split(':').collect();
        if parts.len() < 3 || parts[0] != "did" {
            return Err(LodError::InvalidUri(format!("Invalid DID format: {}", did)));
        }

        Ok(Self {
            method: parts[1].to_string(),
            identifier: parts[2..].join(":"),
        })
    }

    /// Returns the string representation of the DID.
    pub fn to_string(&self) -> String {
        format!("did:{}:{}", self.method, self.identifier)
    }

    /// Returns the URI form.
    pub fn to_uri(&self) -> String {
        self.to_string()
    }
}

/// DID document containing public keys and service endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    /// The DID this document describes
    pub id: Did,
    /// Verification methods (public keys)
    pub verification_method: Vec<VerificationMethod>,
    /// Authentication methods
    pub authentication: Vec<String>,
    /// Service endpoints
    pub service: Vec<ServiceEndpoint>,
    /// Creation timestamp
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Last update timestamp
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

impl DidDocument {
    /// Creates a new DID document.
    pub fn new(did: Did) -> Self {
        Self {
            id: did,
            verification_method: Vec::new(),
            authentication: Vec::new(),
            service: Vec::new(),
            created: Some(chrono::Utc::now()),
            updated: None,
        }
    }

    /// Adds a verification method.
    pub fn add_verification_method(&mut self, method: VerificationMethod) {
        self.verification_method.push(method);
    }

    /// Adds an authentication method.
    pub fn add_authentication(&mut self, method_id: impl Into<String>) {
        self.authentication.push(method_id.into());
    }

    /// Adds a service endpoint.
    pub fn add_service(&mut self, service: ServiceEndpoint) {
        self.service.push(service);
    }

    /// Updates the timestamp.
    pub fn update(&mut self) {
        self.updated = Some(chrono::Utc::now());
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
    pub fn to_triples(&self, base_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let subject = self.id.to_uri();

        // Type
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("https://w3id.org/did#DIDDocument".to_string()),
        });

        // Verification methods
        for (i, vm) in self.verification_method.iter().enumerate() {
            let vm_uri = format!("{}#key-{}", base_uri, i);
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "https://w3id.org/security#verificationMethod".to_string(),
                object: RdfValue::Uri(vm_uri.clone()),
            });

            triples.push(Triple {
                subject: vm_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("https://w3id.org/security#VerificationMethod".to_string()),
            });

            triples.push(Triple {
                subject: vm_uri.clone(),
                predicate: "https://w3id.org/security#publicKeyJwk".to_string(),
                object: RdfValue::string(&vm.public_key_jwk),
            });
        }

        // Services
        for service in &self.service {
            let service_uri = format!("{}#{}", base_uri, service.id);
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "https://w3id.org/did#service".to_string(),
                object: RdfValue::Uri(service_uri.clone()),
            });

            triples.push(Triple {
                subject: service_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(service.service_type.clone()),
            });

            triples.push(Triple {
                subject: service_uri,
                predicate: "https://w3id.org/did#serviceEndpoint".to_string(),
                object: RdfValue::Uri(service.service_endpoint.clone()),
            });
        }

        triples
    }
}

/// Verification method (public key).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    /// Method ID
    pub id: String,
    /// Key type (e.g., "Ed25519VerificationKey2020")
    pub key_type: String,
    /// Controller DID
    pub controller: String,
    /// Public key in JWK format
    pub public_key_jwk: String,
}

impl VerificationMethod {
    /// Creates a new verification method.
    pub fn new(
        id: impl Into<String>,
        key_type: impl Into<String>,
        controller: impl Into<String>,
        public_key_jwk: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            key_type: key_type.into(),
            controller: controller.into(),
            public_key_jwk: public_key_jwk.into(),
        }
    }
}

/// Service endpoint for DID communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Service ID
    pub id: String,
    /// Service type
    pub service_type: String,
    /// Endpoint URL
    pub service_endpoint: String,
}

impl ServiceEndpoint {
    /// Creates a new service endpoint.
    pub fn new(
        id: impl Into<String>,
        service_type: impl Into<String>,
        service_endpoint: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            service_type: service_type.into(),
            service_endpoint: service_endpoint.into(),
        }
    }
}

/// DID resolver for retrieving DID documents.
#[derive(Debug, Default)]
pub struct DidResolver {
    /// Cache of resolved DID documents
    cache: HashMap<String, DidDocument>,
}

impl DidResolver {
    /// Creates a new DID resolver.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Resolves a DID to its document.
    pub fn resolve(&self, did: &Did) -> LodResult<Option<DidDocument>> {
        Ok(self.cache.get(&did.to_string()).cloned())
    }

    /// Registers a DID document.
    pub fn register(&mut self, doc: DidDocument) {
        self.cache.insert(doc.id.to_string(), doc);
    }

    /// Lists all registered DIDs.
    pub fn list_dids(&self) -> Vec<Did> {
        self.cache.values().map(|doc| doc.id.clone()).collect()
    }

    /// Returns the number of registered DIDs.
    pub fn count(&self) -> usize {
        self.cache.len()
    }
}

/// DID method for IPLD-based identifiers.
pub struct DidIpld;

impl DidIpld {
    /// Creates a DID from an IPLD CID.
    pub fn from_cid(cid: &crate::ipld::Cid) -> Did {
        Did::new("ipld", cid.hash.clone())
    }

    /// Extracts the CID from a DID.
    pub fn to_cid(did: &Did) -> LodResult<crate::ipld::Cid> {
        if did.method != "ipld" {
            return Err(LodError::InvalidUri(format!(
                "Not an IPLD DID: {}",
                did.to_string()
            )));
        }

        Ok(crate::ipld::Cid::new(&did.identifier, "dag-json"))
    }
}

/// DID method for web-based identifiers.
pub struct DidWeb;

impl DidWeb {
    /// Creates a DID from a web domain.
    pub fn from_domain(domain: &str) -> Did {
        Did::new("web", domain)
    }

    /// Extracts the domain from a DID.
    pub fn to_domain(did: &Did) -> LodResult<String> {
        if did.method != "web" {
            return Err(LodError::InvalidUri(format!(
                "Not a web DID: {}",
                did.to_string()
            )));
        }

        Ok(did.identifier.clone())
    }

    /// Constructs the well-known DID document URL.
    pub fn well_known_url(did: &Did) -> LodResult<String> {
        let domain = Self::to_domain(did)?;
        Ok(format!("https://{}/.well-known/did.json", domain))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_creation() {
        let did = Did::new("key", "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK");
        assert_eq!(did.method, "key");
        assert_eq!(
            did.to_string(),
            "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        );
    }

    #[test]
    fn test_did_parse() {
        let did_str = "did:web:example.org";
        let did = Did::parse(did_str).unwrap();
        assert_eq!(did.method, "web");
        assert_eq!(did.identifier, "example.org");
    }

    #[test]
    fn test_did_parse_invalid() {
        let result = Did::parse("invalid:did:format");
        assert!(result.is_err());
    }

    #[test]
    fn test_did_document() {
        let did = Did::new("web", "example.org");
        let mut doc = DidDocument::new(did.clone());

        let vm = VerificationMethod::new(
            "#key-1",
            "Ed25519VerificationKey2020",
            did.to_string(),
            "{}",
        );
        doc.add_verification_method(vm);

        assert_eq!(doc.verification_method.len(), 1);
    }

    #[test]
    fn test_did_document_json() {
        let did = Did::new("web", "example.org");
        let doc = DidDocument::new(did);

        let json = doc.to_json().unwrap();
        let parsed = DidDocument::from_json(&json).unwrap();

        assert_eq!(doc.id, parsed.id);
    }

    #[test]
    fn test_did_resolver() {
        let mut resolver = DidResolver::new();

        let did = Did::new("web", "example.org");
        let doc = DidDocument::new(did.clone());

        resolver.register(doc);

        let resolved = resolver.resolve(&did).unwrap();
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().id, did);
    }

    #[test]
    fn test_did_ipld() {
        let cid = crate::ipld::Cid::new("QmTest123", "dag-json");
        let did = DidIpld::from_cid(&cid);

        assert_eq!(did.method, "ipld");
        assert_eq!(did.identifier, "QmTest123");

        let extracted_cid = DidIpld::to_cid(&did).unwrap();
        assert_eq!(extracted_cid.hash, cid.hash);
    }

    #[test]
    fn test_did_web() {
        let did = DidWeb::from_domain("example.org");

        assert_eq!(did.method, "web");
        assert_eq!(did.identifier, "example.org");

        let url = DidWeb::well_known_url(&did).unwrap();
        assert_eq!(url, "https://example.org/.well-known/did.json");
    }

    #[test]
    fn test_service_endpoint() {
        let service =
            ServiceEndpoint::new("legal-api", "LegalDataAPI", "https://example.org/api/legal");

        assert_eq!(service.id, "legal-api");
        assert_eq!(service.service_type, "LegalDataAPI");
    }

    #[test]
    fn test_did_document_update() {
        let did = Did::new("web", "example.org");
        let mut doc = DidDocument::new(did);

        assert!(doc.updated.is_none());
        doc.update();
        assert!(doc.updated.is_some());
    }

    #[test]
    fn test_did_to_triples() {
        let did = Did::new("web", "example.org");
        let mut doc = DidDocument::new(did.clone());

        let vm = VerificationMethod::new(
            "#key-1",
            "Ed25519VerificationKey2020",
            did.to_string(),
            "{}",
        );
        doc.add_verification_method(vm);

        let triples = doc.to_triples("https://example.org");
        assert!(!triples.is_empty());

        // Check for DID document type
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
    }

    #[test]
    fn test_resolver_list_dids() {
        let mut resolver = DidResolver::new();

        let did1 = Did::new("web", "example1.org");
        let did2 = Did::new("web", "example2.org");

        resolver.register(DidDocument::new(did1));
        resolver.register(DidDocument::new(did2));

        let dids = resolver.list_dids();
        assert_eq!(dids.len(), 2);
    }

    #[test]
    fn test_resolver_count() {
        let mut resolver = DidResolver::new();
        assert_eq!(resolver.count(), 0);

        let did = Did::new("web", "example.org");
        resolver.register(DidDocument::new(did));
        assert_eq!(resolver.count(), 1);
    }
}
