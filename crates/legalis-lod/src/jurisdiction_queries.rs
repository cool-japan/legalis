//! Jurisdiction geometry query support for legal knowledge graphs.
//!
//! This module provides advanced query capabilities for jurisdiction geometries,
//! including spatial queries, hierarchical queries, and relationship detection.

use std::collections::HashMap;

/// Query results for jurisdiction queries
#[derive(Debug, Clone)]
pub struct JurisdictionQueryResult {
    /// Jurisdiction URI
    pub uri: String,
    /// Jurisdiction name
    pub name: String,
    /// Jurisdiction type
    pub jurisdiction_type: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Jurisdiction query builder for complex spatial queries
#[derive(Debug, Clone)]
pub struct JurisdictionQuery {
    /// Base namespace
    #[allow(dead_code)]
    base_ns: String,
    /// SPARQL query parts
    parts: QueryParts,
}

#[derive(Debug, Clone)]
struct QueryParts {
    prefixes: Vec<(String, String)>,
    select: Vec<String>,
    where_clauses: Vec<String>,
    filter: Option<String>,
    order_by: Option<String>,
    limit: Option<usize>,
}

impl JurisdictionQuery {
    /// Creates a new jurisdiction query builder
    pub fn new(base_ns: impl Into<String>) -> Self {
        let mut parts = QueryParts {
            prefixes: Vec::new(),
            select: Vec::new(),
            where_clauses: Vec::new(),
            filter: None,
            order_by: None,
            limit: None,
        };

        // Add standard prefixes
        parts.prefixes.push((
            "geo".to_string(),
            "http://www.opengis.net/ont/geosparql#".to_string(),
        ));
        parts.prefixes.push((
            "sf".to_string(),
            "http://www.opengis.net/ont/sf#".to_string(),
        ));
        parts.prefixes.push((
            "geof".to_string(),
            "http://www.opengis.net/def/function/geosparql/".to_string(),
        ));
        parts.prefixes.push((
            "eli".to_string(),
            "http://data.europa.eu/eli/ontology#".to_string(),
        ));
        parts.prefixes.push((
            "legalis".to_string(),
            "https://legalis.dev/ontology#".to_string(),
        ));
        parts.prefixes.push((
            "rdfs".to_string(),
            "http://www.w3.org/2000/01/rdf-schema#".to_string(),
        ));

        Self {
            base_ns: base_ns.into(),
            parts,
        }
    }

    /// Finds all jurisdictions
    pub fn find_all() -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query.parts.select.push("?type".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction legalis:jurisdictionType ?type".to_string());
        query.build()
    }

    /// Finds jurisdictions by type (e.g., "country", "state", "city")
    pub fn find_by_type(jurisdiction_type: &str) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query.parts.where_clauses.push(format!(
            "?jurisdiction legalis:jurisdictionType \"{}\"",
            jurisdiction_type
        ));
        query.build()
    }

    /// Finds jurisdictions containing a specific point
    pub fn containing_point(lon: f64, lat: f64) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction geo:hasGeometry ?geom".to_string());
        query
            .parts
            .where_clauses
            .push("?geom geo:asWKT ?wkt".to_string());
        query.parts.filter = Some(format!(
            "geof:sfContains(?wkt, \"POINT ({} {})\"^^geo:wktLiteral)",
            lon, lat
        ));
        query.build()
    }

    /// Finds jurisdictions within another jurisdiction
    pub fn within_jurisdiction(parent_uri: &str) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push(format!("?jurisdiction geo:sfWithin <{}>", parent_uri));
        query.build()
    }

    /// Finds neighboring jurisdictions (those that touch or share a border)
    pub fn neighboring_jurisdictions(jurisdiction_uri: &str) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?neighbor".to_string());
        query.parts.select.push("?name".to_string());
        query
            .parts
            .where_clauses
            .push(format!("<{}>", "geo:hasGeometry ?geom1"));
        query
            .parts
            .where_clauses
            .push("?neighbor a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?neighbor rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push("?neighbor geo:hasGeometry ?geom2".to_string());
        query
            .parts
            .where_clauses
            .push(format!("FILTER (<{}> != ?neighbor)", jurisdiction_uri));
        query.parts.where_clauses.push(
            "FILTER (geof:sfTouches(?geom1, ?geom2) || geof:sfOverlaps(?geom1, ?geom2))"
                .to_string(),
        );
        query.build()
    }

    /// Finds jurisdictions intersecting a bounding box
    pub fn in_bounding_box(min_lon: f64, min_lat: f64, max_lon: f64, max_lat: f64) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction geo:hasGeometry ?geom".to_string());
        query
            .parts
            .where_clauses
            .push("?geom geo:asWKT ?wkt".to_string());

        let bbox_wkt = format!(
            "POLYGON (({} {}, {} {}, {} {}, {} {}, {} {}))",
            min_lon,
            min_lat,
            max_lon,
            min_lat,
            max_lon,
            max_lat,
            min_lon,
            max_lat,
            min_lon,
            min_lat
        );
        query.parts.filter = Some(format!(
            "geof:sfIntersects(?wkt, \"{}\"^^geo:wktLiteral)",
            bbox_wkt
        ));
        query.build()
    }

    /// Finds jurisdiction hierarchy (parent-child relationships)
    pub fn jurisdiction_hierarchy() -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query.parts.select.push("?parent".to_string());
        query.parts.select.push("?parentName".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query.parts.where_clauses.push(
            "OPTIONAL { ?jurisdiction geo:sfWithin ?parent . ?parent rdfs:label ?parentName }"
                .to_string(),
        );
        query.parts.order_by = Some("?parent ?name".to_string());
        query.build()
    }

    /// Finds jurisdictions by area size
    pub fn by_area_size(min_area: Option<f64>, max_area: Option<f64>) -> String {
        let mut query = Self::new("https://example.org/");
        query.parts.select.push("?jurisdiction".to_string());
        query.parts.select.push("?name".to_string());
        query.parts.select.push("?area".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction a eli:Jurisdiction".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction rdfs:label ?name".to_string());
        query
            .parts
            .where_clauses
            .push("?jurisdiction geo:hasGeometry ?geom".to_string());
        query
            .parts
            .where_clauses
            .push("BIND(geof:area(?geom) AS ?area)".to_string());

        let mut filters = Vec::new();
        if let Some(min) = min_area {
            filters.push(format!("?area >= {}", min));
        }
        if let Some(max) = max_area {
            filters.push(format!("?area <= {}", max));
        }
        if !filters.is_empty() {
            query.parts.filter = Some(filters.join(" && "));
        }

        query.parts.order_by = Some("DESC(?area)".to_string());
        query.build()
    }

    /// Builds the SPARQL query
    fn build(&self) -> String {
        let mut query = String::new();

        // Prefixes
        for (prefix, uri) in &self.parts.prefixes {
            query.push_str(&format!("PREFIX {}: <{}>\n", prefix, uri));
        }
        query.push('\n');

        // SELECT
        query.push_str("SELECT ");
        if self.parts.select.is_empty() {
            query.push('*');
        } else {
            query.push_str(&self.parts.select.join(" "));
        }
        query.push_str("\nWHERE {\n");

        // WHERE clauses
        for clause in &self.parts.where_clauses {
            if clause.starts_with("FILTER")
                || clause.starts_with("OPTIONAL")
                || clause.starts_with("BIND")
            {
                query.push_str("  ");
                query.push_str(clause);
                query.push('\n');
            } else {
                query.push_str("  ");
                query.push_str(clause);
                if !clause.trim().ends_with('.') && !clause.trim().ends_with('}') {
                    query.push_str(" .");
                }
                query.push('\n');
            }
        }

        // FILTER
        if let Some(ref filter) = self.parts.filter {
            query.push_str("  FILTER (");
            query.push_str(filter);
            query.push_str(")\n");
        }

        query.push_str("}\n");

        // ORDER BY
        if let Some(ref order_by) = self.parts.order_by {
            query.push_str("ORDER BY ");
            query.push_str(order_by);
            query.push('\n');
        }

        // LIMIT
        if let Some(limit) = self.parts.limit {
            query.push_str(&format!("LIMIT {}\n", limit));
        }

        query
    }
}

/// Jurisdiction distance calculation
#[derive(Debug, Clone)]
pub struct JurisdictionDistance {
    /// First jurisdiction URI
    pub from: String,
    /// Second jurisdiction URI
    pub to: String,
    /// Distance in meters (approximate)
    pub distance_m: f64,
}

impl JurisdictionDistance {
    /// Creates a SPARQL query to find distances between jurisdictions
    pub fn distance_query(from_uri: &str, to_uri: &str) -> String {
        format!(
            r#"PREFIX geo: <http://www.opengis.net/ont/geosparql#>
PREFIX geof: <http://www.opengis.net/def/function/geosparql/>

SELECT ?distance
WHERE {{
  <{}> geo:hasGeometry ?geom1 .
  <{}> geo:hasGeometry ?geom2 .
  ?geom1 geo:asWKT ?wkt1 .
  ?geom2 geo:asWKT ?wkt2 .
  BIND(geof:distance(?wkt1, ?wkt2) AS ?distance)
}}
"#,
            from_uri, to_uri
        )
    }

    /// Creates a query to find nearest jurisdictions to a point
    pub fn nearest_jurisdictions(lon: f64, lat: f64, limit: usize) -> String {
        format!(
            r#"PREFIX geo: <http://www.opengis.net/ont/geosparql#>
PREFIX geof: <http://www.opengis.net/def/function/geosparql/>
PREFIX eli: <http://data.europa.eu/eli/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?jurisdiction ?name ?distance
WHERE {{
  ?jurisdiction a eli:Jurisdiction .
  ?jurisdiction rdfs:label ?name .
  ?jurisdiction geo:hasGeometry ?geom .
  ?geom geo:asWKT ?wkt .
  BIND(geof:distance(?wkt, "POINT ({} {})"^^geo:wktLiteral) AS ?distance)
}}
ORDER BY ?distance
LIMIT {}
"#,
            lon, lat, limit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_jurisdictions() {
        let query = JurisdictionQuery::find_all();
        assert!(query.contains("SELECT"));
        assert!(query.contains("eli:Jurisdiction"));
        assert!(query.contains("rdfs:label"));
    }

    #[test]
    fn test_find_by_type() {
        let query = JurisdictionQuery::find_by_type("city");
        assert!(query.contains("jurisdictionType \"city\""));
    }

    #[test]
    fn test_containing_point() {
        let query = JurisdictionQuery::containing_point(139.6917, 35.6895);
        assert!(query.contains("geof:sfContains"));
        assert!(query.contains("POINT (139.6917 35.6895)"));
    }

    #[test]
    fn test_within_jurisdiction() {
        let query = JurisdictionQuery::within_jurisdiction("https://example.org/japan");
        assert!(query.contains("geo:sfWithin"));
        assert!(query.contains("https://example.org/japan"));
    }

    #[test]
    fn test_bounding_box_query() {
        let query = JurisdictionQuery::in_bounding_box(139.0, 35.0, 140.0, 36.0);
        assert!(query.contains("geof:sfIntersects"));
        assert!(query.contains("POLYGON"));
    }

    #[test]
    fn test_jurisdiction_hierarchy() {
        let query = JurisdictionQuery::jurisdiction_hierarchy();
        assert!(query.contains("OPTIONAL"));
        assert!(query.contains("geo:sfWithin"));
        assert!(query.contains("ORDER BY"));
    }

    #[test]
    fn test_distance_query() {
        let query = JurisdictionDistance::distance_query(
            "https://example.org/tokyo",
            "https://example.org/osaka",
        );
        assert!(query.contains("geof:distance"));
        assert!(query.contains("tokyo"));
        assert!(query.contains("osaka"));
    }

    #[test]
    fn test_nearest_jurisdictions() {
        let query = JurisdictionDistance::nearest_jurisdictions(139.6917, 35.6895, 5);
        assert!(query.contains("ORDER BY ?distance"));
        assert!(query.contains("LIMIT 5"));
    }
}
