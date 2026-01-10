//! RDFa (RDF in Attributes) output for HTML embedding.
//!
//! This module provides utilities to generate RDFa markup that can be embedded
//! in HTML documents, allowing semantic web data to be included in web pages.

use crate::{LodResult, Namespaces, RdfValue, Triple};
use std::collections::HashMap;

/// RDFa version to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RdfaVersion {
    /// RDFa 1.0
    V1_0,
    /// RDFa 1.1 (recommended)
    #[default]
    V1_1,
}

/// RDFa exporter.
#[derive(Debug)]
pub struct RdfaExporter {
    version: RdfaVersion,
    namespaces: Namespaces,
    use_prefix_attr: bool,
}

impl Default for RdfaExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl RdfaExporter {
    /// Creates a new RDFa exporter with default settings.
    pub fn new() -> Self {
        Self {
            version: RdfaVersion::default(),
            namespaces: Namespaces::default(),
            use_prefix_attr: true,
        }
    }

    /// Creates a new RDFa exporter with custom namespaces.
    pub fn with_namespaces(namespaces: Namespaces) -> Self {
        Self {
            version: RdfaVersion::default(),
            namespaces,
            use_prefix_attr: true,
        }
    }

    /// Sets the RDFa version.
    pub fn with_version(mut self, version: RdfaVersion) -> Self {
        self.version = version;
        self
    }

    /// Sets whether to use prefix attribute.
    pub fn with_prefix_attr(mut self, use_prefix: bool) -> Self {
        self.use_prefix_attr = use_prefix;
        self
    }

    /// Exports triples as RDFa embedded in HTML.
    pub fn export_as_html(&self, triples: &[Triple], title: &str) -> LodResult<String> {
        let mut html = String::new();

        // HTML document header
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html");

        if self.use_prefix_attr {
            html.push_str(" prefix=\"");
            let prefixes: Vec<String> = Namespaces::standard_prefixes()
                .iter()
                .map(|(prefix, uri)| format!("{}: {}", prefix, uri))
                .collect();
            html.push_str(&prefixes.join(" "));
            html.push('"');
        }

        html.push_str(">\n<head>\n");
        html.push_str(&format!("  <title>{}</title>\n", escape_html(title)));
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("    .resource { border: 1px solid #ccc; margin: 10px 0; padding: 15px; }\n");
        html.push_str("    .property { margin: 5px 0; }\n");
        html.push_str("    .property-name { font-weight: bold; color: #0066cc; }\n");
        html.push_str("    .property-value { margin-left: 20px; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("  <h1>{}</h1>\n", escape_html(title)));

        // Group triples by subject
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        // Generate RDFa for each subject
        for (subject, subject_triples) in by_subject {
            html.push_str(&self.export_resource(subject, &subject_triples)?);
        }

        html.push_str("</body>\n</html>\n");
        Ok(html)
    }

    /// Exports a single resource as an RDFa div.
    fn export_resource(&self, subject: &str, triples: &[&Triple]) -> LodResult<String> {
        let mut html = String::new();

        // Find the type for better display
        let rdf_type = triples
            .iter()
            .find(|t| t.predicate == "rdf:type")
            .and_then(|t| match &t.object {
                RdfValue::Uri(uri) => Some(uri.as_str()),
                _ => None,
            });

        // Resource container
        html.push_str("  <div class=\"resource\" about=\"");
        html.push_str(subject);
        html.push('"');

        if let Some(type_uri) = rdf_type {
            html.push_str(" typeof=\"");
            html.push_str(&self.curie_or_uri(type_uri));
            html.push('"');
        }

        html.push_str(">\n");

        // Display title/label if available
        let title = triples
            .iter()
            .find(|t| {
                t.predicate == "eli:title"
                    || t.predicate == "dcterms:title"
                    || t.predicate == "rdfs:label"
            })
            .and_then(|t| match &t.object {
                RdfValue::Literal(s, _) => Some(s.as_str()),
                _ => None,
            });

        if let Some(title_text) = title {
            html.push_str("    <h2 property=\"");
            html.push_str(&self.curie_or_uri("dcterms:title"));
            html.push_str("\">");
            html.push_str(&escape_html(title_text));
            html.push_str("</h2>\n");
        }

        // Properties
        for triple in triples {
            // Skip type (already handled) and title (already displayed)
            if triple.predicate == "rdf:type"
                || triple.predicate == "eli:title"
                || triple.predicate == "dcterms:title"
                || triple.predicate == "rdfs:label"
            {
                continue;
            }

            html.push_str("    <div class=\"property\">\n");
            html.push_str("      <span class=\"property-name\">");
            html.push_str(&self.curie_or_uri(&triple.predicate));
            html.push_str(":</span>\n");
            html.push_str("      <span class=\"property-value\"");

            match &triple.object {
                RdfValue::Uri(uri) => {
                    html.push_str(" property=\"");
                    html.push_str(&self.curie_or_uri(&triple.predicate));
                    html.push_str("\" resource=\"");
                    html.push_str(uri);
                    html.push_str("\">");
                    html.push_str(&self.curie_or_uri(uri));
                }
                RdfValue::Literal(s, None) => {
                    html.push_str(" property=\"");
                    html.push_str(&self.curie_or_uri(&triple.predicate));
                    html.push_str("\">");
                    html.push_str(&escape_html(s));
                }
                RdfValue::Literal(s, Some(lang)) => {
                    html.push_str(" property=\"");
                    html.push_str(&self.curie_or_uri(&triple.predicate));
                    html.push_str("\" lang=\"");
                    html.push_str(lang);
                    html.push_str("\">");
                    html.push_str(&escape_html(s));
                }
                RdfValue::TypedLiteral(s, dtype) => {
                    html.push_str(" property=\"");
                    html.push_str(&self.curie_or_uri(&triple.predicate));
                    html.push_str("\" datatype=\"");
                    html.push_str(&self.curie_or_uri(dtype));
                    html.push_str("\">");
                    html.push_str(&escape_html(s));
                }
                RdfValue::BlankNode(id) => {
                    html.push_str(" property=\"");
                    html.push_str(&self.curie_or_uri(&triple.predicate));
                    html.push_str("\" resource=\"_:");
                    html.push_str(id);
                    html.push_str("\">");
                    html.push_str("_:");
                    html.push_str(id);
                }
            }

            html.push_str("</span>\n");
            html.push_str("    </div>\n");
        }

        html.push_str("  </div>\n");
        Ok(html)
    }

    /// Converts a URI to a CURIE (Compact URI) if possible, otherwise returns the full URI.
    fn curie_or_uri(&self, uri: &str) -> String {
        // Check if already a CURIE
        if uri.contains(':') && !uri.starts_with("http://") && !uri.starts_with("https://") {
            return uri.to_string();
        }

        // Try to create a CURIE
        for (prefix, ns) in Namespaces::standard_prefixes() {
            if let Some(suffix) = uri.strip_prefix(ns) {
                return format!("{}:{}", prefix, suffix);
            }
        }

        // Check custom namespaces
        for (prefix, ns) in &self.namespaces.custom {
            if let Some(suffix) = uri.strip_prefix(ns) {
                return format!("{}:{}", prefix, suffix);
            }
        }

        // Return full URI
        uri.to_string()
    }

    /// Exports only the RDFa markup (without HTML wrapper).
    pub fn export_rdfa_fragment(&self, triples: &[Triple]) -> LodResult<String> {
        let mut html = String::new();

        // Group triples by subject
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        // Generate RDFa for each subject
        for (subject, subject_triples) in by_subject {
            html.push_str(&self.export_resource(subject, &subject_triples)?);
        }

        Ok(html)
    }

    /// Generates an inline RDFa span for a single triple.
    pub fn export_inline_property(
        &self,
        predicate: &str,
        object: &RdfValue,
        display_text: Option<&str>,
    ) -> String {
        let mut html = String::new();
        html.push_str("<span property=\"");
        html.push_str(&self.curie_or_uri(predicate));
        html.push('"');

        match object {
            RdfValue::Uri(uri) => {
                html.push_str(" resource=\"");
                html.push_str(uri);
                html.push_str("\">");
                html.push_str(&escape_html(display_text.unwrap_or(uri)));
            }
            RdfValue::Literal(s, Some(lang)) => {
                html.push_str(" lang=\"");
                html.push_str(lang);
                html.push_str("\">");
                html.push_str(&escape_html(display_text.unwrap_or(s)));
            }
            RdfValue::TypedLiteral(s, dtype) => {
                html.push_str(" datatype=\"");
                html.push_str(&self.curie_or_uri(dtype));
                html.push_str("\">");
                html.push_str(&escape_html(display_text.unwrap_or(s)));
            }
            RdfValue::Literal(s, None) => {
                html.push('>');
                html.push_str(&escape_html(display_text.unwrap_or(s)));
            }
            RdfValue::BlankNode(id) => {
                html.push_str(" resource=\"_:");
                html.push_str(id);
                html.push_str("\">");
                html.push_str(&escape_html(display_text.unwrap_or(&format!("_:{}", id))));
            }
        }

        html.push_str("</span>");
        html
    }
}

/// Escapes HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Adult Rights Act"),
            },
            Triple {
                subject: "https://example.org/statute/1".to_string(),
                predicate: "dcterms:identifier".to_string(),
                object: RdfValue::string("adult-rights"),
            },
        ]
    }

    #[test]
    fn test_rdfa_export() {
        let exporter = RdfaExporter::new();
        let triples = sample_triples();
        let html = exporter.export_as_html(&triples, "Test Statute").unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("prefix="));
        assert!(html.contains("about=\"https://example.org/statute/1\""));
        assert!(html.contains("typeof=\"legalis:Statute\""));
        assert!(html.contains("Adult Rights Act"));
    }

    #[test]
    fn test_rdfa_fragment() {
        let exporter = RdfaExporter::new();
        let triples = sample_triples();
        let html = exporter.export_rdfa_fragment(&triples).unwrap();

        assert!(!html.contains("<!DOCTYPE html>"));
        assert!(html.contains("about=\"https://example.org/statute/1\""));
        assert!(html.contains("typeof=\"legalis:Statute\""));
    }

    #[test]
    fn test_inline_property() {
        let exporter = RdfaExporter::new();
        let html =
            exporter.export_inline_property("dcterms:title", &RdfValue::string("Test Title"), None);

        assert!(html.contains("<span property=\"dcterms:title\">"));
        assert!(html.contains("Test Title"));
        assert!(html.contains("</span>"));
    }

    #[test]
    fn test_inline_property_with_language() {
        let exporter = RdfaExporter::new();
        let html = exporter.export_inline_property(
            "dcterms:title",
            &RdfValue::lang_string("Test Title", "en"),
            None,
        );

        assert!(html.contains("lang=\"en\""));
        assert!(html.contains("Test Title"));
    }

    #[test]
    fn test_curie_conversion() {
        let exporter = RdfaExporter::new();
        assert_eq!(
            exporter.curie_or_uri("http://purl.org/dc/terms/title"),
            "dcterms:title"
        );
        assert_eq!(exporter.curie_or_uri("dcterms:title"), "dcterms:title");
        assert_eq!(
            exporter.curie_or_uri("https://example.org/custom"),
            "https://example.org/custom"
        );
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("A & B"), "A &amp; B");
        assert_eq!(escape_html("'quote'"), "&#39;quote&#39;");
    }
}
