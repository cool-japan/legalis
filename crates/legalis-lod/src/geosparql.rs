//! GeoSPARQL 1.1 support for legal jurisdictions and spatial reasoning.
//!
//! This module implements the GeoSPARQL 1.1 standard for representing and querying
//! geospatial information in RDF. It's particularly useful for:
//! - Jurisdiction boundaries
//! - Legal zones and areas
//! - Spatial relationships between legal entities
//! - Map-based knowledge exploration
//!
//! ## Features
//! - Geometry types (Point, LineString, Polygon, MultiPolygon)
//! - Well-Known Text (WKT) serialization
//! - Coordinate Reference Systems (CRS)
//! - Spatial relationship predicates
//! - Simple Features spatial relations (sfWithin, sfContains, etc.)
//! - Egenhofer topology relations

use crate::{RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::fmt;

/// GeoSPARQL namespace
pub const GEOSPARQL_NS: &str = "http://www.opengis.net/ont/geosparql#";

/// GeoSPARQL Simple Features namespace
pub const SF_NS: &str = "http://www.opengis.net/ont/sf#";

/// Geometry types supported by GeoSPARQL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeometryType {
    /// A single point in space
    Point,
    /// A line string (sequence of points)
    LineString,
    /// A polygon (closed line string)
    Polygon,
    /// A collection of polygons
    MultiPolygon,
    /// A generic geometry
    Geometry,
}

impl fmt::Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryType::Point => write!(f, "Point"),
            GeometryType::LineString => write!(f, "LineString"),
            GeometryType::Polygon => write!(f, "Polygon"),
            GeometryType::MultiPolygon => write!(f, "MultiPolygon"),
            GeometryType::Geometry => write!(f, "Geometry"),
        }
    }
}

/// Coordinate Reference System
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoordinateReferenceSystem {
    /// EPSG code (e.g., 4326 for WGS84, 3857 for Web Mercator)
    pub epsg_code: u32,
    /// Human-readable name
    pub name: String,
}

impl CoordinateReferenceSystem {
    /// Creates a new CRS with EPSG code
    pub fn new(epsg_code: u32, name: impl Into<String>) -> Self {
        Self {
            epsg_code,
            name: name.into(),
        }
    }

    /// WGS84 (GPS coordinates)
    pub fn wgs84() -> Self {
        Self::new(4326, "WGS 84")
    }

    /// Web Mercator (common for web maps)
    pub fn web_mercator() -> Self {
        Self::new(3857, "WGS 84 / Pseudo-Mercator")
    }

    /// Returns the CRS URI
    pub fn uri(&self) -> String {
        format!("http://www.opengis.net/def/crs/EPSG/0/{}", self.epsg_code)
    }
}

impl Default for CoordinateReferenceSystem {
    fn default() -> Self {
        Self::wgs84()
    }
}

/// A geographic feature with geometry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoFeature {
    /// Feature URI
    pub uri: String,
    /// Feature label
    pub label: Option<String>,
    /// Geometry
    pub geometry: Geometry,
    /// Additional properties
    pub properties: Vec<(String, String)>,
}

impl GeoFeature {
    /// Creates a new geographic feature
    pub fn new(uri: impl Into<String>, geometry: Geometry) -> Self {
        Self {
            uri: uri.into(),
            label: None,
            geometry,
            properties: Vec::new(),
        }
    }

    /// Sets the label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Adds a property
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }

    /// Converts to RDF triples
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Feature type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("geo:Feature".to_string()),
        });

        // Label
        if let Some(ref label) = self.label {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(label),
            });
        }

        // Geometry
        let geom_uri = format!("{}/geometry", self.uri);
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "geo:hasGeometry".to_string(),
            object: RdfValue::Uri(geom_uri.clone()),
        });

        triples.extend(self.geometry.to_triples(&geom_uri));

        // Additional properties
        for (key, value) in &self.properties {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: key.clone(),
                object: RdfValue::string(value),
            });
        }

        triples
    }
}

/// A geometry with Well-Known Text representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geometry {
    /// Geometry type
    pub geom_type: GeometryType,
    /// Well-Known Text representation
    pub wkt: String,
    /// Coordinate Reference System
    pub crs: CoordinateReferenceSystem,
}

impl Geometry {
    /// Creates a new geometry
    pub fn new(geom_type: GeometryType, wkt: impl Into<String>) -> Self {
        Self {
            geom_type,
            wkt: wkt.into(),
            crs: CoordinateReferenceSystem::default(),
        }
    }

    /// Sets the CRS
    pub fn with_crs(mut self, crs: CoordinateReferenceSystem) -> Self {
        self.crs = crs;
        self
    }

    /// Creates a Point geometry
    pub fn point(lon: f64, lat: f64) -> Self {
        Self::new(GeometryType::Point, format!("POINT ({} {})", lon, lat))
    }

    /// Creates a Polygon geometry from coordinates
    pub fn polygon(coords: &[(f64, f64)]) -> Self {
        let mut wkt = String::from("POLYGON ((");
        for (i, (lon, lat)) in coords.iter().enumerate() {
            if i > 0 {
                wkt.push_str(", ");
            }
            wkt.push_str(&format!("{} {}", lon, lat));
        }
        wkt.push_str("))");
        Self::new(GeometryType::Polygon, wkt)
    }

    /// Creates a LineString geometry
    pub fn line_string(coords: &[(f64, f64)]) -> Self {
        let mut wkt = String::from("LINESTRING (");
        for (i, (lon, lat)) in coords.iter().enumerate() {
            if i > 0 {
                wkt.push_str(", ");
            }
            wkt.push_str(&format!("{} {}", lon, lat));
        }
        wkt.push(')');
        Self::new(GeometryType::LineString, wkt)
    }

    /// Creates a MultiPolygon geometry
    pub fn multi_polygon(polygons: &[Vec<(f64, f64)>]) -> Self {
        let mut wkt = String::from("MULTIPOLYGON (");
        for (i, polygon) in polygons.iter().enumerate() {
            if i > 0 {
                wkt.push_str(", ");
            }
            wkt.push('(');
            for (j, (lon, lat)) in polygon.iter().enumerate() {
                if j > 0 {
                    wkt.push_str(", ");
                }
                wkt.push_str(&format!("{} {}", lon, lat));
            }
            wkt.push(')');
        }
        wkt.push(')');
        Self::new(GeometryType::MultiPolygon, wkt)
    }

    /// Converts to RDF triples
    pub fn to_triples(&self, uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Geometry type
        triples.push(Triple {
            subject: uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(format!("sf:{}", self.geom_type)),
        });

        // Also add general geo:Geometry type
        triples.push(Triple {
            subject: uri.to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("geo:Geometry".to_string()),
        });

        // WKT literal
        triples.push(Triple {
            subject: uri.to_string(),
            predicate: "geo:asWKT".to_string(),
            object: RdfValue::TypedLiteral(self.wkt.clone(), "geo:wktLiteral".to_string()),
        });

        // CRS
        triples.push(Triple {
            subject: uri.to_string(),
            predicate: "geo:hasSpatialReference".to_string(),
            object: RdfValue::Uri(self.crs.uri()),
        });

        triples
    }
}

/// Spatial relationship types (Simple Features)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialRelation {
    /// A is within B
    Within,
    /// A contains B
    Contains,
    /// A intersects B
    Intersects,
    /// A equals B
    Equals,
    /// A is disjoint from B
    Disjoint,
    /// A touches B
    Touches,
    /// A crosses B
    Crosses,
    /// A overlaps B
    Overlaps,
}

impl SpatialRelation {
    /// Returns the GeoSPARQL predicate URI
    pub fn predicate(&self) -> String {
        match self {
            SpatialRelation::Within => "geo:sfWithin".to_string(),
            SpatialRelation::Contains => "geo:sfContains".to_string(),
            SpatialRelation::Intersects => "geo:sfIntersects".to_string(),
            SpatialRelation::Equals => "geo:sfEquals".to_string(),
            SpatialRelation::Disjoint => "geo:sfDisjoint".to_string(),
            SpatialRelation::Touches => "geo:sfTouches".to_string(),
            SpatialRelation::Crosses => "geo:sfCrosses".to_string(),
            SpatialRelation::Overlaps => "geo:sfOverlaps".to_string(),
        }
    }
}

/// Creates a spatial relationship triple
pub fn spatial_relation_triple(
    subject: impl Into<String>,
    relation: SpatialRelation,
    object: impl Into<String>,
) -> Triple {
    Triple {
        subject: subject.into(),
        predicate: relation.predicate(),
        object: RdfValue::Uri(object.into()),
    }
}

/// Jurisdiction with geographic boundary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// Jurisdiction URI
    pub uri: String,
    /// Jurisdiction name
    pub name: String,
    /// Jurisdiction type (country, state, city, etc.)
    pub jurisdiction_type: String,
    /// Geographic boundary
    pub boundary: Geometry,
    /// Parent jurisdiction (if any)
    pub parent: Option<String>,
}

impl Jurisdiction {
    /// Creates a new jurisdiction
    pub fn new(
        uri: impl Into<String>,
        name: impl Into<String>,
        jurisdiction_type: impl Into<String>,
        boundary: Geometry,
    ) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            jurisdiction_type: jurisdiction_type.into(),
            boundary,
            parent: None,
        }
    }

    /// Sets the parent jurisdiction
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Converts to RDF triples
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type declarations
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("geo:Feature".to_string()),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("eli:Jurisdiction".to_string()),
        });

        // Name
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.name),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&self.name),
        });

        // Jurisdiction type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "legalis:jurisdictionType".to_string(),
            object: RdfValue::string(&self.jurisdiction_type),
        });

        // Boundary geometry
        let geom_uri = format!("{}/boundary", self.uri);
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "geo:hasGeometry".to_string(),
            object: RdfValue::Uri(geom_uri.clone()),
        });

        triples.extend(self.boundary.to_triples(&geom_uri));

        // Parent jurisdiction
        if let Some(ref parent) = self.parent {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "geo:sfWithin".to_string(),
                object: RdfValue::Uri(parent.clone()),
            });

            triples.push(Triple {
                subject: parent.clone(),
                predicate: "geo:sfContains".to_string(),
                object: RdfValue::Uri(self.uri.clone()),
            });
        }

        triples
    }
}

/// Legal zone with spatial extent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalZone {
    /// Zone URI
    pub uri: String,
    /// Zone name
    pub name: String,
    /// Zone type (zoning, regulation area, protected area, etc.)
    pub zone_type: String,
    /// Geographic extent
    pub extent: Geometry,
    /// Applicable jurisdiction
    pub jurisdiction: String,
    /// Legal regulations
    pub regulations: Vec<String>,
}

impl LegalZone {
    /// Creates a new legal zone
    pub fn new(
        uri: impl Into<String>,
        name: impl Into<String>,
        zone_type: impl Into<String>,
        extent: Geometry,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            zone_type: zone_type.into(),
            extent,
            jurisdiction: jurisdiction.into(),
            regulations: Vec::new(),
        }
    }

    /// Adds a regulation
    pub fn with_regulation(mut self, regulation: impl Into<String>) -> Self {
        self.regulations.push(regulation.into());
        self
    }

    /// Converts to RDF triples
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type declarations
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("geo:Feature".to_string()),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:LegalZone".to_string()),
        });

        // Name
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.name),
        });

        // Zone type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "legalis:zoneType".to_string(),
            object: RdfValue::string(&self.zone_type),
        });

        // Extent geometry
        let geom_uri = format!("{}/extent", self.uri);
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "geo:hasGeometry".to_string(),
            object: RdfValue::Uri(geom_uri.clone()),
        });

        triples.extend(self.extent.to_triples(&geom_uri));

        // Jurisdiction
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "eli:jurisdiction".to_string(),
            object: RdfValue::Uri(self.jurisdiction.clone()),
        });

        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "geo:sfWithin".to_string(),
            object: RdfValue::Uri(self.jurisdiction.clone()),
        });

        // Regulations
        for regulation in &self.regulations {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "legalis:regulatedBy".to_string(),
                object: RdfValue::Uri(regulation.clone()),
            });
        }

        triples
    }
}

/// GeoSPARQL query builder for spatial queries
#[derive(Debug, Clone)]
pub struct GeoSparqlQuery {
    prefixes: Vec<(String, String)>,
    select: Vec<String>,
    where_clauses: Vec<String>,
    filter: Option<String>,
}

impl Default for GeoSparqlQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoSparqlQuery {
    /// Creates a new GeoSPARQL query builder
    pub fn new() -> Self {
        let mut query = Self {
            prefixes: Vec::new(),
            select: Vec::new(),
            where_clauses: Vec::new(),
            filter: None,
        };

        // Add default prefixes
        query.add_prefix("geo", GEOSPARQL_NS);
        query.add_prefix("sf", SF_NS);
        query.add_prefix("geof", "http://www.opengis.net/def/function/geosparql/");
        query.add_prefix("eli", "http://data.europa.eu/eli/ontology#");
        query.add_prefix("legalis", "https://legalis.dev/ontology#");

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

    /// Adds a spatial filter
    pub fn spatial_filter(&mut self, filter: &str) -> &mut Self {
        self.filter = Some(filter.to_string());
        self
    }

    /// Finds features within a jurisdiction
    pub fn features_within_jurisdiction(jurisdiction_uri: &str) -> Self {
        let mut query = Self::new();
        query.select("?feature");
        query.select("?label");
        query.where_clause(&format!("?feature geo:sfWithin <{}>", jurisdiction_uri));
        query.where_clause("?feature rdfs:label ?label");
        query
    }

    /// Finds jurisdictions containing a point
    pub fn jurisdictions_containing_point(lon: f64, lat: f64) -> Self {
        let mut query = Self::new();
        query.select("?jurisdiction");
        query.select("?name");
        query.where_clause("?jurisdiction a eli:Jurisdiction");
        query.where_clause("?jurisdiction geo:hasGeometry ?geom");
        query.where_clause("?jurisdiction rdfs:label ?name");
        query.spatial_filter(&format!(
            "geof:sfContains(?geom, \"POINT ({} {})\"^^geo:wktLiteral)",
            lon, lat
        ));
        query
    }

    /// Finds legal zones intersecting a geometry
    pub fn zones_intersecting_geometry(wkt: &str) -> Self {
        let mut query = Self::new();
        query.select("?zone");
        query.select("?name");
        query.where_clause("?zone a legalis:LegalZone");
        query.where_clause("?zone geo:hasGeometry ?geom");
        query.where_clause("?zone rdfs:label ?name");
        query.spatial_filter(&format!(
            "geof:sfIntersects(?geom, \"{}\"^^geo:wktLiteral)",
            wkt
        ));
        query
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
            if !clause.trim().ends_with('.') {
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
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_point() {
        let point = Geometry::point(139.6917, 35.6895); // Tokyo
        assert_eq!(point.geom_type, GeometryType::Point);
        assert_eq!(point.wkt, "POINT (139.6917 35.6895)");
    }

    #[test]
    fn test_geometry_polygon() {
        let coords = vec![
            (139.0, 35.0),
            (140.0, 35.0),
            (140.0, 36.0),
            (139.0, 36.0),
            (139.0, 35.0),
        ];
        let polygon = Geometry::polygon(&coords);
        assert_eq!(polygon.geom_type, GeometryType::Polygon);
        assert!(polygon.wkt.contains("POLYGON"));
    }

    #[test]
    fn test_crs_wgs84() {
        let crs = CoordinateReferenceSystem::wgs84();
        assert_eq!(crs.epsg_code, 4326);
        assert_eq!(crs.uri(), "http://www.opengis.net/def/crs/EPSG/0/4326");
    }

    #[test]
    fn test_jurisdiction_triples() {
        let boundary = Geometry::polygon(&vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (0.0, 0.0),
        ]);

        let jurisdiction = Jurisdiction::new(
            "https://example.org/jurisdiction/tokyo",
            "Tokyo",
            "city",
            boundary,
        );

        let triples = jurisdiction.to_triples();

        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "eli:Jurisdiction")));
        assert!(triples.iter().any(|t| t.predicate == "rdfs:label"
            && matches!(&t.object, RdfValue::Literal(s, None) if s == "Tokyo")));
        assert!(triples.iter().any(|t| t.predicate == "geo:hasGeometry"));
    }

    #[test]
    fn test_legal_zone_triples() {
        let extent = Geometry::polygon(&vec![
            (0.0, 0.0),
            (0.5, 0.0),
            (0.5, 0.5),
            (0.0, 0.5),
            (0.0, 0.0),
        ]);

        let zone = LegalZone::new(
            "https://example.org/zone/park",
            "Central Park Protected Area",
            "protected-area",
            extent,
            "https://example.org/jurisdiction/tokyo",
        )
        .with_regulation("https://example.org/statute/park-protection-act");

        let triples = zone.to_triples();

        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "legalis:LegalZone")));
        assert!(triples.iter().any(|t| t.predicate == "legalis:zoneType"));
        assert!(triples.iter().any(|t| t.predicate == "geo:sfWithin"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:regulatedBy"));
    }

    #[test]
    fn test_spatial_relation_triple() {
        let triple = spatial_relation_triple(
            "https://example.org/feature/a",
            SpatialRelation::Within,
            "https://example.org/feature/b",
        );

        assert_eq!(triple.predicate, "geo:sfWithin");
    }

    #[test]
    fn test_geosparql_query_features_within() {
        let query =
            GeoSparqlQuery::features_within_jurisdiction("https://example.org/jurisdiction/tokyo");
        let sparql = query.build();

        assert!(sparql.contains("PREFIX geo:"));
        assert!(sparql.contains("SELECT ?feature ?label"));
        assert!(sparql.contains("geo:sfWithin"));
    }

    #[test]
    fn test_geosparql_query_point_in_jurisdiction() {
        let query = GeoSparqlQuery::jurisdictions_containing_point(139.6917, 35.6895);
        let sparql = query.build();

        assert!(sparql.contains("PREFIX geo:"));
        assert!(sparql.contains("eli:Jurisdiction"));
        assert!(sparql.contains("geof:sfContains"));
        assert!(sparql.contains("POINT (139.6917 35.6895)"));
    }

    #[test]
    fn test_geo_feature() {
        let geom = Geometry::point(139.6917, 35.6895);
        let feature = GeoFeature::new("https://example.org/feature/tokyo-tower", geom)
            .with_label("Tokyo Tower")
            .with_property("height", "333m");

        let triples = feature.to_triples();

        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "geo:Feature")));
        assert!(triples.iter().any(|t| t.predicate == "geo:hasGeometry"));
    }
}
