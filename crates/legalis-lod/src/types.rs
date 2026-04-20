//! Auto-generated module: types for legalis-lod.

use chrono::{DateTime, NaiveDate, Utc};
use legalis_core::{ComparisonOp, Condition, EffectType, Statute};
use std::collections::HashMap;
use thiserror::Error;

/// RDF triple representation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Triple {
    pub subject: String,
    pub predicate: String,
    pub object: RdfValue,
}
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
            Self::Turtle => {
                vec!["text/turtle", "application/x-turtle", "application/turtle"]
            }
            Self::NTriples => vec!["application/n-triples", "text/plain"],
            Self::RdfXml => vec!["application/rdf+xml", "application/xml", "text/xml"],
            Self::JsonLd => vec!["application/ld+json", "application/json"],
            Self::TriG => vec!["application/trig", "application/x-trig"],
        }
    }
    /// Selects the best format based on HTTP Accept header.
    /// Returns the default format (Turtle) if no match is found.
    pub fn from_accept_header(accept: &str) -> Self {
        let accept_lower = accept.to_lowercase();
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
/// Standard namespace prefixes for legal ontologies.
#[derive(Debug, Clone)]
pub struct Namespaces {
    /// Base URI for generated resources
    pub base: String,
    /// Custom namespace mappings
    pub custom: HashMap<String, String>,
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
    pub(crate) fn standard_prefixes() -> Vec<(&'static str, &'static str)> {
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
            ("geo", crate::geosparql::GEOSPARQL_NS),
            ("sf", crate::geosparql::SF_NS),
            ("temporal", crate::temporal_rdf::TEMPORAL_NS),
            ("time", crate::temporal_rdf::TIME_NS),
            ("legalis", "https://legalis.dev/ontology#"),
            ("fabio", crate::ontology::fabio::NAMESPACE),
            ("lkif", crate::ontology::lkif::NAMESPACE),
            ("lrml", crate::ontology::legalruleml::NAMESPACE),
            ("akn", crate::ontology::akoma_ntoso::NAMESPACE),
        ]
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
    pub fn validate_statute(
        &self,
        statute: &Statute,
    ) -> LodResult<crate::validation::ValidationReport> {
        let triples = self.statute_to_triples(statute)?;
        let validator = crate::validation::RdfValidator::new();
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
    pub(crate) fn statute_to_triples(&self, statute: &Statute) -> LodResult<Vec<Triple>> {
        let mut triples = Vec::new();
        let subject = format!(
            "{}statute/{}",
            self.namespaces.base,
            escape_uri(&statute.id)
        );
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
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&statute.id),
        });
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
        if let Some(ref jurisdiction) = statute.jurisdiction {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "eli:jurisdiction".to_string(),
                object: RdfValue::string(jurisdiction),
            });
        }
        if statute.version > 0 {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "eli:version".to_string(),
                object: RdfValue::integer(statute.version as i64),
            });
        }
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
        if statute.discretion_logic.is_some() {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "legalis:hasDiscretion".to_string(),
                object: RdfValue::boolean(true),
            });
        }
        if let Some(ref prov) = self.provenance {
            triples.extend(self.add_provenance_triples(&subject, prov));
        }
        if let Some(ref lic) = self.license {
            triples.extend(self.add_license_triples(&subject, lic));
        }
        if self.include_ontologies {
            triples.extend(crate::ontology::generate_all_ontology_triples(
                &subject,
                statute,
                &self.namespaces.base,
            ));
        }
        Ok(triples)
    }
    fn add_provenance_triples(&self, subject: &str, prov: &ProvenanceInfo) -> Vec<Triple> {
        let mut triples = Vec::new();
        if let Some(ref activity) = prov.activity {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasGeneratedBy".to_string(),
                object: RdfValue::Uri(activity.clone()),
            });
        }
        if let Some(ref agent) = prov.agent {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasAttributedTo".to_string(),
                object: RdfValue::Uri(agent.clone()),
            });
        }
        if let Some(ref time) = prov.generated_at {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:generatedAtTime".to_string(),
                object: RdfValue::datetime(*time),
            });
        }
        if let Some(ref source) = prov.derived_from {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "prov:wasDerivedFrom".to_string(),
                object: RdfValue::Uri(source.clone()),
            });
        }
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
        triples.push(Triple {
            subject: subject.to_string(),
            predicate: "dcterms:license".to_string(),
            object: RdfValue::Uri(license.license_uri.clone()),
        });
        if license.license_uri.contains("creativecommons.org") {
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "cc:license".to_string(),
                object: RdfValue::Uri(license.license_uri.clone()),
            });
        }
        if let Some(ref label) = license.label {
            let license_subject = license.license_uri.clone();
            triples.push(Triple {
                subject: license_subject.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(label),
            });
        }
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
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push_str(&format!("@base <{}> .\n", self.namespaces.base));
        for (prefix, uri) in &self.namespaces.custom {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');
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
            RdfValue::Literal(s, Some(lang)) => {
                format!("\"{}\"@{}", escape_string(s), lang)
            }
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
            RdfValue::Literal(s, Some(lang)) => {
                format!("\"{}\"@{}", escape_string(s), lang)
            }
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
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }
        for (subject, subject_triples) in by_subject {
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
                    continue;
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
        let mut context = serde_json::Map::new();
        for (prefix, uri) in Namespaces::standard_prefixes() {
            context.insert(prefix.to_string(), serde_json::json!(uri));
        }
        doc.insert("@context".to_string(), serde_json::Value::Object(context));
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
                continue;
            }
            let value = match &triple.object {
                RdfValue::Uri(uri) => serde_json::json!({ "@id" : uri }),
                RdfValue::Literal(s, None) => serde_json::json!(s),
                RdfValue::Literal(s, Some(lang)) => {
                    serde_json::json!({ "@value" : s, "@language" : lang })
                }
                RdfValue::TypedLiteral(s, dtype) => {
                    serde_json::json!({ "@value" : s, "@type" : dtype })
                }
                RdfValue::BlankNode(id) => {
                    serde_json::json!({ "@id" : format!("_:{}", id) })
                }
            };
            doc.insert(triple.predicate.clone(), value);
        }
    }
    fn to_trig(&self, triples: &[Triple], graph_name: Option<&str>) -> LodResult<String> {
        let mut output = String::new();
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push_str(&format!("@base <{}> .\n", self.namespaces.base));
        for (prefix, uri) in &self.namespaces.custom {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');
        if let Some(name) = graph_name {
            output.push_str(&format!(
                "<{}graph/{}> {{\n",
                self.namespaces.base,
                escape_uri(name)
            ));
        }
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
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push_str(&format!("@base <{}> .\n", self.namespaces.base));
        for (prefix, uri) in &self.namespaces.custom {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');
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

/// Escapes a string for URI usage.
pub(super) fn escape_uri(s: &str) -> String {
    s.replace(' ', "_").replace('/', "-").replace('&', "-and-")
}

/// Escapes a string for Turtle/N-Triples.
pub(super) fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escapes a string for XML.
pub(super) fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Tries to convert a URI to prefixed form.
pub(super) fn try_prefix(uri: &str) -> Option<String> {
    for (prefix, ns) in Namespaces::standard_prefixes() {
        if let Some(suffix) = uri.strip_prefix(ns) {
            return Some(format!("{}:{}", prefix, suffix));
        }
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return Some(uri.to_string());
        }
    }
    None
}

/// Expands a prefixed URI to full form.
pub(super) fn expand_uri(uri: &str, namespaces: &Namespaces) -> String {
    for (prefix, ns) in Namespaces::standard_prefixes() {
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return format!("{}{}", ns, &uri[prefix_colon.len()..]);
        }
    }
    for (prefix, ns) in &namespaces.custom {
        let prefix_colon = format!("{}:", prefix);
        if uri.starts_with(&prefix_colon) {
            return format!("{}{}", ns, &uri[prefix_colon.len()..]);
        }
    }
    uri.to_string()
}

/// Converts an effect type to URI fragment.
pub(super) fn effect_type_to_uri(effect_type: &EffectType) -> &'static str {
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
pub(super) fn operator_to_uri(op: ComparisonOp) -> &'static str {
    match op {
        ComparisonOp::Equal => "Equal",
        ComparisonOp::NotEqual => "NotEqual",
        ComparisonOp::GreaterThan => "GreaterThan",
        ComparisonOp::GreaterOrEqual => "GreaterOrEqual",
        ComparisonOp::LessThan => "LessThan",
        ComparisonOp::LessOrEqual => "LessOrEqual",
    }
}

/// Converts a condition to RDF triples (standalone function to avoid clippy recursion warning).
pub(super) fn condition_to_triples_impl(uri: &str, condition: &Condition) -> Vec<Triple> {
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
            triples.push(Triple {
                subject: uri.to_string(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(format!("{:?}", condition)),
            });
        }
    }
    triples
}
