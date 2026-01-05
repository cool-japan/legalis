//! VOID (Vocabulary of Interlinked Datasets) descriptions.
//!
//! This module provides utilities to generate VOID dataset descriptions
//! for the exported legal statute data, following the VOID vocabulary.

use crate::{Namespaces, RdfValue, Triple};
use chrono::Utc;

/// VOID dataset descriptor.
#[derive(Debug, Clone)]
pub struct VoidDataset {
    /// Dataset URI
    pub uri: String,
    /// Dataset title
    pub title: String,
    /// Dataset description
    pub description: String,
    /// Publisher name
    pub publisher: Option<String>,
    /// License URL
    pub license: Option<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// SPARQL endpoint URL
    pub sparql_endpoint: Option<String>,
    /// Example resources
    pub example_resources: Vec<String>,
    /// Vocabulary URIs used in the dataset
    pub vocabularies: Vec<String>,
    /// Number of triples (if known)
    pub triples: Option<usize>,
    /// Number of entities (if known)
    pub entities: Option<usize>,
    /// Number of classes (if known)
    pub classes: Option<usize>,
    /// Number of properties (if known)
    pub properties: Option<usize>,
}

impl VoidDataset {
    /// Creates a new VOID dataset descriptor.
    pub fn new(uri: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            title: title.into(),
            description: String::new(),
            publisher: None,
            license: None,
            homepage: None,
            sparql_endpoint: None,
            example_resources: Vec::new(),
            vocabularies: Vec::new(),
            triples: None,
            entities: None,
            classes: None,
            properties: None,
        }
    }

    /// Sets the dataset description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the publisher name.
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Sets the license URL.
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Sets the homepage URL.
    pub fn with_homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    /// Sets the SPARQL endpoint URL.
    pub fn with_sparql_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.sparql_endpoint = Some(endpoint.into());
        self
    }

    /// Adds an example resource.
    pub fn add_example_resource(mut self, resource: impl Into<String>) -> Self {
        self.example_resources.push(resource.into());
        self
    }

    /// Adds a vocabulary URI.
    pub fn add_vocabulary(mut self, vocab: impl Into<String>) -> Self {
        self.vocabularies.push(vocab.into());
        self
    }

    /// Sets statistics for the dataset.
    pub fn with_statistics(
        mut self,
        triples: Option<usize>,
        entities: Option<usize>,
        classes: Option<usize>,
        properties: Option<usize>,
    ) -> Self {
        self.triples = triples;
        self.entities = entities;
        self.classes = classes;
        self.properties = properties;
        self
    }

    /// Converts this VOID dataset to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let subject = &self.uri;

        // Type
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("void:Dataset".to_string()),
        });

        // Title
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&self.title),
        });

        // Description
        if !self.description.is_empty() {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "dcterms:description".to_string(),
                object: RdfValue::string(&self.description),
            });
        }

        // Publisher
        if let Some(ref publisher) = self.publisher {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "dcterms:publisher".to_string(),
                object: RdfValue::string(publisher),
            });
        }

        // License
        if let Some(ref license) = self.license {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "dcterms:license".to_string(),
                object: RdfValue::Uri(license.clone()),
            });
        }

        // Homepage
        if let Some(ref homepage) = self.homepage {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:homepage".to_string(),
                object: RdfValue::Uri(homepage.clone()),
            });
        }

        // SPARQL endpoint
        if let Some(ref endpoint) = self.sparql_endpoint {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:sparqlEndpoint".to_string(),
                object: RdfValue::Uri(endpoint.clone()),
            });
        }

        // Example resources
        for resource in &self.example_resources {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:exampleResource".to_string(),
                object: RdfValue::Uri(resource.clone()),
            });
        }

        // Vocabularies
        for vocab in &self.vocabularies {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:vocabulary".to_string(),
                object: RdfValue::Uri(vocab.clone()),
            });
        }

        // Statistics
        if let Some(triple_count) = self.triples {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:triples".to_string(),
                object: RdfValue::integer(triple_count as i64),
            });
        }

        if let Some(entity_count) = self.entities {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:entities".to_string(),
                object: RdfValue::integer(entity_count as i64),
            });
        }

        if let Some(class_count) = self.classes {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:classes".to_string(),
                object: RdfValue::integer(class_count as i64),
            });
        }

        if let Some(property_count) = self.properties {
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "void:properties".to_string(),
                object: RdfValue::integer(property_count as i64),
            });
        }

        // Creation date
        let now = Utc::now();
        triples.push(Triple {
            subject: subject.clone(),
            predicate: "dcterms:created".to_string(),
            object: RdfValue::date_from_datetime(now),
        });

        triples
    }

    /// Exports the VOID description as Turtle.
    pub fn to_turtle(&self) -> String {
        let triples = self.to_triples();
        let mut output = String::new();

        // Prefixes
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');

        // Dataset triples
        output.push_str(&format!("<{}>\n", self.uri));
        for (i, triple) in triples.iter().enumerate() {
            if i > 0 {
                output.push_str(" ;\n");
            }
            output.push_str(&format!("    {} ", triple.predicate));
            output.push_str(&value_to_turtle(&triple.object));
        }
        output.push_str(" .\n");

        output
    }
}

/// Helper function to convert RdfValue to Turtle representation.
fn value_to_turtle(value: &RdfValue) -> String {
    match value {
        RdfValue::Uri(uri) => format!("<{}>", uri),
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

/// Escapes a string for Turtle.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Creates a default VOID dataset for Legalis.
pub fn create_legalis_void_dataset(base_uri: &str) -> VoidDataset {
    VoidDataset::new(
        format!("{}dataset/legalis", base_uri),
        "Legalis Legal Statute Dataset",
    )
    .with_description(
        "A dataset of legal statutes in RDF format, using the ELI and Legalis ontologies.",
    )
    .add_vocabulary("http://data.europa.eu/eli/ontology#")
    .add_vocabulary("https://legalis.dev/ontology#")
    .add_vocabulary("http://purl.org/dc/terms/")
    .add_vocabulary("http://www.w3.org/2004/02/skos/core#")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_void_dataset() {
        let dataset = VoidDataset::new("https://example.org/dataset", "Test Dataset")
            .with_description("A test dataset")
            .with_publisher("Example Publisher");

        let triples = dataset.to_triples();
        assert!(!triples.is_empty());

        // Check that it has the basic properties
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "void:Dataset")));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:title"));
    }

    #[test]
    fn test_void_with_statistics() {
        let dataset = VoidDataset::new("https://example.org/dataset", "Test").with_statistics(
            Some(1000),
            Some(50),
            Some(10),
            Some(20),
        );

        let triples = dataset.to_triples();
        assert!(triples.iter().any(|t| t.predicate == "void:triples"));
        assert!(triples.iter().any(|t| t.predicate == "void:entities"));
        assert!(triples.iter().any(|t| t.predicate == "void:classes"));
        assert!(triples.iter().any(|t| t.predicate == "void:properties"));
    }

    #[test]
    fn test_void_to_turtle() {
        let dataset = VoidDataset::new("https://example.org/dataset", "Test Dataset");
        let turtle = dataset.to_turtle();

        assert!(turtle.contains("@prefix void:"));
        assert!(turtle.contains("void:Dataset"));
        assert!(turtle.contains("Test Dataset"));
    }

    #[test]
    fn test_create_legalis_void() {
        let dataset = create_legalis_void_dataset("https://example.org/");
        assert_eq!(dataset.uri, "https://example.org/dataset/legalis");
        assert!(!dataset.vocabularies.is_empty());
    }
}
