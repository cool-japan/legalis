//! Linked Data features for Legalis-LOD.
//!
//! This module provides:
//! - URI dereferencing support
//! - Cool URIs for legal resources
//! - owl:sameAs linking for entity resolution
//! - rdfs:seeAlso for related resources

use crate::{LodError, LodResult, RdfFormat, RdfValue, Triple};
use chrono::Datelike;
use legalis_core::Statute;
use std::collections::HashMap;

/// Cool URI patterns for legal resources.
///
/// Implements best practices for legal resource URIs as recommended by
/// European Legislation Identifier (ELI) and similar initiatives.
#[derive(Debug, Clone)]
pub struct CoolUriScheme {
    /// Base domain (e.g., "legislation.example.org")
    pub domain: String,
    /// Whether to use HTTPS
    pub use_https: bool,
    /// URI path pattern
    pub pattern: UriPattern,
}

/// URI pattern types for legal resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UriPattern {
    /// /jurisdiction/year/type/number
    /// Example: /uk/2023/act/42
    Hierarchical,
    /// /id/jurisdiction/type/year/number
    /// Example: /id/uk/act/2023/42
    EliStyle,
    /// /doc/jurisdiction-type-year-number
    /// Example: /doc/uk-act-2023-42
    Flat,
}

impl CoolUriScheme {
    /// Creates a new Cool URI scheme.
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            use_https: true,
            pattern: UriPattern::EliStyle,
        }
    }

    /// Sets the URI pattern.
    pub fn with_pattern(mut self, pattern: UriPattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Sets whether to use HTTPS.
    pub fn with_https(mut self, use_https: bool) -> Self {
        self.use_https = use_https;
        self
    }

    /// Generates a Cool URI for a statute.
    pub fn generate_uri(&self, statute: &Statute) -> String {
        let protocol = if self.use_https { "https" } else { "http" };
        let base = format!("{}://{}", protocol, self.domain);

        // Extract year from effective date if available
        let year = statute
            .temporal_validity
            .effective_date
            .map(|d| d.year().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Extract jurisdiction
        let jurisdiction = statute
            .jurisdiction
            .as_ref()
            .map(|j| j.to_lowercase().replace(' ', "-"))
            .unwrap_or_else(|| "default".to_string());

        // Generate path based on pattern
        let path = match self.pattern {
            UriPattern::Hierarchical => {
                format!("{}/{}/statute/{}", jurisdiction, year, statute.id)
            }
            UriPattern::EliStyle => {
                format!("id/{}/statute/{}/{}", jurisdiction, year, statute.id)
            }
            UriPattern::Flat => {
                format!("doc/{}-statute-{}-{}", jurisdiction, year, statute.id)
            }
        };

        format!("{}/{}", base, path)
    }

    /// Generates version-specific URI.
    pub fn generate_version_uri(&self, statute: &Statute) -> String {
        let base_uri = self.generate_uri(statute);
        if statute.version > 0 {
            format!("{}/version/{}", base_uri, statute.version)
        } else {
            base_uri
        }
    }

    /// Generates format-specific URI for content negotiation.
    pub fn generate_format_uri(&self, statute: &Statute, format: RdfFormat) -> String {
        format!("{}.{}", self.generate_uri(statute), format.extension())
    }
}

/// URI dereferencer for legal resources.
///
/// Provides HTTP-style content negotiation and URI resolution.
#[derive(Debug)]
pub struct UriDereferencer {
    /// URI scheme for generating URIs
    uri_scheme: CoolUriScheme,
    /// Statute registry (URI -> Statute)
    registry: HashMap<String, Statute>,
}

impl UriDereferencer {
    /// Creates a new URI dereferencer.
    pub fn new(uri_scheme: CoolUriScheme) -> Self {
        Self {
            uri_scheme,
            registry: HashMap::new(),
        }
    }

    /// Registers a statute for dereferencing.
    pub fn register(&mut self, statute: Statute) {
        let uri = self.uri_scheme.generate_uri(&statute);
        self.registry.insert(uri, statute);
    }

    /// Registers multiple statutes.
    pub fn register_batch(&mut self, statutes: Vec<Statute>) {
        for statute in statutes {
            self.register(statute);
        }
    }

    /// Dereferences a URI to get the statute.
    pub fn dereference(&self, uri: &str) -> LodResult<&Statute> {
        self.registry
            .get(uri)
            .ok_or_else(|| LodError::InvalidUri(format!("URI not found: {}", uri)))
    }

    /// Resolves a URI with content negotiation.
    pub fn resolve(&self, uri: &str, accept_header: &str) -> LodResult<(RdfFormat, &Statute)> {
        let statute = self.dereference(uri)?;
        let format = RdfFormat::from_accept_header(accept_header);
        Ok((format, statute))
    }

    /// Lists all registered URIs.
    pub fn list_uris(&self) -> Vec<&str> {
        self.registry.keys().map(|s| s.as_str()).collect()
    }
}

/// Entity resolver for linking legal resources.
///
/// Provides owl:sameAs and rdfs:seeAlso linking capabilities.
#[derive(Debug, Clone)]
pub struct EntityResolver {
    /// Base URI for this resolver
    base_uri: String,
    /// owl:sameAs mappings (local URI -> external URI)
    same_as: HashMap<String, Vec<String>>,
    /// rdfs:seeAlso mappings (local URI -> related URI)
    see_also: HashMap<String, Vec<String>>,
}

impl EntityResolver {
    /// Creates a new entity resolver.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            same_as: HashMap::new(),
            see_also: HashMap::new(),
        }
    }

    /// Adds an owl:sameAs link.
    pub fn add_same_as(&mut self, local_id: impl Into<String>, external_uri: impl Into<String>) {
        self.same_as
            .entry(local_id.into())
            .or_default()
            .push(external_uri.into());
    }

    /// Adds an rdfs:seeAlso link.
    pub fn add_see_also(&mut self, local_id: impl Into<String>, related_uri: impl Into<String>) {
        self.see_also
            .entry(local_id.into())
            .or_default()
            .push(related_uri.into());
    }

    /// Adds a link to EUR-Lex.
    pub fn link_to_eurlex(&mut self, statute_id: &str, celex_number: &str) {
        let eurlex_uri = format!("http://data.europa.eu/eli/{}", celex_number);
        self.add_same_as(statute_id, eurlex_uri);
    }

    /// Adds a link to legislation.gov.uk.
    pub fn link_to_uk_legislation(&mut self, statute_id: &str, year: u32, number: u32) {
        let uk_uri = format!("http://www.legislation.gov.uk/ukpga/{}/{}", year, number);
        self.add_same_as(statute_id, uk_uri);
    }

    /// Adds a link to Wikidata.
    pub fn link_to_wikidata(&mut self, statute_id: &str, wikidata_id: &str) {
        let wikidata_uri = format!("http://www.wikidata.org/entity/{}", wikidata_id);
        self.add_same_as(statute_id, wikidata_uri);
    }

    /// Adds a link to DBpedia.
    pub fn link_to_dbpedia(&mut self, statute_id: &str, dbpedia_resource: &str) {
        let dbpedia_uri = format!("http://dbpedia.org/resource/{}", dbpedia_resource);
        self.add_see_also(statute_id, dbpedia_uri);
    }

    /// Generates owl:sameAs triples for a statute.
    pub fn generate_same_as_triples(&self, statute_id: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let subject_uri = format!("{}statute/{}", self.base_uri, statute_id);

        if let Some(external_uris) = self.same_as.get(statute_id) {
            for external_uri in external_uris {
                triples.push(Triple {
                    subject: subject_uri.clone(),
                    predicate: "owl:sameAs".to_string(),
                    object: RdfValue::Uri(external_uri.clone()),
                });
            }
        }

        triples
    }

    /// Generates rdfs:seeAlso triples for a statute.
    pub fn generate_see_also_triples(&self, statute_id: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let subject_uri = format!("{}statute/{}", self.base_uri, statute_id);

        if let Some(related_uris) = self.see_also.get(statute_id) {
            for related_uri in related_uris {
                triples.push(Triple {
                    subject: subject_uri.clone(),
                    predicate: "rdfs:seeAlso".to_string(),
                    object: RdfValue::Uri(related_uri.clone()),
                });
            }
        }

        triples
    }

    /// Generates all linking triples for a statute.
    pub fn generate_linking_triples(&self, statute_id: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        triples.extend(self.generate_same_as_triples(statute_id));
        triples.extend(self.generate_see_also_triples(statute_id));
        triples
    }
}

/// Link validator for checking dead references.
#[derive(Debug)]
pub struct LinkValidator {
    /// URIs to validate
    uris: Vec<String>,
    /// Validation results
    results: HashMap<String, ValidationResult>,
}

/// Result of link validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationResult {
    /// Link is valid
    Valid,
    /// Link is broken (404, etc.)
    Broken,
    /// Link validation timed out
    Timeout,
    /// Link validation skipped
    Skipped,
}

impl LinkValidator {
    /// Creates a new link validator.
    pub fn new() -> Self {
        Self {
            uris: Vec::new(),
            results: HashMap::new(),
        }
    }

    /// Adds a URI to validate.
    pub fn add_uri(&mut self, uri: impl Into<String>) {
        self.uris.push(uri.into());
    }

    /// Extracts URIs from triples for validation.
    pub fn extract_uris_from_triples(&mut self, triples: &[Triple]) {
        for triple in triples {
            if let RdfValue::Uri(uri) = &triple.object {
                if uri.starts_with("http://") || uri.starts_with("https://") {
                    self.uris.push(uri.clone());
                }
            }
        }
    }

    /// Validates all URIs (stub implementation - would need HTTP client in production).
    pub fn validate_all(&mut self) {
        for uri in &self.uris {
            // In a real implementation, this would make HTTP HEAD requests
            // For now, we mark all as skipped
            self.results.insert(uri.clone(), ValidationResult::Skipped);
        }
    }

    /// Gets validation results.
    pub fn get_results(&self) -> &HashMap<String, ValidationResult> {
        &self.results
    }

    /// Gets broken links.
    pub fn get_broken_links(&self) -> Vec<&str> {
        self.results
            .iter()
            .filter(|(_, result)| **result == ValidationResult::Broken)
            .map(|(uri, _)| uri.as_str())
            .collect()
    }
}

impl Default for LinkValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper for RdfValue to string conversion in tests.
impl std::fmt::Display for RdfValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RdfValue::Uri(uri) => write!(f, "{}", uri),
            RdfValue::Literal(s, None) => write!(f, "{}", s),
            RdfValue::Literal(s, Some(lang)) => write!(f, "{}@{}", s, lang),
            RdfValue::TypedLiteral(s, dtype) => write!(f, "{}^^{}", s, dtype),
            RdfValue::BlankNode(id) => write!(f, "_:{}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn sample_statute() -> Statute {
        let mut statute = Statute::new(
            "test-act-2023",
            "Test Act 2023",
            Effect::new(EffectType::Grant, "Grant test rights"),
        );
        statute.jurisdiction = Some("UK".to_string());
        statute
    }

    #[test]
    fn test_cool_uri_hierarchical() {
        let scheme =
            CoolUriScheme::new("legislation.example.org").with_pattern(UriPattern::Hierarchical);
        let statute = sample_statute();
        let uri = scheme.generate_uri(&statute);

        assert!(uri.starts_with("https://legislation.example.org"));
        assert!(uri.contains("uk"));
        assert!(uri.contains("statute"));
        assert!(uri.contains("test-act-2023"));
    }

    #[test]
    fn test_cool_uri_eli_style() {
        let scheme =
            CoolUriScheme::new("legislation.example.org").with_pattern(UriPattern::EliStyle);
        let statute = sample_statute();
        let uri = scheme.generate_uri(&statute);

        assert!(uri.contains("/id/"));
        assert!(uri.contains("uk"));
        assert!(uri.contains("statute"));
    }

    #[test]
    fn test_cool_uri_flat() {
        let scheme = CoolUriScheme::new("legislation.example.org").with_pattern(UriPattern::Flat);
        let statute = sample_statute();
        let uri = scheme.generate_uri(&statute);

        assert!(uri.contains("/doc/"));
        assert!(uri.contains("uk-statute"));
    }

    #[test]
    fn test_cool_uri_http() {
        let scheme = CoolUriScheme::new("legislation.example.org").with_https(false);
        let statute = sample_statute();
        let uri = scheme.generate_uri(&statute);

        assert!(uri.starts_with("http://"));
    }

    #[test]
    fn test_version_uri() {
        let scheme = CoolUriScheme::new("legislation.example.org");
        let mut statute = sample_statute();
        statute.version = 2;
        let uri = scheme.generate_version_uri(&statute);

        assert!(uri.contains("/version/2"));
    }

    #[test]
    fn test_format_uri() {
        let scheme = CoolUriScheme::new("legislation.example.org");
        let statute = sample_statute();
        let uri = scheme.generate_format_uri(&statute, RdfFormat::Turtle);

        assert!(uri.ends_with(".ttl"));
    }

    #[test]
    fn test_uri_dereferencer() {
        let scheme = CoolUriScheme::new("legislation.example.org");
        let mut dereferencer = UriDereferencer::new(scheme.clone());

        let statute = sample_statute();
        let expected_uri = scheme.generate_uri(&statute);

        dereferencer.register(statute);

        let result = dereferencer.dereference(&expected_uri);
        assert!(result.is_ok());

        let dereferenced = result.unwrap();
        assert_eq!(dereferenced.id, "test-act-2023");
    }

    #[test]
    fn test_uri_dereferencer_not_found() {
        let scheme = CoolUriScheme::new("legislation.example.org");
        let dereferencer = UriDereferencer::new(scheme);

        let result = dereferencer.dereference("https://legislation.example.org/nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_content_negotiation() {
        let scheme = CoolUriScheme::new("legislation.example.org");
        let mut dereferencer = UriDereferencer::new(scheme.clone());

        let statute = sample_statute();
        let uri = scheme.generate_uri(&statute);

        dereferencer.register(statute);

        let (format, _) = dereferencer.resolve(&uri, "application/ld+json").unwrap();
        assert_eq!(format, RdfFormat::JsonLd);
    }

    #[test]
    fn test_entity_resolver_same_as() {
        let mut resolver = EntityResolver::new("https://example.org/");
        resolver.add_same_as("test-act", "http://data.europa.eu/eli/test");

        let triples = resolver.generate_same_as_triples("test-act");
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].predicate, "owl:sameAs");
    }

    #[test]
    fn test_entity_resolver_see_also() {
        let mut resolver = EntityResolver::new("https://example.org/");
        resolver.add_see_also("test-act", "http://dbpedia.org/resource/TestAct");

        let triples = resolver.generate_see_also_triples("test-act");
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].predicate, "rdfs:seeAlso");
    }

    #[test]
    fn test_entity_resolver_eurlex() {
        let mut resolver = EntityResolver::new("https://example.org/");
        resolver.link_to_eurlex("test-act", "32023R1234");

        let triples = resolver.generate_same_as_triples("test-act");
        assert_eq!(triples.len(), 1);
        assert!(triples[0].object.to_string().contains("data.europa.eu/eli"));
    }

    #[test]
    fn test_entity_resolver_wikidata() {
        let mut resolver = EntityResolver::new("https://example.org/");
        resolver.link_to_wikidata("test-act", "Q12345");

        let triples = resolver.generate_same_as_triples("test-act");
        assert!(
            triples[0]
                .object
                .to_string()
                .contains("wikidata.org/entity/Q12345")
        );
    }

    #[test]
    fn test_link_validator() {
        let mut validator = LinkValidator::new();
        validator.add_uri("https://example.org/test");
        validator.validate_all();

        let results = validator.get_results();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_extract_uris_from_triples() {
        let mut validator = LinkValidator::new();
        let triples = vec![
            Triple {
                subject: "http://example.org/statute/1".to_string(),
                predicate: "owl:sameAs".to_string(),
                object: RdfValue::Uri("http://data.europa.eu/eli/test".to_string()),
            },
            Triple {
                subject: "http://example.org/statute/1".to_string(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string("Test"),
            },
        ];

        validator.extract_uris_from_triples(&triples);
        assert_eq!(validator.uris.len(), 1);
        assert_eq!(validator.uris[0], "http://data.europa.eu/eli/test");
    }
}
