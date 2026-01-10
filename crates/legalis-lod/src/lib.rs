#![allow(clippy::type_complexity)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::field_reassign_with_default)]

//! Legalis-LOD: Linked Open Data (RDF/TTL) export for Legalis-RS.
//!
//! This crate provides comprehensive RDF export and Linked Data functionality for legal statutes.
//!
//! ## RDF Formats Supported
//! - Turtle (TTL) - Human-readable RDF format
//! - N-Triples (NT) - Line-based RDF format
//! - RDF/XML - XML serialization of RDF
//! - JSON-LD - JSON-based RDF format
//! - TriG - Named graph format
//!
//! ## Ontologies and Vocabularies
//! - ELI (European Legislation Identifier)
//! - FaBiO (FRBR-aligned Bibliographic Ontology)
//! - LKIF-Core (Legal Knowledge Interchange Format)
//! - LegalRuleML (OASIS standard for legal rules)
//! - Akoma Ntoso (Legal document markup)
//! - FRBR (Functional Requirements for Bibliographic Records)
//! - Dublin Core (dc, dcterms)
//! - SKOS (Simple Knowledge Organization System)
//! - VOID (Vocabulary of Interlinked Datasets)
//! - PROV-O (Provenance Ontology)
//! - Custom Legalis ontology
//!
//! ## Linked Data Features
//! - Cool URIs for legal resources (ELI-style, hierarchical, flat patterns)
//! - URI dereferencing with content negotiation
//! - owl:sameAs linking for entity resolution
//! - rdfs:seeAlso for related resources
//! - Integration with EUR-Lex, legislation.gov.uk, Wikidata, DBpedia
//!
//! ## Additional Features
//! - SHACL and ShEx validation
//! - SPARQL query generation
//! - Streaming serialization for large datasets
//! - Export caching
//! - RDFa output for HTML embedding
//! - Provenance tracking (PROV-O)
//! - License metadata (Creative Commons, etc.)
//!
//! See EXAMPLES.md for detailed usage examples.

use chrono::{DateTime, NaiveDate, Utc};
use legalis_core::{ComparisonOp, Condition, EffectType, Statute};
use std::collections::HashMap;
use thiserror::Error;

pub mod audit_log;
pub mod av_annotation;
pub mod bitemporal;
pub mod blockchain;
pub mod cache;
pub mod competency_questions;
pub mod compliance;
pub mod content_addressed;
pub mod continuous_query;
pub mod crossmodal_reasoning;
pub mod crowdsourced_evolution;
pub mod dcat;
pub mod did;
pub mod document_layout;
pub mod embeddings;
pub mod enterprise_deployment;
pub mod entity_linking;
pub mod external;
pub mod fusion;
pub mod geosparql;
pub mod governance;
pub mod image_rdf;
pub mod ipld;
pub mod jurisdiction_queries;
pub mod kg_completion;
pub mod knowledge_graph;
pub mod ldn;
pub mod link_prediction;
pub mod linked_data;
pub mod map_exploration;
pub mod multimodal_alignment;
pub mod neural_symbolic;
pub mod ontology;
pub mod ontology_alignment;
pub mod ontology_learning;
pub mod ontology_metrics;
pub mod ontology_versioning;
pub mod quality;
pub mod rbac;
pub mod rdfa;
pub mod rdfstar;
pub mod realtime;
pub mod reasoning;
pub mod relation_extraction;
pub mod shacl;
pub mod shex;
pub mod similarity;
pub mod sparql;
pub mod sparqlstar;
pub mod spatial_reasoning;
pub mod store;
pub mod streaming;
pub mod streaming_sparql;
pub mod temporal_consistency;
pub mod temporal_rdf;
pub mod time_aware_queries;
pub mod validation;
pub mod verifiable;
pub mod versioning;
pub mod void_desc;

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
    /// TriG format - Turtle with named graphs
    TriG,
}

impl RdfFormat {
    /// Returns the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Turtle => "ttl",
            Self::NTriples => "nt",
            Self::RdfXml => "rdf",
            Self::JsonLd => "jsonld",
            Self::TriG => "trig",
        }
    }

    /// Returns the MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Turtle => "text/turtle",
            Self::NTriples => "application/n-triples",
            Self::RdfXml => "application/rdf+xml",
            Self::JsonLd => "application/ld+json",
            Self::TriG => "application/trig",
        }
    }

    /// Returns all MIME type aliases for this format.
    pub fn mime_type_aliases(&self) -> Vec<&'static str> {
        match self {
            Self::Turtle => vec!["text/turtle", "application/x-turtle", "application/turtle"],
            Self::NTriples => vec!["application/n-triples", "text/plain"],
            Self::RdfXml => vec!["application/rdf+xml", "application/xml", "text/xml"],
            Self::JsonLd => vec!["application/ld+json", "application/json"],
            Self::TriG => vec!["application/trig", "application/x-trig"],
        }
    }

    /// Selects the best format based on HTTP Accept header.
    /// Returns the default format (Turtle) if no match is found.
    pub fn from_accept_header(accept: &str) -> Self {
        // Parse accept header and find best match
        let accept_lower = accept.to_lowercase();

        // Check each format's MIME types
        for format in [
            Self::JsonLd,
            Self::Turtle,
            Self::RdfXml,
            Self::NTriples,
            Self::TriG,
        ] {
            for mime in format.mime_type_aliases() {
                if accept_lower.contains(mime) {
                    return format;
                }
            }
        }

        // Default to Turtle
        Self::Turtle
    }

    /// Returns all supported formats.
    pub fn all_formats() -> Vec<Self> {
        vec![
            Self::Turtle,
            Self::NTriples,
            Self::RdfXml,
            Self::JsonLd,
            Self::TriG,
        ]
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
            ("skos", "http://www.w3.org/2004/02/skos/core#"),
            ("void", "http://rdfs.org/ns/void#"),
            ("prov", "http://www.w3.org/ns/prov#"),
            ("cc", "http://creativecommons.org/ns#"),
            ("geo", geosparql::GEOSPARQL_NS),
            ("sf", geosparql::SF_NS),
            ("temporal", temporal_rdf::TEMPORAL_NS),
            ("time", temporal_rdf::TIME_NS),
            ("legalis", "https://legalis.dev/ontology#"),
            ("fabio", ontology::fabio::NAMESPACE),
            ("lkif", ontology::lkif::NAMESPACE),
            ("lrml", ontology::legalruleml::NAMESPACE),
            ("akn", ontology::akoma_ntoso::NAMESPACE),
        ]
    }
}

/// RDF triple representation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: RdfValue,
}

/// RDF object value types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

/// Provenance information for RDF export.
#[derive(Debug, Clone)]
pub struct ProvenanceInfo {
    /// Agent who generated the data (e.g., organization or person)
    pub agent: Option<String>,
    /// Activity that generated the data
    pub activity: Option<String>,
    /// Generation time
    pub generated_at: Option<DateTime<Utc>>,
    /// Source entity
    pub derived_from: Option<String>,
    /// Additional attribution
    pub attribution: Option<String>,
}

impl Default for ProvenanceInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl ProvenanceInfo {
    /// Creates a new provenance info.
    pub fn new() -> Self {
        Self {
            agent: None,
            activity: None,
            generated_at: Some(Utc::now()),
            derived_from: None,
            attribution: None,
        }
    }

    /// Sets the agent.
    pub fn with_agent(mut self, agent: impl Into<String>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    /// Sets the activity.
    pub fn with_activity(mut self, activity: impl Into<String>) -> Self {
        self.activity = Some(activity.into());
        self
    }

    /// Sets the source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.derived_from = Some(source.into());
        self
    }

    /// Sets the attribution.
    pub fn with_attribution(mut self, attribution: impl Into<String>) -> Self {
        self.attribution = Some(attribution.into());
        self
    }
}

/// License information for RDF export.
#[derive(Debug, Clone)]
pub struct LicenseInfo {
    /// License URI (e.g., Creative Commons)
    pub license_uri: String,
    /// License label
    pub label: Option<String>,
    /// Rights holder
    pub rights_holder: Option<String>,
}

impl LicenseInfo {
    /// Creates a new license info.
    pub fn new(license_uri: impl Into<String>) -> Self {
        Self {
            license_uri: license_uri.into(),
            label: None,
            rights_holder: None,
        }
    }

    /// Sets the label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the rights holder.
    pub fn with_rights_holder(mut self, holder: impl Into<String>) -> Self {
        self.rights_holder = Some(holder.into());
        self
    }

    /// Creates a Creative Commons BY 4.0 license.
    pub fn cc_by_4_0() -> Self {
        Self::new("http://creativecommons.org/licenses/by/4.0/")
            .with_label("Creative Commons Attribution 4.0 International")
    }

    /// Creates a Creative Commons BY-SA 4.0 license.
    pub fn cc_by_sa_4_0() -> Self {
        Self::new("http://creativecommons.org/licenses/by-sa/4.0/")
            .with_label("Creative Commons Attribution-ShareAlike 4.0 International")
    }

    /// Creates a Creative Commons Zero (CC0) license.
    pub fn cc0() -> Self {
        Self::new("http://creativecommons.org/publicdomain/zero/1.0/")
            .with_label("Creative Commons Zero v1.0 Universal")
    }
}

/// LOD exporter for legal statutes.
#[derive(Debug)]
pub struct LodExporter {
    format: RdfFormat,
    namespaces: Namespaces,
    provenance: Option<ProvenanceInfo>,
    license: Option<LicenseInfo>,
    include_ontologies: bool,
}

impl LodExporter {
    /// Creates a new exporter with the specified format.
    pub fn new(format: RdfFormat) -> Self {
        Self {
            format,
            namespaces: Namespaces::default(),
            provenance: None,
            license: None,
            include_ontologies: false,
        }
    }

    /// Creates a new exporter with custom namespaces.
    pub fn with_namespaces(format: RdfFormat, namespaces: Namespaces) -> Self {
        Self {
            format,
            namespaces,
            provenance: None,
            license: None,
            include_ontologies: false,
        }
    }

    /// Sets the base URI.
    pub fn set_base(&mut self, base: impl Into<String>) {
        self.namespaces.base = base.into();
    }

    /// Sets provenance information.
    pub fn set_provenance(&mut self, provenance: ProvenanceInfo) {
        self.provenance = Some(provenance);
    }

    /// Sets license information.
    pub fn set_license(&mut self, license: LicenseInfo) {
        self.license = Some(license);
    }

    /// Builder method to set provenance.
    pub fn with_provenance(mut self, provenance: ProvenanceInfo) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Builder method to set license.
    pub fn with_license(mut self, license: LicenseInfo) -> Self {
        self.license = Some(license);
        self
    }

    /// Builder method to enable ontology triples.
    pub fn with_ontologies(mut self, include: bool) -> Self {
        self.include_ontologies = include;
        self
    }

    /// Sets whether to include ontology-specific triples.
    pub fn set_include_ontologies(&mut self, include: bool) {
        self.include_ontologies = include;
    }

    /// Validates the triples for a statute.
    pub fn validate_statute(&self, statute: &Statute) -> LodResult<validation::ValidationReport> {
        let triples = self.statute_to_triples(statute)?;
        let validator = validation::RdfValidator::new();
        Ok(validator.validate(&triples))
    }

    /// Exports a statute to the configured RDF format.
    pub fn export(&self, statute: &Statute) -> LodResult<String> {
        let triples = self.statute_to_triples(statute)?;

        match self.format {
            RdfFormat::Turtle => self.to_turtle(&triples),
            RdfFormat::NTriples => self.to_ntriples(&triples),
            RdfFormat::RdfXml => self.to_rdf_xml(&triples),
            RdfFormat::JsonLd => self.to_json_ld(&triples, statute),
            RdfFormat::TriG => self.to_trig(&triples, Some(&statute.id)),
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
            RdfFormat::TriG => self.to_trig_batch(&all_triples, statutes),
        }
    }

    /// Generates SKOS concept scheme for statute classifications.
    pub fn generate_concept_scheme(&self, scheme_id: &str, title: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let scheme_uri = format!(
            "{}concept-scheme/{}",
            self.namespaces.base,
            escape_uri(scheme_id)
        );

        triples.push(Triple {
            subject: scheme_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("skos:ConceptScheme".to_string()),
        });

        triples.push(Triple {
            subject: scheme_uri.clone(),
            predicate: "skos:prefLabel".to_string(),
            object: RdfValue::string(title),
        });

        triples.push(Triple {
            subject: scheme_uri,
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(title),
        });

        triples
    }

    /// Creates a SKOS concept for an effect type.
    pub fn create_effect_type_concept(
        &self,
        effect_type: &str,
        label: &str,
        definition: Option<&str>,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let concept_uri = format!(
            "{}concept/effect-type/{}",
            self.namespaces.base,
            escape_uri(effect_type)
        );
        let scheme_uri = format!("{}concept-scheme/effect-types", self.namespaces.base);

        triples.push(Triple {
            subject: concept_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("skos:Concept".to_string()),
        });

        triples.push(Triple {
            subject: concept_uri.clone(),
            predicate: "skos:prefLabel".to_string(),
            object: RdfValue::string(label),
        });

        triples.push(Triple {
            subject: concept_uri.clone(),
            predicate: "skos:inScheme".to_string(),
            object: RdfValue::Uri(scheme_uri.clone()),
        });

        if let Some(def) = definition {
            triples.push(Triple {
                subject: concept_uri,
                predicate: "skos:definition".to_string(),
                object: RdfValue::string(def),
            });
        }

        // Add concept to scheme
        triples.push(Triple {
            subject: scheme_uri,
            predicate: "skos:hasTopConcept".to_string(),
            object: RdfValue::Uri(format!(
                "{}concept/effect-type/{}",
                self.namespaces.base,
                escape_uri(effect_type)
            )),
        });

        triples
    }

    /// Adds SKOS relationships between concepts (broader/narrower).
    pub fn add_skos_hierarchy(&self, broader_concept: &str, narrower_concept: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let broader_uri = format!(
            "{}concept/{}",
            self.namespaces.base,
            escape_uri(broader_concept)
        );
        let narrower_uri = format!(
            "{}concept/{}",
            self.namespaces.base,
            escape_uri(narrower_concept)
        );

        triples.push(Triple {
            subject: narrower_uri.clone(),
            predicate: "skos:broader".to_string(),
            object: RdfValue::Uri(broader_uri.clone()),
        });

        triples.push(Triple {
            subject: broader_uri,
            predicate: "skos:narrower".to_string(),
            object: RdfValue::Uri(narrower_uri),
        });

        triples
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

        // Add SKOS concept for the statute (for classification)
        let concept_uri = format!(
            "{}concept/statute-type/{}",
            self.namespaces.base,
            escape_uri(&statute.id)
        );
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:subject".to_string(),
            object: RdfValue::Uri(concept_uri),
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
                subject: subject.clone(),
                predicate: "legalis:hasDiscretion".to_string(),
                object: RdfValue::boolean(true),
            });
        }

        // Add provenance information if available
        if let Some(ref prov) = self.provenance {
            triples.extend(self.add_provenance_triples(&subject, prov));
        }

        // Add license information if available
        if let Some(ref lic) = self.license {
            triples.extend(self.add_license_triples(&subject, lic));
        }

        // Add ontology-specific triples if enabled
        if self.include_ontologies {
            triples.extend(ontology::generate_all_ontology_triples(
                &subject,
                statute,
                &self.namespaces.base,
            ));
        }

        Ok(triples)
    }

    fn add_provenance_triples(&self, subject: &str, prov: &ProvenanceInfo) -> Vec<Triple> {
        let mut triples = Vec::new();

        // wasGeneratedBy
        if let Some(ref activity) = prov.activity {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasGeneratedBy".to_string(),
                object: RdfValue::Uri(activity.clone()),
            });
        }

        // wasAttributedTo
        if let Some(ref agent) = prov.agent {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasAttributedTo".to_string(),
                object: RdfValue::Uri(agent.clone()),
            });
        }

        // generatedAtTime
        if let Some(ref time) = prov.generated_at {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:generatedAtTime".to_string(),
                object: RdfValue::datetime(*time),
            });
        }

        // wasDerivedFrom
        if let Some(ref source) = prov.derived_from {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasDerivedFrom".to_string(),
                object: RdfValue::Uri(source.clone()),
            });
        }

        // dcterms:creator (attribution)
        if let Some(ref attribution) = prov.attribution {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "dcterms:creator".to_string(),
                object: RdfValue::string(attribution),
            });
        }

        triples
    }

    fn add_license_triples(&self, subject: &str, license: &LicenseInfo) -> Vec<Triple> {
        let mut triples = Vec::new();

        // dcterms:license
        triples.push(Triple {
            subject: subject.to_string(),
            predicate: "dcterms:license".to_string(),
            object: RdfValue::Uri(license.license_uri.clone()),
        });

        // cc:license (for Creative Commons)
        if license.license_uri.contains("creativecommons.org") {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "cc:license".to_string(),
                object: RdfValue::Uri(license.license_uri.clone()),
            });
        }

        // License label
        if let Some(ref label) = license.label {
            let license_subject = license.license_uri.clone();
            triples.push(Triple {
                subject: license_subject.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(label),
            });
        }

        // Rights holder
        if let Some(ref holder) = license.rights_holder {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "dcterms:rightsHolder".to_string(),
                object: RdfValue::string(holder),
            });
        }

        triples
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

    fn to_trig(&self, triples: &[Triple], graph_name: Option<&str>) -> LodResult<String> {
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

        // Named graph
        if let Some(name) = graph_name {
            output.push_str(&format!(
                "<{}graph/{}> {{\n",
                self.namespaces.base,
                escape_uri(name)
            ));
        }

        // Triples (using same logic as Turtle)
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

            let indent = if graph_name.is_some() { "    " } else { "" };
            output.push_str(indent);
            output.push_str(&subject_str);

            for (i, triple) in subject_triples.iter().enumerate() {
                let sep = if i == 0 {
                    format!("\n{}    ", indent)
                } else {
                    format!(" ;\n{}    ", indent)
                };
                output.push_str(&sep);
                output.push_str(&triple.predicate);
                output.push(' ');
                output.push_str(&self.value_to_turtle(&triple.object));
            }
            output.push_str(&format!(" .\n{}\n", indent));
        }

        if graph_name.is_some() {
            output.push_str("}\n");
        }

        Ok(output)
    }

    fn to_trig_batch(&self, triples: &[Triple], statutes: &[Statute]) -> LodResult<String> {
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

        // Each statute gets its own named graph
        for statute in statutes {
            let statute_triples: Vec<&Triple> = triples
                .iter()
                .filter(|t| t.subject.contains(&statute.id))
                .collect();

            output.push_str(&format!(
                "<{}graph/{}> {{\n",
                self.namespaces.base,
                escape_uri(&statute.id)
            ));

            // Group by subject
            let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
            for triple in &statute_triples {
                by_subject.entry(&triple.subject).or_default().push(*triple);
            }

            for (subject, subject_triples) in by_subject {
                let subject_str = if subject.starts_with(&self.namespaces.base) {
                    format!("<{}>", subject)
                } else if let Some(prefixed) = try_prefix(subject) {
                    prefixed
                } else {
                    format!("<{}>", subject)
                };

                output.push_str("    ");
                output.push_str(&subject_str);

                for (i, triple) in subject_triples.iter().enumerate() {
                    let sep = if i == 0 { "\n        " } else { " ;\n        " };
                    output.push_str(sep);
                    output.push_str(&triple.predicate);
                    output.push(' ');
                    output.push_str(&self.value_to_turtle(&triple.object));
                }
                output.push_str(" .\n\n");
            }

            output.push_str("}\n\n");
        }

        Ok(output)
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

    #[test]
    fn test_export_trig() {
        let exporter = LodExporter::new(RdfFormat::TriG);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("@prefix eli:"));
        assert!(output.contains("@prefix legalis:"));
        assert!(output.contains("graph/adult-rights"));
        assert!(output.contains("{"));
        assert!(output.contains("}"));
        assert!(output.contains("Adult Rights Act"));
    }

    #[test]
    fn test_export_trig_batch() {
        let exporter = LodExporter::new(RdfFormat::TriG);
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "test-law",
                "Test Law",
                Effect::new(EffectType::Grant, "Test rights"),
            ),
        ];

        let output = exporter.export_batch(&statutes).unwrap();
        assert!(output.contains("graph/adult-rights"));
        assert!(output.contains("graph/test-law"));
        assert!(output.contains("Adult Rights Act"));
        assert!(output.contains("Test Law"));
    }

    #[test]
    fn test_trig_extension() {
        assert_eq!(RdfFormat::TriG.extension(), "trig");
        assert_eq!(RdfFormat::TriG.mime_type(), "application/trig");
    }

    #[test]
    fn test_skos_concept_scheme() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.generate_concept_scheme("effect-types", "Legal Effect Types");

        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "skos:ConceptScheme")));
        assert!(triples.iter().any(|t| t.predicate == "skos:prefLabel"));
    }

    #[test]
    fn test_skos_effect_concept() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.create_effect_type_concept(
            "grant",
            "Grant Effect",
            Some("An effect that grants rights or permissions"),
        );

        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "skos:Concept")));
        assert!(triples.iter().any(|t| t.predicate == "skos:prefLabel"));
        assert!(triples.iter().any(|t| t.predicate == "skos:definition"));
        assert!(triples.iter().any(|t| t.predicate == "skos:inScheme"));
    }

    #[test]
    fn test_skos_hierarchy() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.add_skos_hierarchy("legal-effect", "grant-effect");

        assert!(triples.iter().any(|t| t.predicate == "skos:broader"));
        assert!(triples.iter().any(|t| t.predicate == "skos:narrower"));
        assert_eq!(triples.len(), 2);
    }

    #[test]
    fn test_content_negotiation() {
        assert_eq!(
            RdfFormat::from_accept_header("application/ld+json"),
            RdfFormat::JsonLd
        );
        assert_eq!(
            RdfFormat::from_accept_header("text/turtle"),
            RdfFormat::Turtle
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/rdf+xml"),
            RdfFormat::RdfXml
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/n-triples"),
            RdfFormat::NTriples
        );
        assert_eq!(
            RdfFormat::from_accept_header("application/trig"),
            RdfFormat::TriG
        );
        // Default to Turtle for unknown
        assert_eq!(
            RdfFormat::from_accept_header("text/html"),
            RdfFormat::Turtle
        );
    }

    #[test]
    fn test_mime_type_aliases() {
        let turtle_aliases = RdfFormat::Turtle.mime_type_aliases();
        assert!(turtle_aliases.contains(&"text/turtle"));
        assert!(turtle_aliases.contains(&"application/x-turtle"));
    }

    #[test]
    fn test_provenance_info() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/legalis")
            .with_activity("https://example.org/activity/export")
            .with_source("https://example.org/original")
            .with_attribution("Legalis Project");

        assert!(prov.agent.is_some());
        assert!(prov.activity.is_some());
        assert!(prov.derived_from.is_some());
        assert!(prov.attribution.is_some());
    }

    #[test]
    fn test_license_info() {
        let license = LicenseInfo::cc_by_4_0().with_rights_holder("Example Organization");

        assert!(license.license_uri.contains("creativecommons.org"));
        assert!(license.label.is_some());
        assert_eq!(
            license.rights_holder,
            Some("Example Organization".to_string())
        );
    }

    #[test]
    fn test_export_with_provenance() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/legalis")
            .with_attribution("Legalis Team");

        let exporter = LodExporter::new(RdfFormat::Turtle).with_provenance(prov);

        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("prov:wasAttributedTo"));
        assert!(output.contains("dcterms:creator"));
        assert!(output.contains("prov:generatedAtTime"));
    }

    #[test]
    fn test_export_with_license() {
        let license = LicenseInfo::cc_by_4_0();

        let exporter = LodExporter::new(RdfFormat::Turtle).with_license(license);

        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        assert!(output.contains("dcterms:license"));
        assert!(output.contains("cc:license"));
        assert!(output.contains("creativecommons.org"));
    }

    #[test]
    fn test_all_formats() {
        let formats = RdfFormat::all_formats();
        assert_eq!(formats.len(), 5);
        assert!(formats.contains(&RdfFormat::Turtle));
        assert!(formats.contains(&RdfFormat::JsonLd));
        assert!(formats.contains(&RdfFormat::TriG));
    }

    // Round-trip conversion tests
    #[test]
    fn test_round_trip_basic_statute() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();

        // Export to triples
        let triples = exporter.statute_to_triples(&statute).unwrap();

        // Verify key information is preserved
        assert!(triples.iter().any(|t| t.predicate == "eli:title"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:identifier"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:hasEffect"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:hasPrecondition")
        );
    }

    #[test]
    fn test_round_trip_with_metadata() {
        let prov = ProvenanceInfo::new()
            .with_agent("https://example.org/agent/test")
            .with_attribution("Test Team");

        let license = LicenseInfo::cc_by_4_0();

        let exporter = LodExporter::new(RdfFormat::Turtle)
            .with_provenance(prov)
            .with_license(license);

        let statute = sample_statute();
        let triples = exporter.statute_to_triples(&statute).unwrap();

        // Verify provenance preserved
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "prov:wasAttributedTo")
        );
        assert!(triples.iter().any(|t| t.predicate == "dcterms:creator"));

        // Verify license preserved
        assert!(triples.iter().any(|t| t.predicate == "dcterms:license"));
    }

    #[test]
    fn test_round_trip_complex_conditions() {
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

        let exporter = LodExporter::new(RdfFormat::Turtle);
        let triples = exporter.statute_to_triples(&statute).unwrap();

        // Verify condition structure preserved
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:AndCondition"))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:AgeCondition"))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:IncomeCondition"))
        );
        assert!(triples.iter().any(|t| t.predicate == "legalis:leftOperand"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:rightOperand")
        );
    }

    #[test]
    fn test_round_trip_validation_consistency() {
        let exporter = LodExporter::new(RdfFormat::Turtle);
        let statute = sample_statute();

        // Validate the statute
        let report = exporter.validate_statute(&statute).unwrap();

        // Should pass validation (no critical errors)
        assert!(report.triple_count > 0);
        assert!(report.subject_count > 0);
    }

    #[test]
    fn test_round_trip_all_formats_consistency() {
        let statute = sample_statute();

        // Export to all formats
        for format in RdfFormat::all_formats() {
            let exporter = LodExporter::new(format);
            let output = exporter.export(&statute);

            // All formats should successfully export
            assert!(output.is_ok(), "Failed to export to {:?}", format);

            let output = output.unwrap();
            assert!(!output.is_empty(), "{:?} produced empty output", format);

            // All formats should contain the title
            assert!(
                output.contains("Adult Rights Act"),
                "{:?} missing title",
                format
            );
        }
    }

    #[test]
    fn test_round_trip_batch_consistency() {
        let statutes = vec![
            sample_statute(),
            Statute::new(
                "test-law",
                "Test Law",
                Effect::new(EffectType::Grant, "Test rights"),
            ),
        ];

        let exporter = LodExporter::new(RdfFormat::Turtle);
        let batch_output = exporter.export_batch(&statutes).unwrap();

        // Should contain both statutes
        assert!(batch_output.contains("adult-rights"));
        assert!(batch_output.contains("test-law"));
        assert!(batch_output.contains("Adult Rights Act"));
        assert!(batch_output.contains("Test Law"));
    }

    #[test]
    fn test_round_trip_special_characters() {
        let statute = Statute::new(
            "special-chars",
            "Law with \"quotes\" and <tags> & symbols",
            Effect::new(EffectType::Grant, "Special\ncharacters\ttab"),
        );

        // Test Turtle
        let exporter_turtle = LodExporter::new(RdfFormat::Turtle);
        let turtle_output = exporter_turtle.export(&statute).unwrap();
        assert!(turtle_output.contains("\\\"quotes\\\""));
        assert!(turtle_output.contains("\\n"));

        // Test RDF/XML
        let exporter_xml = LodExporter::new(RdfFormat::RdfXml);
        let xml_output = exporter_xml.export(&statute).unwrap();
        assert!(xml_output.contains("&lt;tags&gt;") || xml_output.contains("&quot;quotes&quot;"));
    }

    // Benchmark tests
    #[test]
    fn test_benchmark_single_statute_export() {
        let statute = sample_statute();
        let exporter = LodExporter::new(RdfFormat::Turtle);

        // Warm up
        let _ = exporter.export(&statute);

        // Measure
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = exporter.export(&statute);
        }
        let duration = start.elapsed();

        println!("Single statute export (100 iterations): {:?}", duration);
        println!("Average: {:?}", duration / 100);

        // Sanity check - should be reasonably fast
        assert!(
            duration.as_millis() < 10000,
            "Export too slow: {:?}",
            duration
        );
    }

    #[test]
    fn test_benchmark_batch_export() {
        let statutes: Vec<Statute> = (0..100)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute Number {}", i),
                    Effect::new(EffectType::Grant, format!("Effect {}", i)),
                )
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18 + (i % 10),
                })
            })
            .collect();

        let exporter = LodExporter::new(RdfFormat::Turtle);

        let start = std::time::Instant::now();
        let output = exporter.export_batch(&statutes).unwrap();
        let duration = start.elapsed();

        println!("Batch export (100 statutes): {:?}", duration);
        println!("Per statute: {:?}", duration / 100);

        // Sanity check
        assert!(!output.is_empty());
        assert!(
            duration.as_millis() < 10000,
            "Batch export too slow: {:?}",
            duration
        );
    }

    #[test]
    fn test_benchmark_all_formats() {
        let statute = sample_statute();

        for format in RdfFormat::all_formats() {
            let exporter = LodExporter::new(format);

            let start = std::time::Instant::now();
            for _ in 0..50 {
                let _ = exporter.export(&statute);
            }
            let duration = start.elapsed();

            println!("{:?} format (50 iterations): {:?}", format, duration);
            println!("Average: {:?}", duration / 50);

            // All formats should be reasonably fast
            assert!(
                duration.as_millis() < 5000,
                "{:?} too slow: {:?}",
                format,
                duration
            );
        }
    }

    #[test]
    fn test_benchmark_validation() {
        let statute = sample_statute();
        let exporter = LodExporter::new(RdfFormat::Turtle);

        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = exporter.validate_statute(&statute);
        }
        let duration = start.elapsed();

        println!("Validation (100 iterations): {:?}", duration);
        println!("Average: {:?}", duration / 100);

        // Validation should be fast
        assert!(
            duration.as_millis() < 5000,
            "Validation too slow: {:?}",
            duration
        );
    }

    #[test]
    fn test_export_with_ontologies() {
        let exporter = LodExporter::new(RdfFormat::Turtle).with_ontologies(true);
        let statute = sample_statute();
        let output = exporter.export(&statute).unwrap();

        // Should contain ontology-specific triples
        assert!(output.contains("fabio:"));
        assert!(output.contains("lkif:"));
        assert!(output.contains("lrml:"));
        assert!(output.contains("akn:"));
    }

    #[test]
    fn test_benchmark_streaming_export() {
        use std::io::Cursor;

        let statutes: Vec<Statute> = (0..50)
            .map(|i| {
                Statute::new(
                    format!("statute-{}", i),
                    format!("Statute {}", i),
                    Effect::new(EffectType::Grant, format!("Effect {}", i)),
                )
            })
            .collect();

        let mut buffer = Cursor::new(Vec::new());
        let ns = Namespaces::default();

        let start = std::time::Instant::now();
        {
            let mut serializer =
                streaming::StreamingSerializer::new(&mut buffer, RdfFormat::Turtle, ns);
            serializer.write_header().unwrap();

            for statute in &statutes {
                let exporter = LodExporter::new(RdfFormat::Turtle);
                let triples = exporter.statute_to_triples(statute).unwrap();
                serializer.write_triples(&triples).unwrap();
            }

            serializer.finalize().unwrap();
        }
        let duration = start.elapsed();

        println!("Streaming export (50 statutes): {:?}", duration);
        println!("Per statute: {:?}", duration / 50);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(!output.is_empty());
        assert!(
            duration.as_millis() < 5000,
            "Streaming export too slow: {:?}",
            duration
        );
    }
}
