//! Spatial reasoning for legal zones and geographic constraints.
//!
//! This module provides spatial reasoning capabilities for legal knowledge graphs,
//! including:
//! - Zone containment and overlap detection
//! - Regulatory applicability based on location
//! - Spatial conflict detection
//! - Zone hierarchy inference

use crate::geosparql::{LegalZone, SpatialRelation};
use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Spatial reasoning engine for legal zones
#[derive(Debug, Clone)]
pub struct SpatialReasoner {
    /// Base namespace for URIs
    base_ns: String,
    /// Legal zones indexed by URI
    zones: HashMap<String, LegalZone>,
    /// Cached spatial relationships
    relationships: Vec<SpatialRelationship>,
}

/// A spatial relationship between two features
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialRelationship {
    /// Subject feature URI
    pub subject: String,
    /// Spatial relation type
    pub relation: SpatialRelation,
    /// Object feature URI
    pub object: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl SpatialReasoner {
    /// Creates a new spatial reasoner
    pub fn new(base_ns: impl Into<String>) -> Self {
        Self {
            base_ns: base_ns.into(),
            zones: HashMap::new(),
            relationships: Vec::new(),
        }
    }

    /// Adds a legal zone to the reasoner
    pub fn add_zone(&mut self, zone: LegalZone) {
        self.zones.insert(zone.uri.clone(), zone);
    }

    /// Finds all zones applicable at a specific location
    pub fn zones_at_location(&self, _lon: f64, _lat: f64) -> Vec<&LegalZone> {
        // In a real implementation, this would use actual geometric calculations
        // For now, we return zones that might contain the point based on simple checks
        self.zones.values().collect()
    }

    /// Infers spatial relationships between zones
    pub fn infer_relationships(&mut self) {
        let zone_list: Vec<_> = self.zones.values().collect();

        for (i, zone_a) in zone_list.iter().enumerate() {
            for zone_b in zone_list.iter().skip(i + 1) {
                // In a real implementation, we would calculate actual spatial relationships
                // For now, we create placeholder relationships

                // Check if zone_a might be within zone_b based on jurisdiction
                if zone_a.jurisdiction == zone_b.uri {
                    self.relationships.push(SpatialRelationship {
                        subject: zone_a.uri.clone(),
                        relation: SpatialRelation::Within,
                        object: zone_b.uri.clone(),
                        confidence: 0.9,
                    });
                }
            }
        }
    }

    /// Finds zones that overlap with a given zone
    pub fn find_overlapping_zones(&self, zone_uri: &str) -> Vec<&LegalZone> {
        self.relationships
            .iter()
            .filter(|r| {
                (r.subject == zone_uri || r.object == zone_uri)
                    && r.relation == SpatialRelation::Overlaps
            })
            .filter_map(|r| {
                let other_uri = if r.subject == zone_uri {
                    &r.object
                } else {
                    &r.subject
                };
                self.zones.get(other_uri)
            })
            .collect()
    }

    /// Finds zones contained within a given zone
    pub fn find_contained_zones(&self, zone_uri: &str) -> Vec<&LegalZone> {
        self.relationships
            .iter()
            .filter(|r| r.object == zone_uri && r.relation == SpatialRelation::Within)
            .filter_map(|r| self.zones.get(&r.subject))
            .collect()
    }

    /// Detects spatial conflicts (overlapping zones with conflicting regulations)
    pub fn detect_conflicts(&self) -> Vec<ZoneConflict> {
        let mut conflicts = Vec::new();

        for rel in &self.relationships {
            if rel.relation == SpatialRelation::Overlaps
                && let (Some(zone_a), Some(zone_b)) =
                    (self.zones.get(&rel.subject), self.zones.get(&rel.object))
            {
                // Check if zones have potentially conflicting regulations
                if !zone_a.regulations.is_empty() && !zone_b.regulations.is_empty() {
                    // Simple conflict detection: if zones overlap and have different regulations
                    let common_regs: Vec<_> = zone_a
                        .regulations
                        .iter()
                        .filter(|r| zone_b.regulations.contains(r))
                        .collect();

                    if common_regs.is_empty() && zone_a.zone_type != zone_b.zone_type {
                        conflicts.push(ZoneConflict {
                            zone_a: zone_a.uri.clone(),
                            zone_b: zone_b.uri.clone(),
                            conflict_type: ConflictType::OverlappingRegulations,
                            description: format!(
                                "Zones '{}' and '{}' overlap with different regulations",
                                zone_a.name, zone_b.name
                            ),
                        });
                    }
                }
            }
        }

        conflicts
    }

    /// Generates RDF triples for inferred relationships
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        for rel in &self.relationships {
            triples.push(Triple {
                subject: rel.subject.clone(),
                predicate: rel.relation.predicate(),
                object: RdfValue::Uri(rel.object.clone()),
            });

            // Add confidence as a reified statement if needed
            if rel.confidence < 1.0 {
                let stmt_uri = format!("{}statement/{}", self.base_ns, triples.len());
                triples.push(Triple {
                    subject: stmt_uri.clone(),
                    predicate: "rdf:type".to_string(),
                    object: RdfValue::Uri("rdf:Statement".to_string()),
                });
                triples.push(Triple {
                    subject: stmt_uri.clone(),
                    predicate: "rdf:subject".to_string(),
                    object: RdfValue::Uri(rel.subject.clone()),
                });
                triples.push(Triple {
                    subject: stmt_uri.clone(),
                    predicate: "rdf:predicate".to_string(),
                    object: RdfValue::Uri(rel.relation.predicate()),
                });
                triples.push(Triple {
                    subject: stmt_uri.clone(),
                    predicate: "rdf:object".to_string(),
                    object: RdfValue::Uri(rel.object.clone()),
                });
                triples.push(Triple {
                    subject: stmt_uri,
                    predicate: "legalis:confidence".to_string(),
                    object: RdfValue::TypedLiteral(
                        rel.confidence.to_string(),
                        "xsd:decimal".to_string(),
                    ),
                });
            }
        }

        triples
    }
}

/// A spatial conflict between legal zones
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneConflict {
    /// First zone URI
    pub zone_a: String,
    /// Second zone URI
    pub zone_b: String,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Human-readable description
    pub description: String,
}

/// Types of spatial conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictType {
    /// Overlapping zones with different regulations
    OverlappingRegulations,
    /// Contained zone with conflicting regulation
    ConflictingContainment,
    /// Gap in coverage (no applicable zone)
    CoverageGap,
}

/// Regulatory applicability checker
#[derive(Debug, Clone)]
pub struct RegulationApplicability {
    /// Base namespace
    #[allow(dead_code)]
    base_ns: String,
}

impl RegulationApplicability {
    /// Creates a new applicability checker
    pub fn new(base_ns: impl Into<String>) -> Self {
        Self {
            base_ns: base_ns.into(),
        }
    }

    /// Generates a SPARQL query to find applicable regulations at a location
    pub fn applicable_at_point(lon: f64, lat: f64) -> String {
        format!(
            r#"PREFIX geo: <http://www.opengis.net/ont/geosparql#>
PREFIX geof: <http://www.opengis.net/def/function/geosparql/>
PREFIX legalis: <https://legalis.dev/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?zone ?zoneName ?regulation ?regulationTitle
WHERE {{
  ?zone a legalis:LegalZone .
  ?zone rdfs:label ?zoneName .
  ?zone geo:hasGeometry ?geom .
  ?geom geo:asWKT ?wkt .
  ?zone legalis:regulatedBy ?regulation .
  ?regulation dcterms:title ?regulationTitle .
  FILTER (geof:sfContains(?wkt, "POINT ({} {})"^^geo:wktLiteral))
}}
"#,
            lon, lat
        )
    }

    /// Generates a query to find all regulations applicable in a jurisdiction
    pub fn regulations_in_jurisdiction(jurisdiction_uri: &str) -> String {
        format!(
            r#"PREFIX geo: <http://www.opengis.net/ont/geosparql#>
PREFIX legalis: <https://legalis.dev/ontology#>
PREFIX eli: <http://data.europa.eu/eli/ontology#>
PREFIX dcterms: <http://purl.org/dc/terms/>

SELECT DISTINCT ?regulation ?title
WHERE {{
  ?zone a legalis:LegalZone .
  ?zone eli:jurisdiction <{}> .
  ?zone legalis:regulatedBy ?regulation .
  ?regulation dcterms:title ?title .
}}
"#,
            jurisdiction_uri
        )
    }

    /// Generates a query to find zones where multiple regulations apply
    pub fn overlapping_regulations() -> String {
        r#"PREFIX geo: <http://www.opengis.net/ont/geosparql#>
PREFIX legalis: <https://legalis.dev/ontology#>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

SELECT ?point ?zone1 ?zone2 ?reg1 ?reg2
WHERE {
  ?zone1 a legalis:LegalZone .
  ?zone1 geo:hasGeometry ?geom1 .
  ?zone1 legalis:regulatedBy ?reg1 .

  ?zone2 a legalis:LegalZone .
  ?zone2 geo:hasGeometry ?geom2 .
  ?zone2 legalis:regulatedBy ?reg2 .

  FILTER (?zone1 != ?zone2)
  FILTER (?reg1 != ?reg2)
  FILTER (geof:sfIntersects(?geom1, ?geom2))
}
"#
        .to_string()
    }
}

/// Zone hierarchy builder for inferring containment relationships
#[derive(Debug, Clone)]
pub struct ZoneHierarchy {
    /// Parent-child relationships
    containment: HashMap<String, Vec<String>>,
}

impl ZoneHierarchy {
    /// Creates a new zone hierarchy
    pub fn new() -> Self {
        Self {
            containment: HashMap::new(),
        }
    }

    /// Adds a containment relationship
    pub fn add_containment(&mut self, parent: impl Into<String>, child: impl Into<String>) {
        self.containment
            .entry(parent.into())
            .or_default()
            .push(child.into());
    }

    /// Gets all children of a zone
    pub fn children(&self, parent: &str) -> Vec<&String> {
        self.containment
            .get(parent)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all descendants of a zone (recursive)
    pub fn descendants(&self, parent: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut to_visit = vec![parent.to_string()];

        while let Some(current) = to_visit.pop() {
            if let Some(children) = self.containment.get(&current) {
                for child in children {
                    result.push(child.clone());
                    to_visit.push(child.clone());
                }
            }
        }

        result
    }

    /// Generates RDF triples for the hierarchy
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        for (parent, children) in &self.containment {
            for child in children {
                triples.push(Triple {
                    subject: child.clone(),
                    predicate: "geo:sfWithin".to_string(),
                    object: RdfValue::Uri(parent.clone()),
                });
                triples.push(Triple {
                    subject: parent.clone(),
                    predicate: "geo:sfContains".to_string(),
                    object: RdfValue::Uri(child.clone()),
                });
            }
        }

        triples
    }
}

impl Default for ZoneHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geosparql::Geometry;

    #[test]
    fn test_spatial_reasoner_creation() {
        let reasoner = SpatialReasoner::new("https://example.org/");
        assert_eq!(reasoner.zones.len(), 0);
        assert_eq!(reasoner.relationships.len(), 0);
    }

    #[test]
    fn test_add_zone() {
        let mut reasoner = SpatialReasoner::new("https://example.org/");
        let zone = LegalZone::new(
            "https://example.org/zone/1",
            "Test Zone",
            "zoning",
            Geometry::point(0.0, 0.0),
            "https://example.org/jurisdiction/1",
        );
        reasoner.add_zone(zone);
        assert_eq!(reasoner.zones.len(), 1);
    }

    #[test]
    fn test_applicable_at_point_query() {
        let query = RegulationApplicability::applicable_at_point(139.6917, 35.6895);
        assert!(query.contains("geof:sfContains"));
        assert!(query.contains("POINT (139.6917 35.6895)"));
        assert!(query.contains("legalis:regulatedBy"));
    }

    #[test]
    fn test_regulations_in_jurisdiction_query() {
        let query = RegulationApplicability::regulations_in_jurisdiction(
            "https://example.org/jurisdiction/tokyo",
        );
        assert!(query.contains("eli:jurisdiction"));
        assert!(query.contains("legalis:regulatedBy"));
    }

    #[test]
    fn test_zone_hierarchy() {
        let mut hierarchy = ZoneHierarchy::new();
        hierarchy.add_containment("parent", "child1");
        hierarchy.add_containment("parent", "child2");
        hierarchy.add_containment("child1", "grandchild");

        assert_eq!(hierarchy.children("parent").len(), 2);
        assert_eq!(hierarchy.descendants("parent").len(), 3);
    }

    #[test]
    fn test_hierarchy_triples() {
        let mut hierarchy = ZoneHierarchy::new();
        hierarchy.add_containment(
            "https://example.org/zone/parent",
            "https://example.org/zone/child",
        );

        let triples = hierarchy.to_triples();
        assert!(triples.iter().any(|t| t.predicate == "geo:sfWithin"));
        assert!(triples.iter().any(|t| t.predicate == "geo:sfContains"));
    }
}
