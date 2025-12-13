//! Legalis-LOD: Linked Open Data (RDF/TTL) export for Legalis-RS.
//!
//! This crate provides export functionality for legal statutes to RDF formats:
//! - Turtle (TTL) - Human-readable RDF format
//! - N-Triples (NT) - Line-based RDF format
//! - RDF/XML - XML serialization of RDF
//! - JSON-LD - JSON-based RDF format
//!
//! Vocabularies supported:
//! - ELI (European Legislation Identifier)
//! - FRBR (Functional Requirements for Bibliographic Records)
//! - Dublin Core (dc, dcterms)
//! - Custom Legalis ontology

use chrono::{DateTime, NaiveDate, Utc};
use legalis_core::{ComparisonOp, Condition, EffectType, Statute};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during LOD export.
#[derive(Debug, Error)]
pub enum LodError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Missing required metadata: {0}")]
    MissingMetadata(String),
}

/// Result type for LOD operations.
pub type LodResult<T> = Result<T, LodError>;

/// RDF serialization format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RdfFormat {
    /// Turtle format (TTL) - human-readable
    #[default]
    Turtle,
    /// N-Triples format
    NTriples,
    /// RDF/XML format
    RdfXml,
    /// JSON-LD format
    JsonLd,
}

impl RdfFormat {
    /// Returns the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Turtle => "ttl",
            Self::NTriples => "nt",
            Self::RdfXml => "rdf",
            Self::JsonLd => "jsonld",
        }
    }

    /// Returns the MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Turtle => "text/turtle",
            Self::NTriples => "application/n-triples",
            Self::RdfXml => "application/rdf+xml",
            Self::JsonLd => "application/ld+json",
        }
    }
}

/// Standard namespace prefixes for legal ontologies.
#[derive(Debug, Clone)]
pub struct Namespaces {
    /// Base URI for generated resources
    pub base: String,
    /// Custom namespace mappings
    pub custom: HashMap<String, String>,
}

impl Default for Namespaces {
    fn default() -> Self {
        Self {
            base: "https://example.org/legalis/".to_string(),
            custom: HashMap::new(),
        }
    }
}

impl Namespaces {
    /// Creates namespaces with a custom base URI.
    pub fn with_base(base: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            custom: HashMap::new(),
        }
    }

    /// Adds a custom namespace.
    pub fn add(&mut self, prefix: impl Into<String>, uri: impl Into<String>) {
        self.custom.insert(prefix.into(), uri.into());
    }

    /// Returns all standard prefixes for Turtle format.
    fn standard_prefixes() -> Vec<(&'static str, &'static str)> {
        vec![
            ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
            ("owl", "http://www.w3.org/2002/07/owl#"),
            ("xsd", "http://www.w3.org/2001/XMLSchema#"),
            ("dc", "http://purl.org/dc/elements/1.1/"),
            ("dcterms", "http://purl.org/dc/terms/"),
            ("eli", "http://data.europa.eu/eli/ontology#"),
            ("frbr", "http://purl.org/vocab/frbr/core#"),
            ("legalis", "https://legalis.dev/ontology#"),
        ]
    }
}

/// RDF triple representation.
#[derive(Debug, Clone)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: RdfValue,
}

/// RDF object value types.
#[derive(Debug, Clone)]
pub enum RdfValue {
    /// URI reference
    Uri(String),
    /// Literal with optional language tag
    Literal(String, Option<String>),
    /// Typed literal
    TypedLiteral(String, String),
    /// Blank node
    BlankNode(String),
}

impl RdfValue {
    /// Creates a string literal.
    pub fn string(s: impl Into<String>) -> Self {
        Self::Literal(s.into(), None)
    }

    /// Creates a string literal with language tag.
    pub fn lang_string(s: impl Into<String>, lang: impl Into<String>) -> Self {
        Self::Literal(s.into(), Some(lang.into()))
    }

    /// Creates an integer literal.
    pub fn integer(n: i64) -> Self {
        Self::TypedLiteral(n.to_string(), "xsd:integer".to_string())
    }

    /// Creates a boolean literal.
    pub fn boolean(b: bool) -> Self {
        Self::TypedLiteral(b.to_string(), "xsd:boolean".to_string())
    }

    /// Creates a date literal from NaiveDate.
    pub fn date(d: NaiveDate) -> Self {
        Self::TypedLiteral(d.format("%Y-%m-%d").to_string(), "xsd:date".to_string())
    }

    /// Creates a date literal from DateTime.
    pub fn datetime(d: DateTime<Utc>) -> Self {
        Self::TypedLiteral(d.to_rfc3339(), "xsd:dateTime".to_string())
    }

    /// Creates a date literal from DateTime (alias for consistency).
    pub fn date_from_datetime(d: DateTime<Utc>) -> Self {
        Self::TypedLiteral(d.format("%Y-%m-%d").to_string(), "xsd:date".to_string())
    }
}

/// LOD exporter for legal statutes.
#[derive(Debug)]
pub struct LodExporter {
    format: RdfFormat,
    namespaces: Namespaces,
}

impl LodExporter {
    /// Creates a new exporter with the specified format.
    pub fn new(format: RdfFormat) -> Self {
        Self {
            format,
            namespaces: Namespaces::default(),
        }
    }

    /// Creates a new exporter with custom namespaces.
    pub fn with_namespaces(format: RdfFormat, namespaces: Namespaces) -> Self {
        Self { format, namespaces }
    }

    /// Sets the base URI.
    pub fn set_base(&mut self, base: impl Into<String>) {
        self.namespaces.base = base.into();
    }

    /// Exports a statute to the configured RDF format.
    pub fn export(&self, statute: &Statute) -> LodResult<String> {
        let triples = self.statute_to_triples(statute)?;

        match self.format {
            RdfFormat::Turtle => self.to_turtle(&triples),
            RdfFormat::NTriples => self.to_ntriples(&triples),
            RdfFormat::RdfXml => self.to_rdf_xml(&triples),
            RdfFormat::JsonLd => self.to_json_ld(&triples, statute),
        }
    }

    /// Exports multiple statutes to the configured RDF format.
    pub fn export_batch(&self, statutes: &[Statute]) -> LodResult<String> {
        let mut all_triples = Vec::new();
        for statute in statutes {
            all_triples.extend(self.statute_to_triples(statute)?);
        }

        match self.format {
            RdfFormat::Turtle => self.to_turtle(&all_triples),
            RdfFormat::NTriples => self.to_ntriples(&all_triples),
            RdfFormat::RdfXml => self.to_rdf_xml(&all_triples),
            RdfFormat::JsonLd => self.to_json_ld_batch(&all_triples, statutes),
        }
    }

    fn statute_to_triples(&self, statute: &Statute) -> LodResult<Vec<Triple>> {
        let mut triples = Vec::new();
        let subject = format!(
            "{}statute/{}",
            self.namespaces.base,
            escape_uri(&statute.id)
        );

        // Type declaration
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("eli:LegalResource".to_string()),
        });

        triples.push(Triple {
            subject: subject.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        });

        // Title (using ELI and Dublin Core)
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "eli:title".to_string(),
            object: RdfValue::string(&statute.title),
        });

        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&statute.title),
        });

        // Identifier
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&statute.id),
        });

        // Effect
        let effect_uri = format!("{}effect/{}", self.namespaces.base, escape_uri(&statute.id));
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "legalis:hasEffect".to_string(),
            object: RdfValue::Uri(effect_uri.clone()),
        });

        triples.push(Triple {
            subject: effect_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Effect".to_string()),
        });

        triples.push(Triple {
            subject: effect_uri.clone(),
            predicate: "legalis:effectType".to_string(),
            object: RdfValue::Uri(format!(
                "legalis:{}",
                effect_type_to_uri(&statute.effect.effect_type)
            )),
        });

        triples.push(Triple {
            subject: effect_uri,
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&statute.effect.description),
        });

        // Preconditions
        for (i, condition) in statute.preconditions.iter().enumerate() {
            let condition_uri = format!(
                "{}condition/{}/{}",
                self.namespaces.base,
                escape_uri(&statute.id),
                i
            );
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "legalis:hasPrecondition".to_string(),
                object: RdfValue::Uri(condition_uri.clone()),
            });
            triples.extend(self.condition_to_triples(&condition_uri, condition));
        }

        // Jurisdiction
        if let Some(ref jurisdiction) = statute.jurisdiction {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "eli:jurisdiction".to_string(),
                object: RdfValue::string(jurisdiction),
            });
        }

        // Version
        if statute.version > 0 {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "eli:version".to_string(),
                object: RdfValue::integer(statute.version as i64),
            });
        }

        // Temporal validity
        if let Some(effective_date) = statute.temporal_validity.effective_date {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "eli:date_document".to_string(),
                object: RdfValue::date(effective_date),
            });
        }

        if let Some(expiry_date) = statute.temporal_validity.expiry_date {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "legalis:expiryDate".to_string(),
                object: RdfValue::date(expiry_date),
            });
        }

        // Discretion indicator
        if statute.discretion_logic.is_some() {
            triples.push(Triple {
                subject,
                predicate: "legalis:hasDiscretion".to_string(),
                object: RdfValue::boolean(true),
            });
        }

        Ok(triples)
    }

    fn condition_to_triples(&self, uri: &str, condition: &Condition) -> Vec<Triple> {
        condition_to_triples_impl(uri, condition)
    }

    fn to_turtle(&self, triples: &[Triple]) -> LodResult<String> {
        let mut output = String::new();

        // Prefixes
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push_str(&format!("@base <{}> .\n", self.namespaces.base));
        for (prefix, uri) in &self.namespaces.custom {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');

        // Group triples by subject for prettier output
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        for (subject, subject_triples) in by_subject {
            let subject_str = if subject.starts_with(&self.namespaces.base) {
                format!("<{}>", subject)
            } else if let Some(prefixed) = try_prefix(subject) {
                prefixed
            } else {
                format!("<{}>", subject)
            };

            output.push_str(&subject_str);

            for (i, triple) in subject_triples.iter().enumerate() {
                let sep = if i == 0 { "\n    " } else { " ;\n    " };
                output.push_str(sep);
                output.push_str(&triple.predicate);
                output.push(' ');
                output.push_str(&self.value_to_turtle(&triple.object));
            }
            output.push_str(" .\n\n");
        }

        Ok(output)
    }

    fn value_to_turtle(&self, value: &RdfValue) -> String {
        match value {
            RdfValue::Uri(uri) => {
                if let Some(prefixed) = try_prefix(uri) {
                    prefixed
                } else {
                    format!("<{}>", uri)
                }
            }
            RdfValue::Literal(s, None) => format!("\"{}\"", escape_string(s)),
            RdfValue::Literal(s, Some(lang)) => format!("\"{}\"@{}", escape_string(s), lang),
            RdfValue::TypedLiteral(s, dtype) => {
                if dtype == "xsd:integer" || dtype == "xsd:boolean" {
                    s.clone()
                } else {
                    format!("\"{}\"^^{}", escape_string(s), dtype)
                }
            }
            RdfValue::BlankNode(id) => format!("_:{}", id),
        }
    }

    fn to_ntriples(&self, triples: &[Triple]) -> LodResult<String> {
        let mut output = String::new();

        for triple in triples {
            let subject = expand_uri(&triple.subject, &self.namespaces);
            let predicate = expand_uri(&triple.predicate, &self.namespaces);
            let object = self.value_to_ntriples(&triple.object);

            output.push_str(&format!("<{}> <{}> {} .\n", subject, predicate, object));
        }

        Ok(output)
    }

    fn value_to_ntriples(&self, value: &RdfValue) -> String {
        match value {
            RdfValue::Uri(uri) => format!("<{}>", expand_uri(uri, &self.namespaces)),
            RdfValue::Literal(s, None) => format!("\"{}\"", escape_string(s)),
            RdfValue::Literal(s, Some(lang)) => format!("\"{}\"@{}", escape_string(s), lang),
            RdfValue::TypedLiteral(s, dtype) => {
                let full_type = expand_uri(dtype, &self.namespaces);
                format!("\"{}\"^^<{}>", escape_string(s), full_type)
            }
            RdfValue::BlankNode(id) => format!("_:{}", id),
        }
    }

    fn to_rdf_xml(&self, triples: &[Triple]) -> LodResult<String> {
        let mut output = String::new();
        output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        output.push_str("<rdf:RDF\n");
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("    xmlns:{}=\"{}\"\n", prefix, uri));
        }
        output.push_str(&format!("    xml:base=\"{}\">\n\n", self.namespaces.base));

        // Group by subject
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        for (subject, subject_triples) in by_subject {
            // Find the primary type
            let rdf_type = subject_triples
                .iter()
                .find(|t| t.predicate == "rdf:type")
                .map(|t| match &t.object {
                    RdfValue::Uri(u) => u.clone(),
                    _ => "rdf:Description".to_string(),
                })
                .unwrap_or_else(|| "rdf:Description".to_string());

            output.push_str(&format!("  <{} rdf:about=\"{}\">\n", rdf_type, subject));

            for triple in subject_triples {
                if triple.predicate == "rdf:type" {
                    continue; // Already handled
                }
                output.push_str(&format!("    {}\n", self.triple_to_rdf_xml_element(triple)));
            }

            output.push_str(&format!("  </{}>\n\n", rdf_type));
        }

        output.push_str("</rdf:RDF>\n");
        Ok(output)
    }

    fn triple_to_rdf_xml_element(&self, triple: &Triple) -> String {
        match &triple.object {
            RdfValue::Uri(uri) => {
                format!("<{} rdf:resource=\"{}\"/>", triple.predicate, uri)
            }
            RdfValue::Literal(s, None) => {
                format!(
                    "<{}>{}</{}>",
                    triple.predicate,
                    escape_xml(s),
                    triple.predicate
                )
            }
            RdfValue::Literal(s, Some(lang)) => {
                format!(
                    "<{} xml:lang=\"{}\">{}</{}>",
                    triple.predicate,
                    lang,
                    escape_xml(s),
                    triple.predicate
                )
            }
            RdfValue::TypedLiteral(s, dtype) => {
                format!(
                    "<{} rdf:datatype=\"{}\">{}</{}>",
                    triple.predicate,
                    expand_uri(dtype, &self.namespaces),
                    escape_xml(s),
                    triple.predicate
                )
            }
            RdfValue::BlankNode(id) => {
                format!("<{} rdf:nodeID=\"{}\"/>", triple.predicate, id)
            }
        }
    }

    fn to_json_ld(&self, triples: &[Triple], statute: &Statute) -> LodResult<String> {
        let mut doc = serde_json::Map::new();

        // Context
        let mut context = serde_json::Map::new();
        for (prefix, uri) in Namespaces::standard_prefixes() {
            context.insert(prefix.to_string(), serde_json::json!(uri));
        }
        doc.insert("@context".to_string(), serde_json::Value::Object(context));

        // ID and type
        doc.insert(
            "@id".to_string(),
            serde_json::json!(format!(
                "{}statute/{}",
                self.namespaces.base,
                escape_uri(&statute.id)
            )),
        );
        doc.insert(
            "@type".to_string(),
            serde_json::json!(["eli:LegalResource", "legalis:Statute"]),
        );

        // Properties from triples
        self.add_json_ld_properties(&mut doc, triples);

        serde_json::to_string_pretty(&doc).map_err(|e| LodError::SerializationError(e.to_string()))
    }

    fn to_json_ld_batch(&self, triples: &[Triple], statutes: &[Statute]) -> LodResult<String> {
        let mut graph = Vec::new();

        for statute in statutes {
            let statute_triples: Vec<&Triple> = triples
                .iter()
                .filter(|t| t.subject.contains(&statute.id))
                .collect();

            let mut doc = serde_json::Map::new();
            doc.insert(
                "@id".to_string(),
                serde_json::json!(format!(
                    "{}statute/{}",
                    self.namespaces.base,
                    escape_uri(&statute.id)
                )),
            );
            self.add_json_ld_properties(
                &mut doc,
                &statute_triples.iter().copied().cloned().collect::<Vec<_>>(),
            );
            graph.push(serde_json::Value::Object(doc));
        }

        let mut result = serde_json::Map::new();
        let mut context = serde_json::Map::new();
        for (prefix, uri) in Namespaces::standard_prefixes() {
            context.insert(prefix.to_string(), serde_json::json!(uri));
        }
        result.insert("@context".to_string(), serde_json::Value::Object(context));
        result.insert("@graph".to_string(), serde_json::Value::Array(graph));

        serde_json::to_string_pretty(&result)
            .map_err(|e| LodError::SerializationError(e.to_string()))
    }

    fn add_json_ld_properties(
        &self,
        doc: &mut serde_json::Map<String, serde_json::Value>,
        triples: &[Triple],
    ) {
        for triple in triples {
            if triple.predicate == "rdf:type" {
                continue; // Handle types separately
            }

            let value = match &triple.object {
                RdfValue::Uri(uri) => serde_json::json!({"@id": uri}),
                RdfValue::Literal(s, None) => serde_json::json!(s),
                RdfValue::Literal(s, Some(lang)) => {
                    serde_json::json!({"@value": s, "@language": lang})
                }
                RdfValue::TypedLiteral(s, dtype) => {
                    serde_json::json!({"@value": s, "@type": dtype})
                }
                RdfValue::BlankNode(id) => serde_json::json!({"@id": format!("_:{}", id)}),
            };

            doc.insert(triple.predicate.clone(), value);
        }
    }
}

/// Converts a condition to RDF triples (standalone function to avoid clippy recursion warning).
fn condition_to_triples_impl(uri: &str, condition: &Condition) -> Vec<Triple> {
    let mut triples = Vec::new();

    triples.push(Triple {
        subject: uri.to_string(),
        predicate: "rdf:type".to_string(),
        object: RdfValue::Uri("legalis:Condition".to_string()),
    });

    match condition {
        Condition::Age { operator, value } => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:AgeCondition".to_string()),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:operator".to_string(),
                object: RdfValue::Uri(format!("legalis:{}", operator_to_uri(*operator))),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:value".to_string(),
                object: RdfValue::integer(*value as i64),
            });
        }
        Condition::Income { operator, value } => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:IncomeCondition".to_string()),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:operator".to_string(),
                object: RdfValue::Uri(format!("legalis:{}", operator_to_uri(*operator))),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:value".to_string(),
                object: RdfValue::integer(*value as i64),
            });
        }
        Condition::HasAttribute { key } => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:AttributeCondition".to_string()),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:attributeKey".to_string(),
                object: RdfValue::string(key),
            });
        }
        Condition::And(left, right) => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:AndCondition".to_string()),
            });
            let left_uri = format!("{}/left", uri);
            let right_uri = format!("{}/right", uri);
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:leftOperand".to_string(),
                object: RdfValue::Uri(left_uri.clone()),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:rightOperand".to_string(),
                object: RdfValue::Uri(right_uri.clone()),
            });
            triples.extend(condition_to_triples_impl(&left_uri, left));
            triples.extend(condition_to_triples_impl(&right_uri, right));
        }
        Condition::Or(left, right) => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:OrCondition".to_string()),
            });
            let left_uri = format!("{}/left", uri);
            let right_uri = format!("{}/right", uri);
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:leftOperand".to_string(),
                object: RdfValue::Uri(left_uri.clone()),
            });
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:rightOperand".to_string(),
                object: RdfValue::Uri(right_uri.clone()),
            });
            triples.extend(condition_to_triples_impl(&left_uri, left));
            triples.extend(condition_to_triples_impl(&right_uri, right));
        }
        Condition::Not(inner) => {
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:NotCondition".to_string()),
            });
            let inner_uri = format!("{}/inner", uri);
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "legalis:operand".to_string(),
                object: RdfValue::Uri(inner_uri.clone()),
            });
            triples.extend(condition_to_triples_impl(&inner_uri, inner));
        }
        _ => {
            // Generic condition representation
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(format!("{:?}", condition)),
            });
        }
    }

    triples
}

/// Escapes a string for URI usage.
fn escape_uri(s: &str) -> String {
    s.replace(' ', "_").replace('/', "-").replace('&', "-and-")
}

/// Escapes a string for Turtle/N-Triples.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escapes a string for XML.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Tries to convert a URI to prefixed form.
fn try_prefix(uri: &str) -> Option<String> {
    for (prefix, ns) in Namespaces::standard_prefixes() {
        if let Some(suffix) = uri.strip_prefix(ns) {
            return Some(format!("{}:{}", prefix, suffix));
        }
        // Also handle already-prefixed URIs
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return Some(uri.to_string());
        }
    }
    None
}

/// Expands a prefixed URI to full form.
fn expand_uri(uri: &str, namespaces: &Namespaces) -> String {
    for (prefix, ns) in Namespaces::standard_prefixes() {
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return format!("{}{}", ns, &uri[prefix_colon.len()..]);
        }
    }
    // Check custom namespaces
    for (prefix, ns) in &namespaces.custom {
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return format!("{}{}", ns, &uri[prefix_colon.len()..]);
        }
    }
    uri.to_string()
}

/// Converts an effect type to URI fragment.
fn effect_type_to_uri(effect_type: &EffectType) -> &'static str {
    match effect_type {
        EffectType::Grant => "GrantEffect",
        EffectType::Revoke => "RevokeEffect",
        EffectType::MonetaryTransfer => "MonetaryTransferEffect",
        EffectType::Obligation => "ObligationEffect",
        EffectType::Prohibition => "ProhibitionEffect",
        EffectType::StatusChange => "StatusChangeEffect",
        EffectType::Custom => "CustomEffect",
    }
}

/// Converts a comparison operator to URI fragment.
fn operator_to_uri(op: ComparisonOp) -> &'static str {
    match op {
        ComparisonOp::Equal => "Equal",
        ComparisonOp::NotEqual => "NotEqual",
        ComparisonOp::GreaterThan => "GreaterThan",
        ComparisonOp::GreaterOrEqual => "GreaterOrEqual",
        ComparisonOp::LessThan => "LessThan",
        ComparisonOp::LessOrEqual => "LessOrEqual",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn sample_statute() -> Statute {
        Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_export_turtle() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("@prefix eli:"));
        assert!(output.contains("@prefix legalis:"));
        assert!(output.contains("eli:LegalResource"));
        assert!(output.contains("Adult Rights Act"));
    }

    #[test]
    fn test_export_ntriples() {
        let exporter = LodExporter::new(RdfFormat::NTriples);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("<http://data.europa.eu/eli/ontology#LegalResource>"));
        assert!(output.contains("Adult Rights Act"));
    }

    #[test]
    fn test_export_rdf_xml() {
        let exporter = LodExporter::new(RdfFormat::RdfXml);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("<?xml version"));
        assert!(output.contains("rdf:RDF"));
        assert!(output.contains("Adult Rights Act"));
    }

    #[test]
    fn test_export_json_ld() {
        let exporter = LodExporter::new(RdfFormat::JsonLd);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("\"@context\""));
        assert!(output.contains("\"@id\""));
        assert!(output.contains("Adult Rights Act"));
    }

    #[test]
    fn test_condition_triples() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = Statute::new(
            "complex-law",
            "Complex Law",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let output = exporter.export(&statute).unwrap();
        assert!(output.contains("legalis:AndCondition"));
        assert!(output.contains("legalis:AgeCondition"));
        assert!(output.contains("legalis:IncomeCondition"));
    }

    #[test]
    fn test_custom_namespace() {
        let mut namespaces = Namespaces::with_base("https://law.example.jp/");
        namespaces.add("jplaw", "https://law.example.jp/ontology#");

        let exporter = LodExporter::with_namespaces(RdfFormat::Turtle, namespaces);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("https://law.example.jp/statute/adult-rights"));
    }

    #[test]
    fn test_format_extensions() {
        assert_eq!(RdfFormat::Turtle.extension(), "ttl");
        assert_eq!(RdfFormat::NTriples.extension(), "nt");
        assert_eq!(RdfFormat::RdfXml.extension(), "rdf");
        assert_eq!(RdfFormat::JsonLd.extension(), "jsonld");
    }

    #[test]
    fn test_escape_uri() {
        assert_eq!(escape_uri("hello world"), "hello_world");
        assert_eq!(escape_uri("a/b"), "a-b");
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_string("say \"hi\""), "say \\\"hi\\\"");
    }

    #[test]
    fn test_batch_export() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "minor-protection",
                "Minor Protection Act",
                Effect::new(EffectType::Grant, "Protection rights"),
            ),
        ];

        let output = exporter.export_batch(&statutes).unwrap();
        assert!(output.contains("adult-rights"));
        assert!(output.contains("minor-protection"));
    }
}
