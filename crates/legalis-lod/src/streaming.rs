//! Streaming RDF serialization for large datasets.
//!
//! This module provides streaming serializers that write RDF data incrementally,
//! allowing for efficient processing of large statute collections without loading
//! everything into memory at once.

use crate::{LodError, LodResult, Namespaces, RdfFormat, RdfValue, Triple};
use std::collections::HashMap;
use std::io::Write;

/// A streaming RDF serializer.
pub struct StreamingSerializer<W: Write> {
    writer: W,
    format: RdfFormat,
    namespaces: Namespaces,
    header_written: bool,
    subject_count: usize,
}

impl<W: Write> StreamingSerializer<W> {
    /// Creates a new streaming serializer.
    pub fn new(writer: W, format: RdfFormat, namespaces: Namespaces) -> Self {
        Self {
            writer,
            format,
            namespaces,
            header_written: false,
            subject_count: 0,
        }
    }

    /// Writes the RDF header (prefixes, etc.).
    pub fn write_header(&mut self) -> LodResult<()> {
        if self.header_written {
            return Ok(());
        }

        match self.format {
            RdfFormat::Turtle | RdfFormat::TriG => {
                self.write_turtle_header()?;
            }
            RdfFormat::RdfXml => {
                self.write_rdf_xml_header()?;
            }
            RdfFormat::JsonLd => {
                writeln!(self.writer, "{{")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
                writeln!(self.writer, "  \"@context\": {{")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
                for (i, (prefix, uri)) in Namespaces::standard_prefixes().iter().enumerate() {
                    let comma = if i < Namespaces::standard_prefixes().len() - 1 {
                        ","
                    } else {
                        ""
                    };
                    writeln!(self.writer, "    \"{}\": \"{}\"{}", prefix, uri, comma)
                        .map_err(|e| LodError::SerializationError(e.to_string()))?;
                }
                writeln!(self.writer, "  }},")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
                writeln!(self.writer, "  \"@graph\": [")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }
            RdfFormat::NTriples => {
                // N-Triples has no header
            }
        }

        self.header_written = true;
        Ok(())
    }

    fn write_turtle_header(&mut self) -> LodResult<()> {
        for (prefix, uri) in Namespaces::standard_prefixes() {
            writeln!(self.writer, "@prefix {}: <{}> .", prefix, uri)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }
        writeln!(self.writer, "@base <{}> .", self.namespaces.base)
            .map_err(|e| LodError::SerializationError(e.to_string()))?;
        for (prefix, uri) in &self.namespaces.custom {
            writeln!(self.writer, "@prefix {}: <{}> .", prefix, uri)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }
        writeln!(self.writer).map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(())
    }

    fn write_rdf_xml_header(&mut self) -> LodResult<()> {
        writeln!(self.writer, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
            .map_err(|e| LodError::SerializationError(e.to_string()))?;
        write!(self.writer, "<rdf:RDF").map_err(|e| LodError::SerializationError(e.to_string()))?;
        for (prefix, uri) in Namespaces::standard_prefixes() {
            write!(self.writer, "\n    xmlns:{}=\"{}\"", prefix, uri)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }
        writeln!(self.writer, "\n    xml:base=\"{}\">", self.namespaces.base)
            .map_err(|e| LodError::SerializationError(e.to_string()))?;
        writeln!(self.writer).map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(())
    }

    /// Writes a batch of triples.
    pub fn write_triples(&mut self, triples: &[Triple]) -> LodResult<()> {
        if !self.header_written {
            self.write_header()?;
        }

        match self.format {
            RdfFormat::Turtle => self.write_triples_turtle(triples)?,
            RdfFormat::NTriples => self.write_triples_ntriples(triples)?,
            RdfFormat::RdfXml => self.write_triples_rdf_xml(triples)?,
            RdfFormat::JsonLd => self.write_triples_json_ld(triples)?,
            RdfFormat::TriG => self.write_triples_turtle(triples)?, // Same as Turtle for now
        }

        Ok(())
    }

    fn write_triples_turtle(&mut self, triples: &[Triple]) -> LodResult<()> {
        // Group by subject for prettier output
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

            write!(self.writer, "{}", subject_str)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;

            for (i, triple) in subject_triples.iter().enumerate() {
                let sep = if i == 0 { "\n    " } else { " ;\n    " };
                write!(
                    self.writer,
                    "{}{} {}",
                    sep,
                    triple.predicate,
                    self.value_to_turtle(&triple.object)
                )
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }
            writeln!(self.writer, " .").map_err(|e| LodError::SerializationError(e.to_string()))?;
            writeln!(self.writer).map_err(|e| LodError::SerializationError(e.to_string()))?;
        }

        Ok(())
    }

    fn write_triples_ntriples(&mut self, triples: &[Triple]) -> LodResult<()> {
        for triple in triples {
            let subject = expand_uri(&triple.subject, &self.namespaces);
            let predicate = expand_uri(&triple.predicate, &self.namespaces);
            let object = self.value_to_ntriples(&triple.object);

            writeln!(self.writer, "<{}> <{}> {} .", subject, predicate, object)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }
        Ok(())
    }

    fn write_triples_rdf_xml(&mut self, triples: &[Triple]) -> LodResult<()> {
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
                .and_then(|t| match &t.object {
                    RdfValue::Uri(u) => Some(u.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| "rdf:Description".to_string());

            writeln!(self.writer, "  <{} rdf:about=\"{}\">", rdf_type, subject)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;

            for triple in subject_triples {
                if triple.predicate == "rdf:type" {
                    continue; // Already handled
                }
                writeln!(
                    self.writer,
                    "    {}",
                    self.triple_to_rdf_xml_element(triple)
                )
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }

            writeln!(self.writer, "  </{}>\n", rdf_type)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }

        Ok(())
    }

    fn write_triples_json_ld(&mut self, triples: &[Triple]) -> LodResult<()> {
        // Group by subject
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        for (idx, (subject, subject_triples)) in by_subject.iter().enumerate() {
            if idx > 0 || self.subject_count > 0 {
                writeln!(self.writer, "    ,")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }

            writeln!(self.writer, "    {{")
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
            writeln!(self.writer, "      \"@id\": \"{}\",", subject)
                .map_err(|e| LodError::SerializationError(e.to_string()))?;

            // Write properties
            for (i, triple) in subject_triples.iter().enumerate() {
                let value = match &triple.object {
                    RdfValue::Uri(uri) => format!("{{\"@id\": \"{}\"}}", uri),
                    RdfValue::Literal(s, None) => format!("\"{}\"", escape_json(s)),
                    RdfValue::Literal(s, Some(lang)) => {
                        format!(
                            "{{\"@value\": \"{}\", \"@language\": \"{}\"}}",
                            escape_json(s),
                            lang
                        )
                    }
                    RdfValue::TypedLiteral(s, dtype) => {
                        format!(
                            "{{\"@value\": \"{}\", \"@type\": \"{}\"}}",
                            escape_json(s),
                            dtype
                        )
                    }
                    RdfValue::BlankNode(id) => format!("{{\"@id\": \"_:{}\"}}", id),
                };

                let comma = if i < subject_triples.len() - 1 {
                    ","
                } else {
                    ""
                };
                writeln!(
                    self.writer,
                    "      \"{}\": {}{}",
                    triple.predicate, value, comma
                )
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }

            write!(self.writer, "    }}")
                .map_err(|e| LodError::SerializationError(e.to_string()))?;
        }

        self.subject_count += by_subject.len();
        Ok(())
    }

    /// Writes the RDF footer and finalizes the output.
    pub fn finalize(mut self) -> LodResult<W> {
        match self.format {
            RdfFormat::RdfXml => {
                writeln!(self.writer, "</rdf:RDF>")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }
            RdfFormat::JsonLd => {
                writeln!(self.writer).map_err(|e| LodError::SerializationError(e.to_string()))?;
                writeln!(self.writer, "  ]")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
                writeln!(self.writer, "}}")
                    .map_err(|e| LodError::SerializationError(e.to_string()))?;
            }
            _ => {
                // Turtle, N-Triples, TriG have no footer
            }
        }

        self.writer
            .flush()
            .map_err(|e| LodError::SerializationError(e.to_string()))?;
        Ok(self.writer)
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
}

/// Tries to convert a URI to prefixed form.
fn try_prefix(uri: &str) -> Option<String> {
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
fn expand_uri(uri: &str, namespaces: &Namespaces) -> String {
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

/// Escapes a string for JSON.
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfFormat;

    #[test]
    fn test_streaming_turtle() {
        let mut buffer = Vec::new();
        let ns = Namespaces::default();
        let mut serializer = StreamingSerializer::new(&mut buffer, RdfFormat::Turtle, ns);

        serializer.write_header().unwrap();

        let triples = vec![Triple {
            subject: "https://example.org/test".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        }];

        serializer.write_triples(&triples).unwrap();
        serializer.finalize().unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("@prefix"));
        assert!(output.contains("rdf:type"));
    }

    #[test]
    fn test_streaming_ntriples() {
        let mut buffer = Vec::new();
        let ns = Namespaces::default();
        let mut serializer = StreamingSerializer::new(&mut buffer, RdfFormat::NTriples, ns);

        let triples = vec![Triple {
            subject: "https://example.org/test".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        }];

        serializer.write_triples(&triples).unwrap();
        serializer.finalize().unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("<https://example.org/test>"));
        assert!(output.contains("<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>"));
    }

    #[test]
    fn test_streaming_multiple_batches() {
        let mut buffer = Vec::new();
        let ns = Namespaces::default();
        let mut serializer = StreamingSerializer::new(&mut buffer, RdfFormat::Turtle, ns);

        serializer.write_header().unwrap();

        // First batch
        let triples1 = vec![Triple {
            subject: "https://example.org/test1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        }];

        serializer.write_triples(&triples1).unwrap();

        // Second batch
        let triples2 = vec![Triple {
            subject: "https://example.org/test2".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        }];

        serializer.write_triples(&triples2).unwrap();
        serializer.finalize().unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("test1"));
        assert!(output.contains("test2"));
    }
}
