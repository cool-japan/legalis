//! Time-aware SPARQL queries for temporal legal knowledge graphs.
//!
//! This module provides query builders and utilities for querying temporal
//! RDF data, including:
//! - Queries at specific points in time
//! - Queries over time ranges
//! - Historical queries
//! - Change detection queries

use crate::temporal_rdf::{TEMPORAL_NS, TIME_NS, TimePeriod};
use chrono::{DateTime, Utc};

/// Time-aware SPARQL query builder
#[derive(Debug, Clone)]
pub struct TimeAwareQuery {
    prefixes: Vec<(String, String)>,
    select: Vec<String>,
    where_clauses: Vec<String>,
    filter: Option<String>,
    order_by: Option<String>,
    limit: Option<usize>,
}

impl Default for TimeAwareQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeAwareQuery {
    /// Creates a new time-aware query builder
    pub fn new() -> Self {
        let mut query = Self {
            prefixes: Vec::new(),
            select: Vec::new(),
            where_clauses: Vec::new(),
            filter: None,
            order_by: None,
            limit: None,
        };

        // Add standard prefixes
        query.add_prefix("temporal", TEMPORAL_NS);
        query.add_prefix("time", TIME_NS);
        query.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#");

        query
    }

    /// Adds a prefix
    pub fn add_prefix(&mut self, prefix: &str, uri: &str) {
        self.prefixes.push((prefix.to_string(), uri.to_string()));
    }

    /// Adds a SELECT variable
    pub fn select(&mut self, var: &str) -> &mut Self {
        self.select.push(var.to_string());
        self
    }

    /// Adds a WHERE clause
    pub fn where_clause(&mut self, clause: &str) -> &mut Self {
        self.where_clauses.push(clause.to_string());
        self
    }

    /// Sets ORDER BY
    pub fn order_by(&mut self, order: &str) -> &mut Self {
        self.order_by = Some(order.to_string());
        self
    }

    /// Sets LIMIT
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Queries triples valid at a specific time
    pub fn valid_at(time: DateTime<Utc>) -> String {
        let time_str = time.to_rfc3339();
        format!(
            r#"PREFIX temporal: <{}>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?subject ?predicate ?object
WHERE {{
  ?assertion a temporal:TemporalAssertion .
  ?assertion temporal:assertedTriple ?triple .
  ?assertion temporal:validFrom ?validFrom .
  OPTIONAL {{ ?assertion temporal:validTo ?validTo }}

  FILTER (?validFrom <= "{}"^^xsd:dateTime)
  FILTER (!bound(?validTo) || ?validTo > "{}"^^xsd:dateTime)

  # Extract triple components (simplified)
  BIND (str(?triple) AS ?tripleStr)
}}
"#,
            TEMPORAL_NS, time_str, time_str
        )
    }

    /// Queries changes that occurred during a time period
    pub fn changes_during(period: &TimePeriod) -> String {
        let start_str = period.start.to_rfc3339();
        let end_str = period
            .end
            .map(|e| e.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339());

        format!(
            r#"PREFIX temporal: <{}>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?subject ?predicate ?object ?changeTime ?changeType
WHERE {{
  ?assertion a temporal:TemporalAssertion .
  ?assertion temporal:assertedTriple ?triple .
  ?assertion temporal:transactionFrom ?changeTime .

  FILTER (?changeTime >= "{}"^^xsd:dateTime && ?changeTime < "{}"^^xsd:dateTime)

  BIND (IF(bound(?object), "addition", "deletion") AS ?changeType)
}}
ORDER BY ?changeTime
"#,
            TEMPORAL_NS, start_str, end_str
        )
    }

    /// Queries the history of a specific subject
    pub fn history_of(subject_uri: &str) -> String {
        format!(
            r#"PREFIX temporal: <{}>

SELECT ?predicate ?object ?validFrom ?validTo ?transactionFrom
WHERE {{
  ?assertion a temporal:TemporalAssertion .
  ?assertion temporal:assertedTriple ?triple .
  ?assertion temporal:validFrom ?validFrom .
  OPTIONAL {{ ?assertion temporal:validTo ?validTo }}
  OPTIONAL {{ ?assertion temporal:transactionFrom ?transactionFrom }}

  FILTER (strstarts(str(?triple), "<{}>"))
}}
ORDER BY DESC(?validFrom)
"#,
            TEMPORAL_NS, subject_uri
        )
    }

    /// Queries facts that were true at one time but not at another
    pub fn changed_between(time1: DateTime<Utc>, time2: DateTime<Utc>) -> String {
        let time1_str = time1.to_rfc3339();
        let time2_str = time2.to_rfc3339();

        format!(
            r#"PREFIX temporal: <{}>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?subject ?predicate ?object ?changeType
WHERE {{
  # Find assertions valid at time1 but not time2
  {{
    ?assertion1 a temporal:TemporalAssertion .
    ?assertion1 temporal:assertedTriple ?triple .
    ?assertion1 temporal:validFrom ?vf1 .
    OPTIONAL {{ ?assertion1 temporal:validTo ?vt1 }}

    FILTER (?vf1 <= "{}"^^xsd:dateTime)
    FILTER (!bound(?vt1) || ?vt1 > "{}"^^xsd:dateTime)

    FILTER NOT EXISTS {{
      ?assertion2 a temporal:TemporalAssertion .
      ?assertion2 temporal:assertedTriple ?triple .
      ?assertion2 temporal:validFrom ?vf2 .
      OPTIONAL {{ ?assertion2 temporal:validTo ?vt2 }}

      FILTER (?vf2 <= "{}"^^xsd:dateTime)
      FILTER (!bound(?vt2) || ?vt2 > "{}"^^xsd:dateTime)
    }}

    BIND ("removed" AS ?changeType)
  }}
  UNION
  {{
    # Find assertions valid at time2 but not time1
    ?assertion3 a temporal:TemporalAssertion .
    ?assertion3 temporal:assertedTriple ?triple .
    ?assertion3 temporal:validFrom ?vf3 .
    OPTIONAL {{ ?assertion3 temporal:validTo ?vt3 }}

    FILTER (?vf3 <= "{}"^^xsd:dateTime)
    FILTER (!bound(?vt3) || ?vt3 > "{}"^^xsd:dateTime)

    FILTER NOT EXISTS {{
      ?assertion4 a temporal:TemporalAssertion .
      ?assertion4 temporal:assertedTriple ?triple .
      ?assertion4 temporal:validFrom ?vf4 .
      OPTIONAL {{ ?assertion4 temporal:validTo ?vt4 }}

      FILTER (?vf4 <= "{}"^^xsd:dateTime)
      FILTER (!bound(?vt4) || ?vt4 > "{}"^^xsd:dateTime)
    }}

    BIND ("added" AS ?changeType)
  }}
}}
"#,
            TEMPORAL_NS,
            time1_str,
            time1_str,
            time2_str,
            time2_str,
            time2_str,
            time2_str,
            time1_str,
            time1_str
        )
    }

    /// Queries overlapping valid time periods
    pub fn overlapping_validities(subject_uri: &str) -> String {
        format!(
            r#"PREFIX temporal: <{}>

SELECT ?assertion1 ?assertion2 ?vf1 ?vt1 ?vf2 ?vt2
WHERE {{
  ?assertion1 a temporal:TemporalAssertion .
  ?assertion1 temporal:assertedTriple ?triple1 .
  ?assertion1 temporal:validFrom ?vf1 .
  ?assertion1 temporal:validTo ?vt1 .

  ?assertion2 a temporal:TemporalAssertion .
  ?assertion2 temporal:assertedTriple ?triple2 .
  ?assertion2 temporal:validFrom ?vf2 .
  ?assertion2 temporal:validTo ?vt2 .

  FILTER (?assertion1 != ?assertion2)
  FILTER (strstarts(str(?triple1), "<{}>"))
  FILTER (strstarts(str(?triple2), "<{}>"))
  FILTER (?vf1 < ?vt2 && ?vf2 < ?vt1)
}}
"#,
            TEMPORAL_NS, subject_uri, subject_uri
        )
    }

    /// Queries the current (most recent) state
    pub fn current_state() -> String {
        format!(
            r#"PREFIX temporal: <{}>

SELECT ?subject ?predicate ?object ?validFrom
WHERE {{
  ?assertion a temporal:TemporalAssertion .
  ?assertion temporal:assertedTriple ?triple .
  ?assertion temporal:validFrom ?validFrom .
  FILTER NOT EXISTS {{ ?assertion temporal:validTo ?validTo }}

  # Extract subject from triple (simplified representation)
  BIND (str(?triple) AS ?tripleStr)
}}
ORDER BY DESC(?validFrom)
"#,
            TEMPORAL_NS
        )
    }

    /// Queries for statute versions over time
    pub fn statute_versions(statute_id: &str) -> String {
        format!(
            r#"PREFIX temporal: <{}>
PREFIX eli: <http://data.europa.eu/eli/ontology#>

SELECT ?version ?validFrom ?validTo ?title
WHERE {{
  ?version eli:realizes <{}> .
  ?assertion a temporal:TemporalAssertion .
  ?assertion temporal:assertedTriple ?triple .
  ?assertion temporal:validFrom ?validFrom .
  OPTIONAL {{ ?assertion temporal:validTo ?validTo }}

  ?version dcterms:title ?title .

  FILTER (contains(str(?triple), str(?version)))
}}
ORDER BY ?validFrom
"#,
            TEMPORAL_NS, statute_id
        )
    }

    /// Builds the SPARQL query string
    pub fn build(&self) -> String {
        let mut query = String::new();

        // Prefixes
        for (prefix, uri) in &self.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        query.push('\n');

        // SELECT
        query.push_str("SELECT ");
        if self.select.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.select.join(" "));
        }
        query.push_str("\nWHERE {\n");

        // WHERE clauses
        for clause in &self.where_clauses {
            query.push_str("  ");
            query.push_str(clause);
            if !clause.trim().ends_with('.') && !clause.trim().ends_with('}') {
                query.push_str(" .");
            }
            query.push('\n');
        }

        // FILTER
        if let Some(ref filter) = self.filter {
            query.push_str("  FILTER (");
            query.push_str(filter);
            query.push_str(")\n");
        }

        query.push_str("}\n");

        // ORDER BY
        if let Some(ref order_by) = self.order_by {
            query.push_str("ORDER BY ");
            query.push_str(order_by);
            query.push('\n');
        }

        // LIMIT
        if let Some(limit) = self.limit {
            query.push_str(&format!("LIMIT {}\n", limit));
        }

        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_at_query() {
        let time = Utc::now();
        let query = TimeAwareQuery::valid_at(time);

        assert!(query.contains("PREFIX temporal:"));
        assert!(query.contains("temporal:TemporalAssertion"));
        assert!(query.contains("temporal:validFrom"));
        assert!(query.contains("temporal:validTo"));
    }

    #[test]
    fn test_changes_during_query() {
        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);
        let period = TimePeriod::new(start, Some(end));

        let query = TimeAwareQuery::changes_during(&period);

        assert!(query.contains("temporal:transactionFrom"));
        assert!(query.contains("FILTER"));
        assert!(query.contains("ORDER BY"));
    }

    #[test]
    fn test_history_of_query() {
        let query = TimeAwareQuery::history_of("https://example.org/statute/123");

        assert!(query.contains("temporal:TemporalAssertion"));
        assert!(query.contains("ORDER BY DESC"));
        assert!(query.contains("https://example.org/statute/123"));
    }

    #[test]
    fn test_changed_between_query() {
        let time1 = Utc::now();
        let time2 = time1 + chrono::Duration::days(1);

        let query = TimeAwareQuery::changed_between(time1, time2);

        assert!(query.contains("UNION"));
        assert!(query.contains("?changeType"));
        assert!(query.contains("\"removed\""));
        assert!(query.contains("\"added\""));
    }

    #[test]
    fn test_overlapping_validities_query() {
        let query = TimeAwareQuery::overlapping_validities("https://example.org/law/1");

        assert!(query.contains("?assertion1"));
        assert!(query.contains("?assertion2"));
        assert!(query.contains("FILTER (?assertion1 != ?assertion2)"));
    }

    #[test]
    fn test_current_state_query() {
        let query = TimeAwareQuery::current_state();

        assert!(query.contains("FILTER NOT EXISTS"));
        assert!(query.contains("temporal:validTo"));
        assert!(query.contains("ORDER BY DESC"));
    }

    #[test]
    fn test_statute_versions_query() {
        let query = TimeAwareQuery::statute_versions("https://example.org/statute/123");

        assert!(query.contains("eli:realizes"));
        assert!(query.contains("dcterms:title"));
        assert!(query.contains("ORDER BY ?validFrom"));
    }

    #[test]
    fn test_query_builder() {
        let mut query = TimeAwareQuery::new();
        query
            .select("?s")
            .select("?p")
            .select("?o")
            .where_clause("?s ?p ?o")
            .order_by("?s")
            .limit(10);

        let sparql = query.build();

        assert!(sparql.contains("SELECT ?s ?p ?o"));
        assert!(sparql.contains("WHERE"));
        assert!(sparql.contains("ORDER BY ?s"));
        assert!(sparql.contains("LIMIT 10"));
    }
}
