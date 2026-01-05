//! SPARQL-star query support for querying RDF-star data.
//!
//! SPARQL-star extends SPARQL to support querying quoted triples (RDF-star).
//! This allows querying metadata about statements.
//!
//! Example SPARQL-star query:
//! ```sparql
//! SELECT ?author ?confidence WHERE {
//!   << :Paris :capitalOf :France >> :saidBy ?author ;
//!                                   :confidence ?confidence .
//! }
//! ```

use crate::rdfstar::{RdfStarNode, RdfStarTriple};
use crate::{LodResult, RdfValue};
use std::collections::HashMap;

/// SPARQL-star query builder.
pub struct SparqlStarQuery {
    prefixes: Vec<(String, String)>,
    patterns: Vec<TriplePattern>,
    variables: Vec<String>,
    filters: Vec<String>,
    order_by: Option<Vec<String>>,
    limit: Option<usize>,
    offset: Option<usize>,
}

/// A triple pattern that can include variables and quoted triples.
#[derive(Debug, Clone)]
pub struct TriplePattern {
    pub subject: PatternNode,
    pub predicate: PatternNode,
    pub object: PatternNode,
}

/// A node in a triple pattern can be a variable, value, or quoted triple.
#[derive(Debug, Clone)]
pub enum PatternNode {
    /// Variable (e.g., ?x, ?author)
    Variable(String),
    /// Concrete value
    Value(RdfValue),
    /// URI
    Uri(String),
    /// Quoted triple pattern
    Quoted(Box<TriplePattern>),
}

impl SparqlStarQuery {
    /// Creates a new SPARQL-star query builder.
    pub fn new() -> Self {
        Self {
            prefixes: Vec::new(),
            patterns: Vec::new(),
            variables: Vec::new(),
            filters: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// Adds a prefix declaration.
    pub fn prefix(mut self, prefix: impl Into<String>, uri: impl Into<String>) -> Self {
        self.prefixes.push((prefix.into(), uri.into()));
        self
    }

    /// Adds a variable to SELECT.
    pub fn select(mut self, var: impl Into<String>) -> Self {
        self.variables.push(var.into());
        self
    }

    /// Adds a triple pattern to the WHERE clause.
    pub fn triple_pattern(
        mut self,
        subject: PatternNode,
        predicate: PatternNode,
        object: PatternNode,
    ) -> Self {
        self.patterns.push(TriplePattern {
            subject,
            predicate,
            object,
        });
        self
    }

    /// Adds a filter condition.
    pub fn filter(mut self, condition: impl Into<String>) -> Self {
        self.filters.push(condition.into());
        self
    }

    /// Sets ORDER BY clause.
    pub fn order_by(mut self, vars: Vec<String>) -> Self {
        self.order_by = Some(vars);
        self
    }

    /// Sets LIMIT clause.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets OFFSET clause.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds the SPARQL-star query string.
    pub fn build(&self) -> LodResult<String> {
        let mut query = String::new();

        // Prefixes
        for (prefix, uri) in &self.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        if !self.prefixes.is_empty() {
            query.push('\n');
        }

        // SELECT
        query.push_str("SELECT ");
        if self.variables.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.variables.join(" "));
        }
        query.push_str("\nWHERE {\n");

        // Triple patterns
        for pattern in &self.patterns {
            query.push_str("  ");
            query.push_str(&Self::serialize_pattern(pattern)?);
            query.push_str(" .\n");
        }

        // Filters
        for filter in &self.filters {
            query.push_str(&format!("  FILTER ({})\n", filter));
        }

        query.push_str("}\n");

        // ORDER BY
        if let Some(ref order_vars) = self.order_by {
            query.push_str(&format!("ORDER BY {}\n", order_vars.join(" ")));
        }

        // LIMIT
        if let Some(limit) = self.limit {
            query.push_str(&format!("LIMIT {}\n", limit));
        }

        // OFFSET
        if let Some(offset) = self.offset {
            query.push_str(&format!("OFFSET {}\n", offset));
        }

        Ok(query)
    }

    fn serialize_pattern(pattern: &TriplePattern) -> LodResult<String> {
        let subject = Self::serialize_node(&pattern.subject)?;
        let predicate = Self::serialize_node(&pattern.predicate)?;
        let object = Self::serialize_node(&pattern.object)?;

        Ok(format!("{} {} {}", subject, predicate, object))
    }

    fn serialize_node(node: &PatternNode) -> LodResult<String> {
        match node {
            PatternNode::Variable(v) => Ok(format!("?{}", v)),
            PatternNode::Uri(uri) => Ok(format!("<{}>", uri)),
            PatternNode::Value(value) => Ok(Self::serialize_value(value)),
            PatternNode::Quoted(pattern) => {
                Ok(format!("<< {} >>", Self::serialize_pattern(pattern)?))
            }
        }
    }

    fn serialize_value(value: &RdfValue) -> String {
        match value {
            RdfValue::Uri(uri) => format!("<{}>", uri),
            RdfValue::Literal(s, None) => format!("\"{}\"", escape_sparql_string(s)),
            RdfValue::Literal(s, Some(lang)) => {
                format!("\"{}\"@{}", escape_sparql_string(s), lang)
            }
            RdfValue::TypedLiteral(s, dtype) => {
                format!("\"{}\"^^{}", escape_sparql_string(s), dtype)
            }
            RdfValue::BlankNode(id) => format!("_:{}", id),
        }
    }
}

impl Default for SparqlStarQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-built SPARQL-star query templates for common legal queries.
pub struct SparqlStarTemplates;

impl SparqlStarTemplates {
    /// Queries for all assertions made by a specific author.
    pub fn find_assertions_by_author(author_uri: &str) -> SparqlStarQuery {
        SparqlStarQuery::new()
            .prefix("prov", "http://www.w3.org/ns/prov#")
            .select("?s")
            .select("?p")
            .select("?o")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("http://www.w3.org/ns/prov#wasAttributedTo".to_string()),
                PatternNode::Uri(author_uri.to_string()),
            )
    }

    /// Queries for statements with confidence above a threshold.
    pub fn find_high_confidence_statements(min_confidence: f64) -> SparqlStarQuery {
        SparqlStarQuery::new()
            .prefix("legalis", "https://legalis.dev/ontology#")
            .select("?s")
            .select("?p")
            .select("?o")
            .select("?conf")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("https://legalis.dev/ontology#confidence".to_string()),
                PatternNode::Variable("conf".to_string()),
            )
            .filter(format!("?conf >= {}", min_confidence))
    }

    /// Queries for judicial interpretations of a specific statute.
    pub fn find_interpretations_of_statute(statute_uri: &str) -> SparqlStarQuery {
        SparqlStarQuery::new()
            .prefix("legalis", "https://legalis.dev/ontology#")
            .prefix("prov", "http://www.w3.org/ns/prov#")
            .select("?court")
            .select("?case")
            .select("?date")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Uri(statute_uri.to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("https://legalis.dev/ontology#interpretedIn".to_string()),
                PatternNode::Variable("case".to_string()),
            )
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Uri(statute_uri.to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("http://www.w3.org/ns/prov#wasAttributedTo".to_string()),
                PatternNode::Variable("court".to_string()),
            )
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Uri(statute_uri.to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("http://www.w3.org/ns/prov#generatedAtTime".to_string()),
                PatternNode::Variable("date".to_string()),
            )
    }

    /// Queries for conflicting statements (statements with both support and opposition).
    pub fn find_disputed_statements() -> SparqlStarQuery {
        SparqlStarQuery::new()
            .prefix("legalis", "https://legalis.dev/ontology#")
            .select("?s")
            .select("?p")
            .select("?o")
            .select("?supporter")
            .select("?opponent")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("https://legalis.dev/ontology#supportedBy".to_string()),
                PatternNode::Variable("supporter".to_string()),
            )
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("https://legalis.dev/ontology#opposedBy".to_string()),
                PatternNode::Variable("opponent".to_string()),
            )
    }

    /// Queries for provenance chain (who derived from whom).
    pub fn find_provenance_chain(statement_uri: &str) -> SparqlStarQuery {
        SparqlStarQuery::new()
            .prefix("prov", "http://www.w3.org/ns/prov#")
            .select("?derived")
            .select("?original")
            .triple_pattern(
                PatternNode::Uri(statement_uri.to_string()),
                PatternNode::Uri("http://www.w3.org/ns/prov#wasDerivedFrom".to_string()),
                PatternNode::Variable("original".to_string()),
            )
    }
}

/// Executes SPARQL-star queries against RDF-star triples in memory.
pub struct SparqlStarExecutor {
    triples: Vec<RdfStarTriple>,
}

impl SparqlStarExecutor {
    /// Creates a new executor with a set of RDF-star triples.
    pub fn new(triples: Vec<RdfStarTriple>) -> Self {
        Self { triples }
    }

    /// Executes a simple pattern match (basic implementation).
    /// Returns variable bindings for each match.
    pub fn execute_pattern(&self, pattern: &TriplePattern) -> Vec<HashMap<String, RdfStarNode>> {
        let mut results = Vec::new();

        for triple in &self.triples {
            if let Some(bindings) = self.match_pattern(pattern, triple) {
                results.push(bindings);
            }
        }

        results
    }

    fn match_pattern(
        &self,
        pattern: &TriplePattern,
        triple: &RdfStarTriple,
    ) -> Option<HashMap<String, RdfStarNode>> {
        let mut bindings = HashMap::new();

        // Match subject
        if !self.match_node(&pattern.subject, &triple.subject, &mut bindings) {
            return None;
        }

        // Match predicate (must be URI in practice)
        if let PatternNode::Variable(v) = &pattern.predicate {
            bindings.insert(v.clone(), RdfStarNode::uri(&triple.predicate));
        } else if let PatternNode::Uri(uri) = &pattern.predicate {
            if uri != &triple.predicate {
                return None;
            }
        }

        // Match object
        if !self.match_node(&pattern.object, &triple.object, &mut bindings) {
            return None;
        }

        Some(bindings)
    }

    fn match_node(
        &self,
        pattern: &PatternNode,
        node: &RdfStarNode,
        bindings: &mut HashMap<String, RdfStarNode>,
    ) -> bool {
        match pattern {
            PatternNode::Variable(v) => {
                // Bind variable
                if let Some(existing) = bindings.get(v) {
                    // Check consistency
                    self.nodes_equal(existing, node)
                } else {
                    bindings.insert(v.clone(), node.clone());
                    true
                }
            }
            PatternNode::Uri(uri) => {
                // Check if node is a URI with the same value
                matches!(node, RdfStarNode::Value(RdfValue::Uri(u)) if u == uri)
            }
            PatternNode::Value(val) => {
                // Check if node has the same value
                matches!(node, RdfStarNode::Value(v) if v == val)
            }
            PatternNode::Quoted(_) => {
                // For simplicity, we just check if it's quoted
                // Full implementation would recursively match the quoted pattern
                matches!(node, RdfStarNode::Quoted(_))
            }
        }
    }

    fn nodes_equal(&self, a: &RdfStarNode, b: &RdfStarNode) -> bool {
        match (a, b) {
            (RdfStarNode::Value(v1), RdfStarNode::Value(v2)) => v1 == v2,
            (RdfStarNode::Quoted(q1), RdfStarNode::Quoted(q2)) => {
                q1.subject == q2.subject && q1.predicate == q2.predicate && q1.object == q2.object
            }
            _ => false,
        }
    }
}

/// Escapes special characters in SPARQL strings.
fn escape_sparql_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_query_build() {
        let query = SparqlStarQuery::new()
            .prefix("ex", "http://example.org/")
            .select("?x")
            .select("?y")
            .triple_pattern(
                PatternNode::Variable("x".to_string()),
                PatternNode::Uri("http://example.org/knows".to_string()),
                PatternNode::Variable("y".to_string()),
            )
            .build()
            .unwrap();

        assert!(query.contains("PREFIX ex:"));
        assert!(query.contains("SELECT ?x ?y"));
        assert!(query.contains("?x <http://example.org/knows> ?y"));
    }

    #[test]
    fn test_quoted_triple_query() {
        let query = SparqlStarQuery::new()
            .select("?author")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Uri("http://example.org/Paris".to_string()),
                    predicate: PatternNode::Uri("http://example.org/capitalOf".to_string()),
                    object: PatternNode::Uri("http://example.org/France".to_string()),
                })),
                PatternNode::Uri("http://example.org/saidBy".to_string()),
                PatternNode::Variable("author".to_string()),
            )
            .build()
            .unwrap();

        assert!(query.contains("<<"));
        assert!(query.contains(">>"));
        assert!(query.contains("?author"));
    }

    #[test]
    fn test_filter_query() {
        let query = SparqlStarQuery::new()
            .select("?x")
            .select("?age")
            .triple_pattern(
                PatternNode::Variable("x".to_string()),
                PatternNode::Uri("http://example.org/age".to_string()),
                PatternNode::Variable("age".to_string()),
            )
            .filter("?age > 18")
            .build()
            .unwrap();

        assert!(query.contains("FILTER (?age > 18)"));
    }

    #[test]
    fn test_limit_offset() {
        let query = SparqlStarQuery::new()
            .select("?x")
            .triple_pattern(
                PatternNode::Variable("x".to_string()),
                PatternNode::Variable("p".to_string()),
                PatternNode::Variable("o".to_string()),
            )
            .limit(10)
            .offset(20)
            .build()
            .unwrap();

        assert!(query.contains("LIMIT 10"));
        assert!(query.contains("OFFSET 20"));
    }

    #[test]
    fn test_order_by() {
        let query = SparqlStarQuery::new()
            .select("?x")
            .select("?y")
            .triple_pattern(
                PatternNode::Variable("x".to_string()),
                PatternNode::Variable("p".to_string()),
                PatternNode::Variable("y".to_string()),
            )
            .order_by(vec!["?x".to_string(), "DESC(?y)".to_string()])
            .build()
            .unwrap();

        assert!(query.contains("ORDER BY ?x DESC(?y)"));
    }

    #[test]
    fn test_template_find_assertions_by_author() {
        let query = SparqlStarTemplates::find_assertions_by_author("http://example.org/Author1")
            .build()
            .unwrap();

        assert!(query.contains("PREFIX prov:"));
        assert!(query.contains("<<"));
        assert!(query.contains("wasAttributedTo"));
    }

    #[test]
    fn test_template_high_confidence() {
        let query = SparqlStarTemplates::find_high_confidence_statements(0.8)
            .build()
            .unwrap();

        assert!(query.contains("confidence"));
        assert!(query.contains("FILTER (?conf >= 0.8)"));
    }

    #[test]
    fn test_template_interpretations() {
        let query =
            SparqlStarTemplates::find_interpretations_of_statute("http://example.org/Statute1")
                .build()
                .unwrap();

        assert!(query.contains("interpretedIn"));
        assert!(query.contains("?court"));
        assert!(query.contains("?case"));
    }

    #[test]
    fn test_template_disputed_statements() {
        let query = SparqlStarTemplates::find_disputed_statements()
            .build()
            .unwrap();

        assert!(query.contains("supportedBy"));
        assert!(query.contains("opposedBy"));
    }

    #[test]
    fn test_executor_simple_match() {
        let triple = RdfStarTriple {
            subject: RdfStarNode::uri("http://example.org/Alice"),
            predicate: "http://example.org/knows".to_string(),
            object: RdfStarNode::uri("http://example.org/Bob"),
        };

        let executor = SparqlStarExecutor::new(vec![triple]);

        let pattern = TriplePattern {
            subject: PatternNode::Variable("x".to_string()),
            predicate: PatternNode::Uri("http://example.org/knows".to_string()),
            object: PatternNode::Variable("y".to_string()),
        };

        let results = executor.execute_pattern(&pattern);
        assert_eq!(results.len(), 1);
        assert!(results[0].contains_key("x"));
        assert!(results[0].contains_key("y"));
    }

    #[test]
    fn test_executor_no_match() {
        let triple = RdfStarTriple {
            subject: RdfStarNode::uri("http://example.org/Alice"),
            predicate: "http://example.org/knows".to_string(),
            object: RdfStarNode::uri("http://example.org/Bob"),
        };

        let executor = SparqlStarExecutor::new(vec![triple]);

        let pattern = TriplePattern {
            subject: PatternNode::Variable("x".to_string()),
            predicate: PatternNode::Uri("http://example.org/likes".to_string()),
            object: PatternNode::Variable("y".to_string()),
        };

        let results = executor.execute_pattern(&pattern);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_escape_sparql_string() {
        assert_eq!(escape_sparql_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_sparql_string("say \"hi\""), "say \\\"hi\\\"");
    }

    #[test]
    fn test_pattern_node_creation() {
        let var = PatternNode::Variable("x".to_string());
        assert!(matches!(var, PatternNode::Variable(_)));

        let uri = PatternNode::Uri("http://example.org/test".to_string());
        assert!(matches!(uri, PatternNode::Uri(_)));

        let val = PatternNode::Value(RdfValue::Literal("test".to_string(), None));
        assert!(matches!(val, PatternNode::Value(_)));
    }

    #[test]
    fn test_complex_quoted_pattern() {
        let query = SparqlStarQuery::new()
            .prefix("ex", "http://example.org/")
            .select("?author")
            .select("?confidence")
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("http://example.org/saidBy".to_string()),
                PatternNode::Variable("author".to_string()),
            )
            .triple_pattern(
                PatternNode::Quoted(Box::new(TriplePattern {
                    subject: PatternNode::Variable("s".to_string()),
                    predicate: PatternNode::Variable("p".to_string()),
                    object: PatternNode::Variable("o".to_string()),
                })),
                PatternNode::Uri("http://example.org/confidence".to_string()),
                PatternNode::Variable("confidence".to_string()),
            )
            .filter("?confidence > 0.5")
            .build()
            .unwrap();

        assert!(query.contains("PREFIX ex:"));
        assert!(query.contains("?author"));
        assert!(query.contains("?confidence"));
        assert!(query.contains("FILTER (?confidence > 0.5)"));
    }
}
