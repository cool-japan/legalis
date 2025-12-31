//! RDF-star (RDF*) support for making statements about statements.
//!
//! RDF-star extends RDF by allowing triples to be used as subjects or objects of other triples,
//! enabling metadata about triples without the complexity of traditional reification.
//!
//! Example: "John says that Paris is the capital of France"
//! ```turtle
//! << :Paris :capitalOf :France >> :saidBy :John .
//! ```

use crate::{LodResult, RdfValue, Triple};
use std::fmt;

/// A quoted triple (also called embedded triple) that can be used as subject or object.
/// This is the core feature of RDF-star.
#[derive(Debug, Clone, PartialEq)]
pub struct QuotedTriple {
    pub subject: String,
    pub predicate: String,
    pub object: RdfValue,
}

impl QuotedTriple {
    /// Creates a new quoted triple.
    pub fn new(subject: String, predicate: String, object: RdfValue) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    /// Converts a regular triple to a quoted triple.
    pub fn from_triple(triple: &Triple) -> Self {
        Self {
            subject: triple.subject.clone(),
            predicate: triple.predicate.clone(),
            object: triple.object.clone(),
        }
    }

    /// Converts this quoted triple to a regular triple.
    pub fn to_triple(&self) -> Triple {
        Triple {
            subject: self.subject.clone(),
            predicate: self.predicate.clone(),
            object: self.object.clone(),
        }
    }
}

impl fmt::Display for QuotedTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<< {} {} {:?} >>",
            self.subject, self.predicate, self.object
        )
    }
}

/// RDF-star triple that can have quoted triples as subject or object.
#[derive(Debug, Clone)]
pub struct RdfStarTriple {
    pub subject: RdfStarNode,
    pub predicate: String,
    pub object: RdfStarNode,
}

/// A node in RDF-star can be either a regular RDF value or a quoted triple.
#[derive(Debug, Clone)]
pub enum RdfStarNode {
    /// Regular RDF value
    Value(RdfValue),
    /// Quoted triple (embedded statement)
    Quoted(Box<QuotedTriple>),
}

impl RdfStarNode {
    /// Creates a URI node.
    pub fn uri(uri: impl Into<String>) -> Self {
        Self::Value(RdfValue::Uri(uri.into()))
    }

    /// Creates a literal node.
    pub fn literal(s: impl Into<String>) -> Self {
        Self::Value(RdfValue::Literal(s.into(), None))
    }

    /// Creates a quoted triple node.
    pub fn quoted(triple: QuotedTriple) -> Self {
        Self::Quoted(Box::new(triple))
    }

    /// Checks if this node is a quoted triple.
    pub fn is_quoted(&self) -> bool {
        matches!(self, Self::Quoted(_))
    }
}

impl RdfStarTriple {
    /// Creates a new RDF-star triple.
    pub fn new(subject: RdfStarNode, predicate: String, object: RdfStarNode) -> Self {
        Self {
            subject,
            predicate,
            object,
        }
    }

    /// Creates a statement about a triple (reification).
    /// Example: stating who said a particular fact, or when it was asserted.
    pub fn annotate_triple(
        triple: &Triple,
        annotation_predicate: String,
        annotation_object: RdfValue,
    ) -> Self {
        Self {
            subject: RdfStarNode::Quoted(Box::new(QuotedTriple::from_triple(triple))),
            predicate: annotation_predicate,
            object: RdfStarNode::Value(annotation_object),
        }
    }

    /// Checks if this is a statement about a statement (has quoted subject or object).
    pub fn is_meta_statement(&self) -> bool {
        self.subject.is_quoted() || self.object.is_quoted()
    }
}

/// RDF-star serializer for Turtle-star format.
pub struct RdfStarSerializer;

impl RdfStarSerializer {
    /// Serializes an RDF-star triple to Turtle-star format.
    pub fn to_turtle_star(triple: &RdfStarTriple) -> LodResult<String> {
        let subject_str = Self::serialize_node(&triple.subject)?;
        let object_str = Self::serialize_node(&triple.object)?;

        Ok(format!(
            "{} {} {} .",
            subject_str, triple.predicate, object_str
        ))
    }

    /// Serializes a node (can be regular value or quoted triple).
    fn serialize_node(node: &RdfStarNode) -> LodResult<String> {
        match node {
            RdfStarNode::Value(value) => Ok(Self::serialize_value(value)),
            RdfStarNode::Quoted(quoted) => Ok(format!(
                "<< {} {} {} >>",
                quoted.subject,
                quoted.predicate,
                Self::serialize_value(&quoted.object)
            )),
        }
    }

    /// Serializes an RDF value.
    fn serialize_value(value: &RdfValue) -> String {
        match value {
            RdfValue::Uri(uri) => format!("<{}>", uri),
            RdfValue::Literal(s, None) => format!("\"{}\"", escape_string(s)),
            RdfValue::Literal(s, Some(lang)) => format!("\"{}\"@{}", escape_string(s), lang),
            RdfValue::TypedLiteral(s, dtype) => format!("\"{}\"^^{}", escape_string(s), dtype),
            RdfValue::BlankNode(id) => format!("_:{}", id),
        }
    }

    /// Serializes multiple RDF-star triples to Turtle-star format with prefixes.
    pub fn to_turtle_star_document(
        triples: &[RdfStarTriple],
        prefixes: &[(String, String)],
    ) -> LodResult<String> {
        let mut output = String::new();

        // Write prefixes
        for (prefix, uri) in prefixes {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push('\n');

        // Write triples
        for triple in triples {
            output.push_str(&Self::to_turtle_star(triple)?);
            output.push('\n');
        }

        Ok(output)
    }
}

/// RDF-star reification builder for common patterns.
pub struct ReificationBuilder {
    base_triple: Triple,
    annotations: Vec<(String, RdfValue)>,
}

impl ReificationBuilder {
    /// Creates a new reification builder for a triple.
    pub fn new(triple: Triple) -> Self {
        Self {
            base_triple: triple,
            annotations: Vec::new(),
        }
    }

    /// Adds an annotation to the triple.
    pub fn with_annotation(mut self, predicate: String, object: RdfValue) -> Self {
        self.annotations.push((predicate, object));
        self
    }

    /// Adds source provenance (who asserted this).
    pub fn with_source(self, source: impl Into<String>) -> Self {
        self.with_annotation(
            "prov:wasAttributedTo".to_string(),
            RdfValue::Uri(source.into()),
        )
    }

    /// Adds temporal information (when this was asserted).
    pub fn with_time(self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.with_annotation(
            "prov:generatedAtTime".to_string(),
            RdfValue::datetime(timestamp),
        )
    }

    /// Adds confidence level (0.0 to 1.0).
    pub fn with_confidence(self, confidence: f64) -> Self {
        self.with_annotation(
            "legalis:confidence".to_string(),
            RdfValue::TypedLiteral(confidence.to_string(), "xsd:double".to_string()),
        )
    }

    /// Builds RDF-star triples representing the annotations.
    pub fn build(self) -> Vec<RdfStarTriple> {
        let quoted = QuotedTriple::from_triple(&self.base_triple);

        self.annotations
            .into_iter()
            .map(|(pred, obj)| RdfStarTriple {
                subject: RdfStarNode::Quoted(Box::new(quoted.clone())),
                predicate: pred,
                object: RdfStarNode::Value(obj),
            })
            .collect()
    }
}

/// Escapes a string for Turtle format.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Provenance tracking for legal statements using RDF-star.
pub struct ProvenanceTracker;

impl ProvenanceTracker {
    /// Tracks who made a legal statement.
    pub fn track_assertion(
        triple: &Triple,
        author: &str,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> Vec<RdfStarTriple> {
        ReificationBuilder::new(triple.clone())
            .with_source(author)
            .with_time(timestamp)
            .build()
    }

    /// Tracks a judicial interpretation of a statute.
    pub fn track_interpretation(
        original: &Triple,
        court: &str,
        case_id: &str,
        date: chrono::DateTime<chrono::Utc>,
    ) -> Vec<RdfStarTriple> {
        let mut builder = ReificationBuilder::new(original.clone())
            .with_source(court)
            .with_time(date);

        builder.annotations.push((
            "legalis:interpretedIn".to_string(),
            RdfValue::Uri(case_id.to_string()),
        ));

        builder.build()
    }

    /// Tracks conflicting legal opinions about a statement.
    pub fn track_disagreement(
        statement: &Triple,
        supporter: &str,
        opponent: &str,
    ) -> Vec<RdfStarTriple> {
        let quoted = QuotedTriple::from_triple(statement);

        vec![
            RdfStarTriple {
                subject: RdfStarNode::Quoted(Box::new(quoted.clone())),
                predicate: "legalis:supportedBy".to_string(),
                object: RdfStarNode::Value(RdfValue::Uri(supporter.to_string())),
            },
            RdfStarTriple {
                subject: RdfStarNode::Quoted(Box::new(quoted)),
                predicate: "legalis:opposedBy".to_string(),
                object: RdfStarNode::Value(RdfValue::Uri(opponent.to_string())),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_triple() -> Triple {
        Triple {
            subject: "ex:Paris".to_string(),
            predicate: "ex:capitalOf".to_string(),
            object: RdfValue::Uri("ex:France".to_string()),
        }
    }

    #[test]
    fn test_quoted_triple() {
        let triple = sample_triple();
        let quoted = QuotedTriple::from_triple(&triple);

        assert_eq!(quoted.subject, "ex:Paris");
        assert_eq!(quoted.predicate, "ex:capitalOf");
    }

    #[test]
    fn test_rdfstar_triple_annotation() {
        let triple = sample_triple();
        let star_triple = RdfStarTriple::annotate_triple(
            &triple,
            "ex:saidBy".to_string(),
            RdfValue::Uri("ex:John".to_string()),
        );

        assert!(star_triple.is_meta_statement());
        assert!(star_triple.subject.is_quoted());
    }

    #[test]
    fn test_serialize_turtle_star() {
        let triple = sample_triple();
        let star_triple = RdfStarTriple::annotate_triple(
            &triple,
            "ex:saidBy".to_string(),
            RdfValue::Uri("ex:John".to_string()),
        );

        let turtle = RdfStarSerializer::to_turtle_star(&star_triple).unwrap();
        assert!(turtle.contains("<<"));
        assert!(turtle.contains(">>"));
        assert!(turtle.contains("ex:saidBy"));
        assert!(turtle.contains("ex:John"));
    }

    #[test]
    fn test_reification_builder() {
        let triple = sample_triple();
        let star_triples = ReificationBuilder::new(triple)
            .with_source("ex:WikiData")
            .with_confidence(0.95)
            .build();

        assert_eq!(star_triples.len(), 2);
        assert!(
            star_triples
                .iter()
                .any(|t| t.predicate == "prov:wasAttributedTo")
        );
        assert!(
            star_triples
                .iter()
                .any(|t| t.predicate == "legalis:confidence")
        );
    }

    #[test]
    fn test_reification_with_time() {
        let triple = sample_triple();
        let now = Utc::now();
        let star_triples = ReificationBuilder::new(triple).with_time(now).build();

        assert_eq!(star_triples.len(), 1);
        assert_eq!(star_triples[0].predicate, "prov:generatedAtTime");
    }

    #[test]
    fn test_provenance_tracker_assertion() {
        let triple = sample_triple();
        let now = Utc::now();
        let assertions = ProvenanceTracker::track_assertion(&triple, "ex:Author", now);

        assert_eq!(assertions.len(), 2);
        assert!(
            assertions
                .iter()
                .any(|t| t.predicate == "prov:wasAttributedTo")
        );
        assert!(
            assertions
                .iter()
                .any(|t| t.predicate == "prov:generatedAtTime")
        );
    }

    #[test]
    fn test_provenance_tracker_interpretation() {
        let triple = sample_triple();
        let now = Utc::now();
        let interpretations =
            ProvenanceTracker::track_interpretation(&triple, "ex:SupremeCourt", "ex:Case123", now);

        assert!(interpretations.len() >= 2);
        assert!(
            interpretations
                .iter()
                .any(|t| t.predicate == "legalis:interpretedIn")
        );
    }

    #[test]
    fn test_provenance_tracker_disagreement() {
        let triple = sample_triple();
        let disagreements =
            ProvenanceTracker::track_disagreement(&triple, "ex:JudgeA", "ex:JudgeB");

        assert_eq!(disagreements.len(), 2);
        assert!(
            disagreements
                .iter()
                .any(|t| t.predicate == "legalis:supportedBy")
        );
        assert!(
            disagreements
                .iter()
                .any(|t| t.predicate == "legalis:opposedBy")
        );
    }

    #[test]
    fn test_serialize_document() {
        let triple = sample_triple();
        let star_triple = RdfStarTriple::annotate_triple(
            &triple,
            "ex:saidBy".to_string(),
            RdfValue::Uri("ex:John".to_string()),
        );

        let prefixes = vec![("ex".to_string(), "http://example.org/".to_string())];
        let doc = RdfStarSerializer::to_turtle_star_document(&[star_triple], &prefixes).unwrap();

        assert!(doc.contains("@prefix ex:"));
        assert!(doc.contains("<<"));
        assert!(doc.contains(">>"));
    }

    #[test]
    fn test_nested_quoted_literals() {
        let triple = Triple {
            subject: "ex:Person1".to_string(),
            predicate: "ex:claims".to_string(),
            object: RdfValue::Literal("The sky is blue".to_string(), None),
        };

        let star_triple = RdfStarTriple::annotate_triple(
            &triple,
            "ex:confidence".to_string(),
            RdfValue::TypedLiteral("0.8".to_string(), "xsd:double".to_string()),
        );

        let turtle = RdfStarSerializer::to_turtle_star(&star_triple).unwrap();
        assert!(turtle.contains("\"The sky is blue\""));
        assert!(turtle.contains("0.8"));
    }

    #[test]
    fn test_rdfstar_node_creation() {
        let uri_node = RdfStarNode::uri("http://example.org/test");
        assert!(matches!(uri_node, RdfStarNode::Value(_)));
        assert!(!uri_node.is_quoted());

        let literal_node = RdfStarNode::literal("test");
        assert!(matches!(literal_node, RdfStarNode::Value(_)));

        let quoted_node = RdfStarNode::quoted(QuotedTriple::from_triple(&sample_triple()));
        assert!(quoted_node.is_quoted());
    }

    #[test]
    fn test_escape_string() {
        assert_eq!(escape_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_string("say \"hi\""), "say \\\"hi\\\"");
        assert_eq!(escape_string("path\\file"), "path\\\\file");
    }

    #[test]
    fn test_quoted_triple_display() {
        let triple = sample_triple();
        let quoted = QuotedTriple::from_triple(&triple);
        let display = format!("{}", quoted);

        assert!(display.contains("<<"));
        assert!(display.contains(">>"));
        assert!(display.contains("ex:Paris"));
    }

    #[test]
    fn test_complex_annotation_chain() {
        let triple = sample_triple();
        let now = Utc::now();

        let star_triples = ReificationBuilder::new(triple)
            .with_source("ex:Author1")
            .with_time(now)
            .with_confidence(0.9)
            .build();

        assert_eq!(star_triples.len(), 3);

        // All should be meta-statements
        for st in &star_triples {
            assert!(st.is_meta_statement());
        }
    }
}
