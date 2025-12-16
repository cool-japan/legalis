//! SPARQL query generation for exported RDF data.
//!
//! This module provides utilities to generate SPARQL queries
//! for querying the exported legal statute data.

use std::fmt;

/// A SPARQL query builder for legal statute data.
#[derive(Debug, Clone)]
pub struct SparqlQueryBuilder {
    prefixes: Vec<(String, String)>,
    select_vars: Vec<String>,
    where_patterns: Vec<String>,
    filters: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl Default for SparqlQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SparqlQueryBuilder {
    /// Creates a new SPARQL query builder with standard prefixes.
    pub fn new() -> Self {
        let mut builder = Self {
            prefixes: Vec::new(),
            select_vars: Vec::new(),
            where_patterns: Vec::new(),
            filters: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        };

        // Add standard prefixes
        builder.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
        builder.add_prefix("rdfs", "http://www.w3.org/2000/01/rdf-schema#");
        builder.add_prefix("eli", "http://data.europa.eu/eli/ontology#");
        builder.add_prefix("dcterms", "http://purl.org/dc/terms/");
        builder.add_prefix("skos", "http://www.w3.org/2004/02/skos/core#");
        builder.add_prefix("legalis", "https://legalis.dev/ontology#");

        builder
    }

    /// Adds a custom prefix.
    pub fn add_prefix(&mut self, prefix: impl Into<String>, uri: impl Into<String>) -> &mut Self {
        self.prefixes.push((prefix.into(), uri.into()));
        self
    }

    /// Adds a SELECT variable.
    pub fn select(&mut self, var: impl Into<String>) -> &mut Self {
        self.select_vars.push(var.into());
        self
    }

    /// Adds a WHERE clause pattern.
    pub fn where_pattern(&mut self, pattern: impl Into<String>) -> &mut Self {
        self.where_patterns.push(pattern.into());
        self
    }

    /// Adds a FILTER clause.
    pub fn filter(&mut self, filter: impl Into<String>) -> &mut Self {
        self.filters.push(filter.into());
        self
    }

    /// Sets the ORDER BY clause.
    pub fn order_by(&mut self, var: impl Into<String>) -> &mut Self {
        self.order_by = Some(var.into());
        self
    }

    /// Sets the LIMIT.
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the OFFSET.
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    /// Builds the SPARQL query string.
    pub fn build(&self) -> String {
        let mut query = String::new();

        // Prefixes
        for (prefix, uri) in &self.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        query.push('\n');

        // SELECT
        query.push_str("SELECT ");
        if self.select_vars.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.select_vars.join(" "));
        }
        query.push('\n');

        // WHERE
        query.push_str("WHERE {\n");
        for pattern in &self.where_patterns {
            query.push_str(&format!("  {}\n", pattern));
        }

        // Filters
        for filter in &self.filters {
            query.push_str(&format!("  FILTER ({})\n", filter));
        }

        query.push_str("}\n");

        // ORDER BY
        if let Some(ref order) = self.order_by {
            query.push_str(&format!("ORDER BY {}\n", order));
        }

        // LIMIT
        if let Some(limit) = self.limit {
            query.push_str(&format!("LIMIT {}\n", limit));
        }

        // OFFSET
        if let Some(offset) = self.offset {
            query.push_str(&format!("OFFSET {}\n", offset));
        }

        query
    }
}

impl fmt::Display for SparqlQueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Pre-built SPARQL query templates for common operations.
pub struct SparqlTemplates;

impl SparqlTemplates {
    /// Query to find all statutes.
    pub fn find_all_statutes() -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?title")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .build()
    }

    /// Query to find statutes by jurisdiction.
    pub fn find_by_jurisdiction(jurisdiction: &str) -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?title")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .where_pattern("?statute eli:jurisdiction ?jurisdiction .")
            .filter(format!("?jurisdiction = \"{}\"", jurisdiction))
            .build()
    }

    /// Query to find statutes with specific effect type.
    pub fn find_by_effect_type(effect_type: &str) -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?title")
            .select("?effect")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .where_pattern("?statute legalis:hasEffect ?effect .")
            .where_pattern(format!(
                "?effect legalis:effectType legalis:{} .",
                effect_type
            ))
            .build()
    }

    /// Query to find statutes with age conditions.
    pub fn find_with_age_condition() -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?title")
            .select("?condition")
            .select("?value")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .where_pattern("?statute legalis:hasPrecondition ?condition .")
            .where_pattern("?condition a legalis:AgeCondition .")
            .where_pattern("?condition legalis:value ?value .")
            .build()
    }

    /// Query to find all effects and their descriptions.
    pub fn find_all_effects() -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?effect")
            .select("?effectType")
            .select("?description")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute legalis:hasEffect ?effect .")
            .where_pattern("?effect legalis:effectType ?effectType .")
            .where_pattern("?effect rdfs:label ?description .")
            .build()
    }

    /// Query to find statutes with discretion.
    pub fn find_with_discretion() -> String {
        SparqlQueryBuilder::new()
            .select("?statute")
            .select("?title")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .where_pattern("?statute legalis:hasDiscretion true .")
            .build()
    }

    /// Query to count statutes by effect type.
    pub fn count_by_effect_type() -> String {
        let mut builder = SparqlQueryBuilder::new();
        builder.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
        builder.add_prefix("legalis", "https://legalis.dev/ontology#");

        let mut query = String::new();
        for (prefix, uri) in &builder.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        query.push_str("\nSELECT ?effectType (COUNT(?statute) AS ?count)\n");
        query.push_str("WHERE {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  ?statute legalis:hasEffect ?effect .\n");
        query.push_str("  ?effect legalis:effectType ?effectType .\n");
        query.push_str("}\n");
        query.push_str("GROUP BY ?effectType\n");
        query.push_str("ORDER BY DESC(?count)\n");
        query
    }

    /// CONSTRUCT query to build a subgraph for a specific statute.
    pub fn construct_statute_subgraph(statute_id: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?statute ?p ?o .\n");
        query.push_str("  ?effect ?ep ?eo .\n");
        query.push_str("  ?condition ?cp ?co .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!(
            "  ?statute dcterms:identifier \"{}\" .\n",
            statute_id
        ));
        query.push_str("  ?statute ?p ?o .\n");
        query.push_str("  OPTIONAL {\n");
        query.push_str("    ?statute legalis:hasEffect ?effect .\n");
        query.push_str("    ?effect ?ep ?eo .\n");
        query.push_str("  }\n");
        query.push_str("  OPTIONAL {\n");
        query.push_str("    ?statute legalis:hasPrecondition ?condition .\n");
        query.push_str("    ?condition ?cp ?co .\n");
        query.push_str("  }\n");
        query.push_str("}\n");
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_query() {
        let query = SparqlQueryBuilder::new()
            .select("?s")
            .select("?p")
            .select("?o")
            .where_pattern("?s ?p ?o .")
            .limit(10)
            .build();

        assert!(query.contains("SELECT ?s ?p ?o"));
        assert!(query.contains("WHERE {"));
        assert!(query.contains("LIMIT 10"));
    }

    #[test]
    fn test_find_all_statutes() {
        let query = SparqlTemplates::find_all_statutes();
        assert!(query.contains("SELECT ?statute ?title"));
        assert!(query.contains("a legalis:Statute"));
    }

    #[test]
    fn test_find_by_jurisdiction() {
        let query = SparqlTemplates::find_by_jurisdiction("JP");
        assert!(query.contains("eli:jurisdiction"));
        assert!(query.contains("\"JP\""));
    }

    #[test]
    fn test_construct_subgraph() {
        let query = SparqlTemplates::construct_statute_subgraph("test-123");
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("test-123"));
    }

    #[test]
    fn test_with_filter() {
        let query = SparqlQueryBuilder::new()
            .select("?x")
            .where_pattern("?x rdf:type ?type .")
            .filter("?x > 100")
            .build();

        assert!(query.contains("FILTER (?x > 100)"));
    }

    #[test]
    fn test_count_by_effect_type() {
        let query = SparqlTemplates::count_by_effect_type();
        assert!(query.contains("COUNT(?statute)"));
        assert!(query.contains("GROUP BY"));
    }
}
