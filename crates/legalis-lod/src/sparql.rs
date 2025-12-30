//! SPARQL query generation and endpoint support for exported RDF data.
//!
//! This module provides utilities for:
//! - SPARQL query generation (SELECT, CONSTRUCT, ASK, DESCRIBE)
//! - Federated query support (SERVICE keyword)
//! - Graph store protocol (SPARQL 1.1 Update)
//! - Named graph management
//! - SPARQL endpoint framework

use std::collections::HashMap;
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
    distinct: bool,
    group_by: Vec<String>,
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
            distinct: false,
            group_by: Vec::new(),
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

    /// Enables DISTINCT modifier.
    pub fn distinct(&mut self, distinct: bool) -> &mut Self {
        self.distinct = distinct;
        self
    }

    /// Adds a GROUP BY variable.
    pub fn group_by(&mut self, var: impl Into<String>) -> &mut Self {
        self.group_by.push(var.into());
        self
    }

    /// Adds a property path pattern (SPARQL 1.1 feature).
    /// Example: `?s legalis:hasPrecondition+ ?o` (one or more)
    pub fn property_path(&mut self, subject: &str, path: &str, object: &str) -> &mut Self {
        self.where_patterns
            .push(format!("{} {} {} .", subject, path, object));
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
        if self.distinct {
            query.push_str("DISTINCT ");
        }
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

        // GROUP BY
        if !self.group_by.is_empty() {
            query.push_str(&format!("GROUP BY {}\n", self.group_by.join(" ")));
        }

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

/// Property path builder for SPARQL 1.1 property paths.
#[derive(Debug, Clone)]
pub struct PropertyPath;

impl PropertyPath {
    /// Creates a zero-or-more path: `property*`
    pub fn zero_or_more(property: &str) -> String {
        format!("{}*", property)
    }

    /// Creates a one-or-more path: `property+`
    pub fn one_or_more(property: &str) -> String {
        format!("{}+", property)
    }

    /// Creates a zero-or-one path: `property?`
    pub fn zero_or_one(property: &str) -> String {
        format!("{}?", property)
    }

    /// Creates an alternative path: `property1|property2`
    pub fn alternative(properties: &[&str]) -> String {
        format!("({})", properties.join("|"))
    }

    /// Creates a sequence path: `property1/property2`
    pub fn sequence(properties: &[&str]) -> String {
        properties.join("/")
    }

    /// Creates an inverse path: `^property`
    pub fn inverse(property: &str) -> String {
        format!("^{}", property)
    }

    /// Creates a negated property set: `!(property1|property2)`
    pub fn negated(properties: &[&str]) -> String {
        format!("!({})", properties.join("|"))
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

    /// CONSTRUCT query to extract all statutes with their basic metadata.
    pub fn construct_all_statutes_summary() -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  ?statute eli:title ?title .\n");
        query.push_str("  ?statute dcterms:identifier ?id .\n");
        query.push_str("  ?statute eli:jurisdiction ?jurisdiction .\n");
        query.push_str("  ?statute legalis:hasEffect ?effect .\n");
        query.push_str("  ?effect legalis:effectType ?effectType .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  ?statute eli:title ?title .\n");
        query.push_str("  ?statute dcterms:identifier ?id .\n");
        query.push_str("  OPTIONAL { ?statute eli:jurisdiction ?jurisdiction . }\n");
        query.push_str("  ?statute legalis:hasEffect ?effect .\n");
        query.push_str("  ?effect legalis:effectType ?effectType .\n");
        query.push_str("}\n");
        query
    }

    /// CONSTRUCT query to extract condition hierarchies for complex statutes.
    pub fn construct_condition_hierarchy(statute_id: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n");
        query.push_str("PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?condition rdf:type ?condType .\n");
        query.push_str("  ?condition legalis:operator ?operator .\n");
        query.push_str("  ?condition legalis:value ?value .\n");
        query.push_str("  ?condition legalis:leftOperand ?left .\n");
        query.push_str("  ?condition legalis:rightOperand ?right .\n");
        query.push_str("  ?condition legalis:operand ?operand .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!(
            "  ?statute dcterms:identifier \"{}\" .\n",
            statute_id
        ));
        query.push_str("  ?statute legalis:hasPrecondition ?condition .\n");
        query.push_str("  ?condition rdf:type ?condType .\n");
        query.push_str("  OPTIONAL { ?condition legalis:operator ?operator . }\n");
        query.push_str("  OPTIONAL { ?condition legalis:value ?value . }\n");
        query.push_str("  OPTIONAL { ?condition legalis:leftOperand ?left . }\n");
        query.push_str("  OPTIONAL { ?condition legalis:rightOperand ?right . }\n");
        query.push_str("  OPTIONAL { ?condition legalis:operand ?operand . }\n");
        query.push_str("}\n");
        query
    }

    /// CONSTRUCT query to build a complete SKOS concept scheme.
    pub fn construct_concept_scheme(scheme_id: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX skos: <http://www.w3.org/2004/02/skos/core#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?scheme a skos:ConceptScheme .\n");
        query.push_str("  ?scheme skos:prefLabel ?schemeLabel .\n");
        query.push_str("  ?scheme skos:hasTopConcept ?concept .\n");
        query.push_str("  ?concept a skos:Concept .\n");
        query.push_str("  ?concept skos:prefLabel ?conceptLabel .\n");
        query.push_str("  ?concept skos:definition ?definition .\n");
        query.push_str("  ?concept skos:broader ?broader .\n");
        query.push_str("  ?concept skos:narrower ?narrower .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!("  BIND(<{}> AS ?scheme)\n", scheme_id));
        query.push_str("  ?scheme a skos:ConceptScheme .\n");
        query.push_str("  ?scheme skos:prefLabel ?schemeLabel .\n");
        query.push_str("  ?scheme skos:hasTopConcept ?concept .\n");
        query.push_str("  ?concept a skos:Concept .\n");
        query.push_str("  ?concept skos:prefLabel ?conceptLabel .\n");
        query.push_str("  OPTIONAL { ?concept skos:definition ?definition . }\n");
        query.push_str("  OPTIONAL { ?concept skos:broader ?broader . }\n");
        query.push_str("  OPTIONAL { ?concept skos:narrower ?narrower . }\n");
        query.push_str("}\n");
        query
    }

    /// CONSTRUCT query to extract provenance information for statutes.
    pub fn construct_provenance_graph() -> String {
        let mut query = String::new();
        query.push_str("PREFIX prov: <http://www.w3.org/ns/prov#>\n");
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?statute prov:wasGeneratedBy ?activity .\n");
        query.push_str("  ?statute prov:wasAttributedTo ?agent .\n");
        query.push_str("  ?statute prov:generatedAtTime ?time .\n");
        query.push_str("  ?statute prov:wasDerivedFrom ?source .\n");
        query.push_str("  ?statute dcterms:creator ?creator .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  OPTIONAL { ?statute prov:wasGeneratedBy ?activity . }\n");
        query.push_str("  OPTIONAL { ?statute prov:wasAttributedTo ?agent . }\n");
        query.push_str("  OPTIONAL { ?statute prov:generatedAtTime ?time . }\n");
        query.push_str("  OPTIONAL { ?statute prov:wasDerivedFrom ?source . }\n");
        query.push_str("  OPTIONAL { ?statute dcterms:creator ?creator . }\n");
        query.push_str("}\n");
        query
    }

    /// CONSTRUCT query to build a temporal validity graph.
    pub fn construct_temporal_validity_graph() -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("CONSTRUCT {\n");
        query.push_str("  ?statute dcterms:identifier ?id .\n");
        query.push_str("  ?statute eli:title ?title .\n");
        query.push_str("  ?statute eli:date_document ?effectiveDate .\n");
        query.push_str("  ?statute legalis:expiryDate ?expiryDate .\n");
        query.push_str("}\n");
        query.push_str("WHERE {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  ?statute dcterms:identifier ?id .\n");
        query.push_str("  ?statute eli:title ?title .\n");
        query.push_str("  OPTIONAL { ?statute eli:date_document ?effectiveDate . }\n");
        query.push_str("  OPTIONAL { ?statute legalis:expiryDate ?expiryDate . }\n");
        query.push_str("}\n");
        query
    }

    /// Query using property paths to find all nested conditions (transitive).
    /// Uses property path: legalis:hasPrecondition+ for one-or-more hops.
    pub fn find_all_nested_conditions(statute_id: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n");
        query.push_str("PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n\n");
        query.push_str("SELECT ?condition ?condType\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!(
            "  ?statute dcterms:identifier \"{}\" .\n",
            statute_id
        ));
        query.push_str("  ?statute legalis:hasPrecondition+ ?condition .\n");
        query.push_str("  ?condition rdf:type ?condType .\n");
        query.push_str("}\n");
        query
    }

    /// Query using property paths to navigate effect types.
    /// Uses sequence path: legalis:hasEffect/legalis:effectType
    pub fn find_statutes_by_effect_type_path(effect_type: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n\n");
        query.push_str("SELECT ?statute ?title\n");
        query.push_str("WHERE {\n");
        query.push_str("  ?statute a legalis:Statute .\n");
        query.push_str("  ?statute eli:title ?title .\n");
        query.push_str(&format!(
            "  ?statute legalis:hasEffect/legalis:effectType legalis:{} .\n",
            effect_type
        ));
        query.push_str("}\n");
        query
    }

    /// Query using alternative property paths to find conditions.
    /// Uses alternative path: (legalis:leftOperand|legalis:rightOperand)
    pub fn find_condition_operands(statute_id: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX dcterms: <http://purl.org/dc/terms/>\n\n");
        query.push_str("SELECT ?condition ?operand\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!(
            "  ?statute dcterms:identifier \"{}\" .\n",
            statute_id
        ));
        query.push_str("  ?statute legalis:hasPrecondition ?condition .\n");
        query.push_str("  ?condition (legalis:leftOperand|legalis:rightOperand) ?operand .\n");
        query.push_str("}\n");
        query
    }

    /// Query using inverse property paths to find what references a statute.
    /// Uses inverse path: ^legalis:references
    pub fn find_referencing_statutes(statute_uri: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n\n");
        query.push_str("SELECT ?referencingStatute ?title\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!(
            "  <{}> ^legalis:references ?referencingStatute .\n",
            statute_uri
        ));
        query.push_str("  ?referencingStatute eli:title ?title .\n");
        query.push_str("}\n");
        query
    }

    /// Query using DISTINCT to find unique jurisdictions.
    pub fn find_distinct_jurisdictions() -> String {
        SparqlQueryBuilder::new()
            .distinct(true)
            .select("?jurisdiction")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:jurisdiction ?jurisdiction .")
            .order_by("?jurisdiction")
            .build()
    }

    /// Query using GROUP BY to count statutes per jurisdiction.
    pub fn count_statutes_per_jurisdiction() -> String {
        let mut builder = SparqlQueryBuilder::new();
        builder
            .select("?jurisdiction")
            .select("(COUNT(?statute) AS ?count)")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:jurisdiction ?jurisdiction .")
            .group_by("?jurisdiction")
            .order_by("DESC(?count)");
        builder.build()
    }

    /// Query to find related statutes using SKOS broader/narrower relationships.
    /// Uses property path: skos:broader* for zero-or-more hops.
    pub fn find_related_statutes_skos(statute_uri: &str) -> String {
        let mut query = String::new();
        query.push_str("PREFIX skos: <http://www.w3.org/2004/02/skos/core#>\n");
        query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n\n");
        query.push_str("SELECT ?relatedStatute ?title ?relation\n");
        query.push_str("WHERE {\n");
        query.push_str(&format!("  <{}> dcterms:subject ?concept .\n", statute_uri));
        query.push_str("  ?concept skos:broader* ?relatedConcept .\n");
        query.push_str("  ?relatedStatute dcterms:subject ?relatedConcept .\n");
        query.push_str("  ?relatedStatute eli:title ?title .\n");
        query.push_str(&format!("  FILTER(?relatedStatute != <{}>)\n", statute_uri));
        query.push_str("}\n");
        query
    }
}

/// Federated query builder for querying multiple SPARQL endpoints.
#[derive(Debug, Clone)]
pub struct FederatedQueryBuilder {
    base_builder: SparqlQueryBuilder,
    service_patterns: Vec<ServicePattern>,
}

/// A SERVICE pattern for federated queries.
#[derive(Debug, Clone)]
pub struct ServicePattern {
    endpoint: String,
    patterns: Vec<String>,
    silent: bool,
}

impl FederatedQueryBuilder {
    /// Creates a new federated query builder.
    pub fn new() -> Self {
        Self {
            base_builder: SparqlQueryBuilder::new(),
            service_patterns: Vec::new(),
        }
    }

    /// Adds a SELECT variable.
    pub fn select(&mut self, var: impl Into<String>) -> &mut Self {
        self.base_builder.select(var);
        self
    }

    /// Adds a local WHERE pattern.
    pub fn where_pattern(&mut self, pattern: impl Into<String>) -> &mut Self {
        self.base_builder.where_pattern(pattern);
        self
    }

    /// Adds a SERVICE pattern for federated querying.
    pub fn service(
        &mut self,
        endpoint: impl Into<String>,
        patterns: Vec<String>,
        silent: bool,
    ) -> &mut Self {
        self.service_patterns.push(ServicePattern {
            endpoint: endpoint.into(),
            patterns,
            silent,
        });
        self
    }

    /// Queries EUR-Lex endpoint.
    pub fn service_eurlex(&mut self, patterns: Vec<String>) -> &mut Self {
        self.service(
            "https://publications.europa.eu/webapi/rdf/sparql",
            patterns,
            true,
        )
    }

    /// Queries Wikidata endpoint.
    pub fn service_wikidata(&mut self, patterns: Vec<String>) -> &mut Self {
        self.service("https://query.wikidata.org/sparql", patterns, true)
    }

    /// Queries DBpedia endpoint.
    pub fn service_dbpedia(&mut self, patterns: Vec<String>) -> &mut Self {
        self.service("https://dbpedia.org/sparql", patterns, true)
    }

    /// Sets the LIMIT.
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.base_builder.limit(limit);
        self
    }

    /// Builds the federated SPARQL query.
    pub fn build(&self) -> String {
        let mut query = String::new();

        // Prefixes
        for (prefix, uri) in &self.base_builder.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        query.push('\n');

        // SELECT
        query.push_str("SELECT ");
        if self.base_builder.select_vars.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.base_builder.select_vars.join(" "));
        }
        query.push('\n');

        // WHERE
        query.push_str("WHERE {\n");

        // Local patterns
        for pattern in &self.base_builder.where_patterns {
            query.push_str(&format!("  {}\n", pattern));
        }

        // SERVICE patterns
        for service in &self.service_patterns {
            if service.silent {
                query.push_str(&format!("  SERVICE SILENT <{}> {{\n", service.endpoint));
            } else {
                query.push_str(&format!("  SERVICE <{}> {{\n", service.endpoint));
            }
            for pattern in &service.patterns {
                query.push_str(&format!("    {}\n", pattern));
            }
            query.push_str("  }\n");
        }

        query.push_str("}\n");

        // LIMIT
        if let Some(limit) = self.base_builder.limit {
            query.push_str(&format!("LIMIT {}\n", limit));
        }

        query
    }
}

impl Default for FederatedQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// SPARQL 1.1 Update operations for graph store protocol.
#[derive(Debug, Clone)]
pub struct SparqlUpdate {
    prefixes: Vec<(String, String)>,
    operations: Vec<UpdateOperation>,
}

/// Types of SPARQL update operations.
#[derive(Debug, Clone)]
pub enum UpdateOperation {
    /// INSERT DATA operation
    InsertData {
        graph: Option<String>,
        triples: String,
    },
    /// DELETE DATA operation
    DeleteData {
        graph: Option<String>,
        triples: String,
    },
    /// DELETE WHERE operation
    DeleteWhere { patterns: String },
    /// INSERT/DELETE operation
    Modify {
        graph: Option<String>,
        delete: String,
        insert: String,
        where_clause: String,
    },
    /// CLEAR operation
    Clear { graph: String, silent: bool },
    /// DROP operation
    Drop { graph: String, silent: bool },
    /// CREATE operation
    Create { graph: String, silent: bool },
    /// LOAD operation
    Load {
        source: String,
        graph: Option<String>,
        silent: bool,
    },
}

impl SparqlUpdate {
    /// Creates a new SPARQL update builder.
    pub fn new() -> Self {
        Self {
            prefixes: vec![
                (
                    "rdf".to_string(),
                    "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                ),
                (
                    "rdfs".to_string(),
                    "http://www.w3.org/2000/01/rdf-schema#".to_string(),
                ),
                (
                    "legalis".to_string(),
                    "https://legalis.dev/ontology#".to_string(),
                ),
            ],
            operations: Vec::new(),
        }
    }

    /// Adds a prefix.
    pub fn add_prefix(&mut self, prefix: impl Into<String>, uri: impl Into<String>) -> &mut Self {
        self.prefixes.push((prefix.into(), uri.into()));
        self
    }

    /// Adds an INSERT DATA operation.
    pub fn insert_data(&mut self, graph: Option<String>, triples: impl Into<String>) -> &mut Self {
        self.operations.push(UpdateOperation::InsertData {
            graph,
            triples: triples.into(),
        });
        self
    }

    /// Adds a DELETE DATA operation.
    pub fn delete_data(&mut self, graph: Option<String>, triples: impl Into<String>) -> &mut Self {
        self.operations.push(UpdateOperation::DeleteData {
            graph,
            triples: triples.into(),
        });
        self
    }

    /// Adds a DELETE WHERE operation.
    pub fn delete_where(&mut self, patterns: impl Into<String>) -> &mut Self {
        self.operations.push(UpdateOperation::DeleteWhere {
            patterns: patterns.into(),
        });
        self
    }

    /// Adds an INSERT/DELETE operation.
    #[allow(clippy::too_many_arguments)]
    pub fn modify(
        &mut self,
        graph: Option<String>,
        delete: impl Into<String>,
        insert: impl Into<String>,
        where_clause: impl Into<String>,
    ) -> &mut Self {
        self.operations.push(UpdateOperation::Modify {
            graph,
            delete: delete.into(),
            insert: insert.into(),
            where_clause: where_clause.into(),
        });
        self
    }

    /// Adds a CLEAR operation.
    pub fn clear(&mut self, graph: impl Into<String>, silent: bool) -> &mut Self {
        self.operations.push(UpdateOperation::Clear {
            graph: graph.into(),
            silent,
        });
        self
    }

    /// Adds a DROP operation.
    pub fn drop(&mut self, graph: impl Into<String>, silent: bool) -> &mut Self {
        self.operations.push(UpdateOperation::Drop {
            graph: graph.into(),
            silent,
        });
        self
    }

    /// Adds a CREATE operation.
    pub fn create(&mut self, graph: impl Into<String>, silent: bool) -> &mut Self {
        self.operations.push(UpdateOperation::Create {
            graph: graph.into(),
            silent,
        });
        self
    }

    /// Adds a LOAD operation.
    pub fn load(
        &mut self,
        source: impl Into<String>,
        graph: Option<String>,
        silent: bool,
    ) -> &mut Self {
        self.operations.push(UpdateOperation::Load {
            source: source.into(),
            graph,
            silent,
        });
        self
    }

    /// Builds the SPARQL update string.
    pub fn build(&self) -> String {
        let mut update = String::new();

        // Prefixes
        for (prefix, uri) in &self.prefixes {
            update.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        if !self.prefixes.is_empty() {
            update.push('\n');
        }

        // Operations
        for op in &self.operations {
            match op {
                UpdateOperation::InsertData { graph, triples } => {
                    if let Some(g) = graph {
                        update.push_str(&format!("INSERT DATA {{\n  GRAPH <{}> {{\n", g));
                        update.push_str(&format!("    {}\n", triples));
                        update.push_str("  }\n}\n\n");
                    } else {
                        update.push_str("INSERT DATA {\n");
                        update.push_str(&format!("  {}\n", triples));
                        update.push_str("}\n\n");
                    }
                }
                UpdateOperation::DeleteData { graph, triples } => {
                    if let Some(g) = graph {
                        update.push_str(&format!("DELETE DATA {{\n  GRAPH <{}> {{\n", g));
                        update.push_str(&format!("    {}\n", triples));
                        update.push_str("  }\n}\n\n");
                    } else {
                        update.push_str("DELETE DATA {\n");
                        update.push_str(&format!("  {}\n", triples));
                        update.push_str("}\n\n");
                    }
                }
                UpdateOperation::DeleteWhere { patterns } => {
                    update.push_str("DELETE WHERE {\n");
                    update.push_str(&format!("  {}\n", patterns));
                    update.push_str("}\n\n");
                }
                UpdateOperation::Modify {
                    graph,
                    delete,
                    insert,
                    where_clause,
                } => {
                    if let Some(g) = graph {
                        update.push_str(&format!("WITH <{}>\n", g));
                    }
                    update.push_str("DELETE {\n");
                    update.push_str(&format!("  {}\n", delete));
                    update.push_str("}\n");
                    update.push_str("INSERT {\n");
                    update.push_str(&format!("  {}\n", insert));
                    update.push_str("}\n");
                    update.push_str("WHERE {\n");
                    update.push_str(&format!("  {}\n", where_clause));
                    update.push_str("}\n\n");
                }
                UpdateOperation::Clear { graph, silent } => {
                    if *silent {
                        update.push_str(&format!("CLEAR SILENT GRAPH <{}>\n\n", graph));
                    } else {
                        update.push_str(&format!("CLEAR GRAPH <{}>\n\n", graph));
                    }
                }
                UpdateOperation::Drop { graph, silent } => {
                    if *silent {
                        update.push_str(&format!("DROP SILENT GRAPH <{}>\n\n", graph));
                    } else {
                        update.push_str(&format!("DROP GRAPH <{}>\n\n", graph));
                    }
                }
                UpdateOperation::Create { graph, silent } => {
                    if *silent {
                        update.push_str(&format!("CREATE SILENT GRAPH <{}>\n\n", graph));
                    } else {
                        update.push_str(&format!("CREATE GRAPH <{}>\n\n", graph));
                    }
                }
                UpdateOperation::Load {
                    source,
                    graph,
                    silent,
                } => {
                    let silent_str = if *silent { "SILENT " } else { "" };
                    if let Some(g) = graph {
                        update.push_str(&format!(
                            "LOAD {}<{}> INTO GRAPH <{}>\n\n",
                            silent_str, source, g
                        ));
                    } else {
                        update.push_str(&format!("LOAD {}<{}>\n\n", silent_str, source));
                    }
                }
            }
        }

        update
    }
}

impl Default for SparqlUpdate {
    fn default() -> Self {
        Self::new()
    }
}

/// Named graph manager for organizing RDF data.
#[derive(Debug, Clone)]
pub struct NamedGraphManager {
    base_uri: String,
    graphs: HashMap<String, GraphMetadata>,
}

/// Metadata for a named graph.
#[derive(Debug, Clone)]
pub struct GraphMetadata {
    pub graph_uri: String,
    pub label: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}

impl NamedGraphManager {
    /// Creates a new named graph manager.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            graphs: HashMap::new(),
        }
    }

    /// Registers a named graph.
    pub fn register_graph(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
    ) -> &mut GraphMetadata {
        let id_str = id.into();
        let graph_uri = format!("{}graph/{}", self.base_uri, id_str);
        self.graphs.insert(
            id_str.clone(),
            GraphMetadata {
                graph_uri,
                label: label.into(),
                description: None,
                created_at: None,
                modified_at: None,
            },
        );
        self.graphs.get_mut(&id_str).unwrap()
    }

    /// Gets graph metadata.
    pub fn get_graph(&self, id: &str) -> Option<&GraphMetadata> {
        self.graphs.get(id)
    }

    /// Gets graph URI.
    pub fn graph_uri(&self, id: &str) -> Option<String> {
        self.graphs.get(id).map(|g| g.graph_uri.clone())
    }

    /// Lists all registered graphs.
    pub fn list_graphs(&self) -> Vec<&GraphMetadata> {
        self.graphs.values().collect()
    }

    /// Generates a CREATE operation for a graph.
    pub fn create_graph_update(&self, id: &str) -> Option<String> {
        self.graphs.get(id).map(|g| {
            let mut update = SparqlUpdate::new();
            update.create(g.graph_uri.clone(), false);
            update.build()
        })
    }

    /// Generates a DROP operation for a graph.
    pub fn drop_graph_update(&self, id: &str, silent: bool) -> Option<String> {
        self.graphs.get(id).map(|g| {
            let mut update = SparqlUpdate::new();
            update.drop(g.graph_uri.clone(), silent);
            update.build()
        })
    }

    /// Generates a query to select from a specific graph.
    pub fn select_from_graph(
        &self,
        id: &str,
        select_vars: Vec<&str>,
        patterns: Vec<&str>,
    ) -> Option<String> {
        self.graphs.get(id).map(|g| {
            let mut query = String::new();
            query.push_str("PREFIX legalis: <https://legalis.dev/ontology#>\n");
            query.push_str("PREFIX eli: <http://data.europa.eu/eli/ontology#>\n\n");
            query.push_str(&format!("SELECT {}\n", select_vars.join(" ")));
            query.push_str(&format!("FROM <{}>\n", g.graph_uri));
            query.push_str("WHERE {\n");
            for pattern in patterns {
                query.push_str(&format!("  {}\n", pattern));
            }
            query.push_str("}\n");
            query
        })
    }
}

/// Type alias for SPARQL query executor function.
type QueryExecutor = Box<dyn Fn(&str) -> Result<String, String>>;

/// Type alias for SPARQL update executor function.
type UpdateExecutor = Box<dyn Fn(&str) -> Result<(), String>>;

/// SPARQL endpoint framework (generic, can be used with any HTTP server).
pub struct SparqlEndpoint {
    /// Query executor function
    query_executor: Option<QueryExecutor>,
    /// Update executor function
    update_executor: Option<UpdateExecutor>,
}

impl SparqlEndpoint {
    /// Creates a new SPARQL endpoint.
    pub fn new() -> Self {
        Self {
            query_executor: None,
            update_executor: None,
        }
    }

    /// Sets the query executor.
    pub fn with_query_executor<F>(mut self, executor: F) -> Self
    where
        F: Fn(&str) -> Result<String, String> + 'static,
    {
        self.query_executor = Some(Box::new(executor));
        self
    }

    /// Sets the update executor.
    pub fn with_update_executor<F>(mut self, executor: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.update_executor = Some(Box::new(executor));
        self
    }

    /// Executes a SPARQL query.
    pub fn execute_query(&self, query: &str) -> Result<String, String> {
        if let Some(ref executor) = self.query_executor {
            executor(query)
        } else {
            Err("No query executor configured".to_string())
        }
    }

    /// Executes a SPARQL update.
    pub fn execute_update(&self, update: &str) -> Result<(), String> {
        if let Some(ref executor) = self.update_executor {
            executor(update)
        } else {
            Err("No update executor configured".to_string())
        }
    }

    /// Validates a SPARQL query (basic syntax check).
    pub fn validate_query(query: &str) -> bool {
        let query_upper = query.to_uppercase();
        query_upper.contains("SELECT")
            || query_upper.contains("CONSTRUCT")
            || query_upper.contains("ASK")
            || query_upper.contains("DESCRIBE")
    }

    /// Validates a SPARQL update (basic syntax check).
    pub fn validate_update(update: &str) -> bool {
        let update_upper = update.to_uppercase();
        update_upper.contains("INSERT")
            || update_upper.contains("DELETE")
            || update_upper.contains("CLEAR")
            || update_upper.contains("DROP")
            || update_upper.contains("CREATE")
            || update_upper.contains("LOAD")
    }
}

impl Default for SparqlEndpoint {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_construct_all_statutes_summary() {
        let query = SparqlTemplates::construct_all_statutes_summary();
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("eli:title"));
        assert!(query.contains("dcterms:identifier"));
    }

    #[test]
    fn test_construct_condition_hierarchy() {
        let query = SparqlTemplates::construct_condition_hierarchy("test-123");
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("test-123"));
        assert!(query.contains("legalis:leftOperand"));
        assert!(query.contains("legalis:rightOperand"));
    }

    #[test]
    fn test_construct_concept_scheme() {
        let query = SparqlTemplates::construct_concept_scheme("https://example.org/scheme");
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("skos:ConceptScheme"));
        assert!(query.contains("skos:prefLabel"));
    }

    #[test]
    fn test_construct_provenance_graph() {
        let query = SparqlTemplates::construct_provenance_graph();
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("prov:wasGeneratedBy"));
        assert!(query.contains("prov:wasAttributedTo"));
    }

    #[test]
    fn test_construct_temporal_validity_graph() {
        let query = SparqlTemplates::construct_temporal_validity_graph();
        assert!(query.contains("CONSTRUCT"));
        assert!(query.contains("eli:date_document"));
        assert!(query.contains("legalis:expiryDate"));
    }

    // Federated Query Tests
    #[test]
    fn test_federated_query_basic() {
        let mut builder = FederatedQueryBuilder::new();
        builder
            .select("?statute")
            .select("?title")
            .where_pattern("?statute a legalis:Statute .")
            .where_pattern("?statute eli:title ?title .")
            .service(
                "https://example.org/sparql",
                vec!["?statute owl:sameAs ?external .".to_string()],
                true,
            );

        let query = builder.build();
        assert!(query.contains("SELECT ?statute ?title"));
        assert!(query.contains("SERVICE SILENT"));
        assert!(query.contains("https://example.org/sparql"));
    }

    #[test]
    fn test_federated_query_wikidata() {
        let mut builder = FederatedQueryBuilder::new();
        builder
            .select("?statute")
            .select("?wikidataLabel")
            .where_pattern("?statute a legalis:Statute .")
            .service_wikidata(vec![
                "?statute owl:sameAs ?wikidata .".to_string(),
                "?wikidata rdfs:label ?wikidataLabel .".to_string(),
            ]);

        let query = builder.build();
        assert!(query.contains("query.wikidata.org"));
        assert!(query.contains("SERVICE SILENT"));
    }

    #[test]
    fn test_federated_query_eurlex() {
        let mut builder = FederatedQueryBuilder::new();
        builder
            .select("?statute")
            .service_eurlex(vec!["?statute eli:title ?euTitle .".to_string()]);

        let query = builder.build();
        assert!(query.contains("publications.europa.eu"));
    }

    // SPARQL Update Tests
    #[test]
    fn test_sparql_update_insert_data() {
        let mut update = SparqlUpdate::new();
        update.insert_data(
            None,
            "<http://example.org/s> <http://example.org/p> <http://example.org/o> .",
        );

        let result = update.build();
        assert!(result.contains("INSERT DATA"));
        assert!(result.contains("http://example.org/s"));
    }

    #[test]
    fn test_sparql_update_insert_data_with_graph() {
        let mut update = SparqlUpdate::new();
        update.insert_data(
            Some("https://example.org/graph/test".to_string()),
            "<http://example.org/s> <http://example.org/p> <http://example.org/o> .",
        );

        let result = update.build();
        assert!(result.contains("INSERT DATA"));
        assert!(result.contains("GRAPH <https://example.org/graph/test>"));
    }

    #[test]
    fn test_sparql_update_delete_data() {
        let mut update = SparqlUpdate::new();
        update.delete_data(
            None,
            "<http://example.org/s> <http://example.org/p> <http://example.org/o> .",
        );

        let result = update.build();
        assert!(result.contains("DELETE DATA"));
    }

    #[test]
    fn test_sparql_update_delete_where() {
        let mut update = SparqlUpdate::new();
        update.delete_where("?s ?p ?o . FILTER(?o = \"old value\")");

        let result = update.build();
        assert!(result.contains("DELETE WHERE"));
        assert!(result.contains("FILTER"));
    }

    #[test]
    fn test_sparql_update_modify() {
        let mut update = SparqlUpdate::new();
        update.modify(
            None,
            "?s eli:title ?oldTitle .",
            "?s eli:title \"New Title\" .",
            "?s a legalis:Statute . ?s eli:title ?oldTitle .",
        );

        let result = update.build();
        assert!(result.contains("DELETE {"));
        assert!(result.contains("INSERT {"));
        assert!(result.contains("WHERE {"));
        assert!(result.contains("New Title"));
    }

    #[test]
    fn test_sparql_update_clear() {
        let mut update = SparqlUpdate::new();
        update.clear("https://example.org/graph/test", true);

        let result = update.build();
        assert!(result.contains("CLEAR SILENT GRAPH"));
    }

    #[test]
    fn test_sparql_update_drop() {
        let mut update = SparqlUpdate::new();
        update.drop("https://example.org/graph/test", false);

        let result = update.build();
        assert!(result.contains("DROP GRAPH"));
        assert!(!result.contains("SILENT"));
    }

    #[test]
    fn test_sparql_update_create() {
        let mut update = SparqlUpdate::new();
        update.create("https://example.org/graph/new", false);

        let result = update.build();
        assert!(result.contains("CREATE GRAPH"));
    }

    #[test]
    fn test_sparql_update_load() {
        let mut update = SparqlUpdate::new();
        update.load(
            "https://example.org/data.ttl",
            Some("https://example.org/graph/loaded".to_string()),
            true,
        );

        let result = update.build();
        assert!(result.contains("LOAD SILENT"));
        assert!(result.contains("INTO GRAPH"));
    }

    // Named Graph Manager Tests
    #[test]
    fn test_named_graph_manager_register() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("statutes-2023", "Statutes from 2023");

        let graph = manager.get_graph("statutes-2023");
        assert!(graph.is_some());
        assert_eq!(graph.unwrap().label, "Statutes from 2023");
    }

    #[test]
    fn test_named_graph_manager_get_uri() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("test", "Test Graph");

        let uri = manager.graph_uri("test");
        assert_eq!(uri, Some("https://example.org/graph/test".to_string()));
    }

    #[test]
    fn test_named_graph_manager_list() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("graph1", "Graph 1");
        manager.register_graph("graph2", "Graph 2");

        let graphs = manager.list_graphs();
        assert_eq!(graphs.len(), 2);
    }

    #[test]
    fn test_named_graph_manager_create_update() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("test", "Test Graph");

        let update = manager.create_graph_update("test");
        assert!(update.is_some());
        assert!(update.unwrap().contains("CREATE GRAPH"));
    }

    #[test]
    fn test_named_graph_manager_drop_update() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("test", "Test Graph");

        let update = manager.drop_graph_update("test", true);
        assert!(update.is_some());
        assert!(update.unwrap().contains("DROP SILENT GRAPH"));
    }

    #[test]
    fn test_named_graph_manager_select_from_graph() {
        let mut manager = NamedGraphManager::new("https://example.org/");
        manager.register_graph("test", "Test Graph");

        let query = manager.select_from_graph("test", vec!["?s", "?p", "?o"], vec!["?s ?p ?o ."]);

        assert!(query.is_some());
        let query_str = query.unwrap();
        assert!(query_str.contains("SELECT ?s ?p ?o"));
        assert!(query_str.contains("FROM <https://example.org/graph/test>"));
    }

    // SPARQL Endpoint Tests
    #[test]
    fn test_sparql_endpoint_query_executor() {
        let endpoint = SparqlEndpoint::new().with_query_executor(|query| {
            if query.contains("SELECT") {
                Ok("Results".to_string())
            } else {
                Err("Invalid query".to_string())
            }
        });

        let result = endpoint.execute_query("SELECT * WHERE { ?s ?p ?o }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Results");
    }

    #[test]
    fn test_sparql_endpoint_update_executor() {
        let endpoint = SparqlEndpoint::new().with_update_executor(|update| {
            if update.contains("INSERT") {
                Ok(())
            } else {
                Err("Invalid update".to_string())
            }
        });

        let result = endpoint.execute_update("INSERT DATA { <s> <p> <o> }");
        assert!(result.is_ok());
    }

    #[test]
    fn test_sparql_endpoint_validate_query() {
        assert!(SparqlEndpoint::validate_query(
            "SELECT * WHERE { ?s ?p ?o }"
        ));
        assert!(SparqlEndpoint::validate_query(
            "CONSTRUCT { ?s ?p ?o } WHERE { ?s ?p ?o }"
        ));
        assert!(SparqlEndpoint::validate_query("ASK { ?s ?p ?o }"));
        assert!(SparqlEndpoint::validate_query(
            "DESCRIBE <http://example.org/resource>"
        ));
        assert!(!SparqlEndpoint::validate_query("INVALID QUERY"));
    }

    #[test]
    fn test_sparql_endpoint_validate_update() {
        assert!(SparqlEndpoint::validate_update(
            "INSERT DATA { <s> <p> <o> }"
        ));
        assert!(SparqlEndpoint::validate_update(
            "DELETE DATA { <s> <p> <o> }"
        ));
        assert!(SparqlEndpoint::validate_update("CLEAR GRAPH <g>"));
        assert!(SparqlEndpoint::validate_update("DROP GRAPH <g>"));
        assert!(SparqlEndpoint::validate_update("CREATE GRAPH <g>"));
        assert!(!SparqlEndpoint::validate_update(
            "SELECT * WHERE { ?s ?p ?o }"
        ));
    }

    #[test]
    fn test_sparql_endpoint_no_executor() {
        let endpoint = SparqlEndpoint::new();
        let result = endpoint.execute_query("SELECT * WHERE { ?s ?p ?o }");
        assert!(result.is_err());
    }

    // Property Path Tests
    #[test]
    fn test_property_path_zero_or_more() {
        let path = PropertyPath::zero_or_more("legalis:hasPrecondition");
        assert_eq!(path, "legalis:hasPrecondition*");
    }

    #[test]
    fn test_property_path_one_or_more() {
        let path = PropertyPath::one_or_more("legalis:hasPrecondition");
        assert_eq!(path, "legalis:hasPrecondition+");
    }

    #[test]
    fn test_property_path_zero_or_one() {
        let path = PropertyPath::zero_or_one("legalis:hasEffect");
        assert_eq!(path, "legalis:hasEffect?");
    }

    #[test]
    fn test_property_path_alternative() {
        let path = PropertyPath::alternative(&["legalis:leftOperand", "legalis:rightOperand"]);
        assert_eq!(path, "(legalis:leftOperand|legalis:rightOperand)");
    }

    #[test]
    fn test_property_path_sequence() {
        let path = PropertyPath::sequence(&["legalis:hasEffect", "legalis:effectType"]);
        assert_eq!(path, "legalis:hasEffect/legalis:effectType");
    }

    #[test]
    fn test_property_path_inverse() {
        let path = PropertyPath::inverse("legalis:references");
        assert_eq!(path, "^legalis:references");
    }

    #[test]
    fn test_property_path_negated() {
        let path = PropertyPath::negated(&["rdf:type", "rdfs:subClassOf"]);
        assert_eq!(path, "!(rdf:type|rdfs:subClassOf)");
    }

    #[test]
    fn test_property_path_in_query() {
        let mut builder = SparqlQueryBuilder::new();
        builder
            .select("?statute")
            .select("?condition")
            .where_pattern("?statute a legalis:Statute .")
            .property_path("?statute", "legalis:hasPrecondition+", "?condition");

        let query = builder.build();
        assert!(query.contains("legalis:hasPrecondition+"));
    }

    #[test]
    fn test_distinct_query() {
        let mut builder = SparqlQueryBuilder::new();
        builder
            .distinct(true)
            .select("?jurisdiction")
            .where_pattern("?statute eli:jurisdiction ?jurisdiction .");

        let query = builder.build();
        assert!(query.contains("SELECT DISTINCT ?jurisdiction"));
    }

    #[test]
    fn test_group_by_query() {
        let mut builder = SparqlQueryBuilder::new();
        builder
            .select("?type")
            .select("(COUNT(*) AS ?count)")
            .where_pattern("?s rdf:type ?type .")
            .group_by("?type");

        let query = builder.build();
        assert!(query.contains("GROUP BY ?type"));
    }

    #[test]
    fn test_find_all_nested_conditions() {
        let query = SparqlTemplates::find_all_nested_conditions("test-123");
        assert!(query.contains("legalis:hasPrecondition+"));
        assert!(query.contains("test-123"));
    }

    #[test]
    fn test_find_statutes_by_effect_type_path() {
        let query = SparqlTemplates::find_statutes_by_effect_type_path("GrantEffect");
        assert!(query.contains("legalis:hasEffect/legalis:effectType"));
        assert!(query.contains("GrantEffect"));
    }

    #[test]
    fn test_find_condition_operands() {
        let query = SparqlTemplates::find_condition_operands("test-123");
        assert!(query.contains("(legalis:leftOperand|legalis:rightOperand)"));
    }

    #[test]
    fn test_find_referencing_statutes() {
        let query = SparqlTemplates::find_referencing_statutes("http://example.org/statute/123");
        assert!(query.contains("^legalis:references"));
    }

    #[test]
    fn test_find_distinct_jurisdictions() {
        let query = SparqlTemplates::find_distinct_jurisdictions();
        assert!(query.contains("SELECT DISTINCT"));
        assert!(query.contains("eli:jurisdiction"));
    }

    #[test]
    fn test_count_statutes_per_jurisdiction() {
        let query = SparqlTemplates::count_statutes_per_jurisdiction();
        assert!(query.contains("COUNT(?statute)"));
        assert!(query.contains("GROUP BY ?jurisdiction"));
    }

    #[test]
    fn test_find_related_statutes_skos() {
        let query = SparqlTemplates::find_related_statutes_skos("http://example.org/statute/123");
        assert!(query.contains("skos:broader*"));
    }
}
