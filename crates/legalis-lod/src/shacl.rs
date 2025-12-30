//! SHACL (Shapes Constraint Language) shape generation.
//!
//! This module provides utilities to generate SHACL shapes for validating
//! legal statute RDF data. SHACL is a W3C recommendation for describing
//! and validating RDF graphs.

use crate::{Namespaces, RdfValue, Triple};

/// SHACL shape generator.
#[derive(Debug)]
pub struct ShaclShapeGenerator {
    namespaces: Namespaces,
}

impl Default for ShaclShapeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ShaclShapeGenerator {
    /// Creates a new SHACL shape generator.
    pub fn new() -> Self {
        Self {
            namespaces: Namespaces::default(),
        }
    }

    /// Creates a new SHACL shape generator with custom namespaces.
    pub fn with_namespaces(namespaces: Namespaces) -> Self {
        Self { namespaces }
    }

    /// Generates SHACL shapes for Legalis statute validation.
    pub fn generate_statute_shapes(&self) -> Vec<Triple> {
        let mut shapes = Vec::new();

        // Statute shape
        shapes.extend(self.create_statute_shape());

        // Effect shape
        shapes.extend(self.create_effect_shape());

        // Condition shapes
        shapes.extend(self.create_age_condition_shape());
        shapes.extend(self.create_income_condition_shape());

        shapes
    }

    /// Generates comprehensive SHACL shape library including custom ontology shapes.
    pub fn generate_comprehensive_shapes(&self) -> Vec<Triple> {
        let mut shapes = Vec::new();

        // Base shapes
        shapes.extend(self.generate_statute_shapes());

        // Custom ontology shapes
        shapes.extend(self.create_discretion_zone_shape());
        shapes.extend(self.create_simulation_result_shape());
        shapes.extend(self.create_temporal_snapshot_shape());

        // Knowledge graph shapes
        shapes.extend(self.create_entity_shape());
        shapes.extend(self.create_relationship_shape());

        shapes
    }

    /// Creates the Discretion Zone shape.
    fn create_discretion_zone_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/DiscretionZoneShape", self.namespaces.base);

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:DiscretionZone".to_string()),
        });

        // Label property (required)
        let label_prop = format!("{}/label", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(label_prop.clone()),
        });

        triples.push(Triple {
            subject: label_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("rdfs:label".to_string()),
        });

        triples.push(Triple {
            subject: label_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: label_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        // Definition property (required)
        let def_prop = format!("{}/definition", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(def_prop.clone()),
        });

        triples.push(Triple {
            subject: def_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("skos:definition".to_string()),
        });

        triples.push(Triple {
            subject: def_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: def_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        triples
    }

    /// Creates the Simulation Result shape.
    fn create_simulation_result_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/SimulationResultShape", self.namespaces.base);

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:SimulationResult".to_string()),
        });

        // Label property (required)
        let label_prop = format!("{}/label", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(label_prop.clone()),
        });

        triples.push(Triple {
            subject: label_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("rdfs:label".to_string()),
        });

        triples.push(Triple {
            subject: label_prop,
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        // Applies to property (required)
        let applies_prop = format!("{}/appliesTo", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(applies_prop.clone()),
        });

        triples.push(Triple {
            subject: applies_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:appliesTo".to_string()),
        });

        triples.push(Triple {
            subject: applies_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: applies_prop,
            predicate: "sh:nodeKind".to_string(),
            object: RdfValue::Uri("sh:IRI".to_string()),
        });

        triples
    }

    /// Creates the Temporal Snapshot shape.
    fn create_temporal_snapshot_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/TemporalSnapshotShape", self.namespaces.base);

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:TemporalSnapshot".to_string()),
        });

        // Snapshot of property (required)
        let snapshot_prop = format!("{}/snapshotOf", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(snapshot_prop.clone()),
        });

        triples.push(Triple {
            subject: snapshot_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:snapshotOf".to_string()),
        });

        triples.push(Triple {
            subject: snapshot_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: snapshot_prop,
            predicate: "sh:nodeKind".to_string(),
            object: RdfValue::Uri("sh:IRI".to_string()),
        });

        // Valid from property (optional, must be date if present)
        let from_prop = format!("{}/validFrom", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(from_prop.clone()),
        });

        triples.push(Triple {
            subject: from_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:validFrom".to_string()),
        });

        triples.push(Triple {
            subject: from_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:date".to_string()),
        });

        triples
    }

    /// Creates a generic Entity shape.
    fn create_entity_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/EntityShape", self.namespaces.base);

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        // Label property (optional but recommended)
        let label_prop = format!("{}/label", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(label_prop.clone()),
        });

        triples.push(Triple {
            subject: label_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("rdfs:label".to_string()),
        });

        triples.push(Triple {
            subject: label_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        triples
    }

    /// Creates a Relationship shape.
    fn create_relationship_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/RelationshipShape", self.namespaces.base);

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:description".to_string(),
            object: RdfValue::string(
                "Validates that relationships have valid subjects and objects",
            ),
        });

        // Subject must be an IRI
        let subj_prop = format!("{}/subject", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(subj_prop.clone()),
        });

        triples.push(Triple {
            subject: subj_prop,
            predicate: "sh:nodeKind".to_string(),
            object: RdfValue::Uri("sh:IRI".to_string()),
        });

        triples
    }

    /// Creates the main Statute shape.
    fn create_statute_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/StatuteShape", self.namespaces.base);

        // Shape type
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        // Target class
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        });

        // Title property (required)
        let title_prop = format!("{}/title", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(title_prop.clone()),
        });

        triples.push(Triple {
            subject: title_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("eli:title".to_string()),
        });

        triples.push(Triple {
            subject: title_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: title_prop.clone(),
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: title_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        // Identifier property (required)
        let id_prop = format!("{}/identifier", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(id_prop.clone()),
        });

        triples.push(Triple {
            subject: id_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("dcterms:identifier".to_string()),
        });

        triples.push(Triple {
            subject: id_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: id_prop.clone(),
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: id_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        // Effect property (required)
        let effect_prop = format!("{}/effect", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(effect_prop.clone()),
        });

        triples.push(Triple {
            subject: effect_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:hasEffect".to_string()),
        });

        triples.push(Triple {
            subject: effect_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: effect_prop.clone(),
            predicate: "sh:class".to_string(),
            object: RdfValue::Uri("legalis:Effect".to_string()),
        });

        triples.push(Triple {
            subject: effect_prop,
            predicate: "sh:node".to_string(),
            object: RdfValue::Uri(format!("{}shapes/EffectShape", self.namespaces.base)),
        });

        // Version property (optional, but must be integer if present)
        let version_prop = format!("{}/version", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(version_prop.clone()),
        });

        triples.push(Triple {
            subject: version_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("eli:version".to_string()),
        });

        triples.push(Triple {
            subject: version_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:integer".to_string()),
        });

        triples
    }

    /// Creates the Effect shape.
    fn create_effect_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/EffectShape", self.namespaces.base);

        // Shape type
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        // Target class
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:Effect".to_string()),
        });

        // Effect type property (required)
        let type_prop = format!("{}/effectType", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(type_prop.clone()),
        });

        triples.push(Triple {
            subject: type_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:effectType".to_string()),
        });

        triples.push(Triple {
            subject: type_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: type_prop.clone(),
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        // Effect type must be one of the valid effect types
        triples.push(Triple {
            subject: type_prop,
            predicate: "sh:in".to_string(),
            object: RdfValue::BlankNode("effectTypes".to_string()),
        });

        // List of valid effect types
        triples.push(Triple {
            subject: "_:effectTypes".to_string(),
            predicate: "rdf:first".to_string(),
            object: RdfValue::Uri("legalis:GrantEffect".to_string()),
        });

        triples.push(Triple {
            subject: "_:effectTypes".to_string(),
            predicate: "rdf:rest".to_string(),
            object: RdfValue::BlankNode("effectTypes2".to_string()),
        });

        triples.push(Triple {
            subject: "_:effectTypes2".to_string(),
            predicate: "rdf:first".to_string(),
            object: RdfValue::Uri("legalis:RevokeEffect".to_string()),
        });

        triples.push(Triple {
            subject: "_:effectTypes2".to_string(),
            predicate: "rdf:rest".to_string(),
            object: RdfValue::BlankNode("effectTypes3".to_string()),
        });

        triples.push(Triple {
            subject: "_:effectTypes3".to_string(),
            predicate: "rdf:first".to_string(),
            object: RdfValue::Uri("legalis:MonetaryTransferEffect".to_string()),
        });

        triples.push(Triple {
            subject: "_:effectTypes3".to_string(),
            predicate: "rdf:rest".to_string(),
            object: RdfValue::Uri("rdf:nil".to_string()),
        });

        // Description property (required)
        let desc_prop = format!("{}/description", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(desc_prop.clone()),
        });

        triples.push(Triple {
            subject: desc_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("rdfs:label".to_string()),
        });

        triples.push(Triple {
            subject: desc_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: desc_prop,
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:string".to_string()),
        });

        triples
    }

    /// Creates the Age Condition shape.
    fn create_age_condition_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/AgeConditionShape", self.namespaces.base);

        // Shape type
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        // Target class
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:AgeCondition".to_string()),
        });

        // Operator property (required)
        let op_prop = format!("{}/operator", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(op_prop.clone()),
        });

        triples.push(Triple {
            subject: op_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:operator".to_string()),
        });

        triples.push(Triple {
            subject: op_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: op_prop,
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        // Value property (required, must be non-negative)
        let value_prop = format!("{}/value", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(value_prop.clone()),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:value".to_string()),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:integer".to_string()),
        });

        triples.push(Triple {
            subject: value_prop,
            predicate: "sh:minInclusive".to_string(),
            object: RdfValue::integer(0),
        });

        triples
    }

    /// Creates the Income Condition shape.
    fn create_income_condition_shape(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let shape_uri = format!("{}shapes/IncomeConditionShape", self.namespaces.base);

        // Shape type
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("sh:NodeShape".to_string()),
        });

        // Target class
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:targetClass".to_string(),
            object: RdfValue::Uri("legalis:IncomeCondition".to_string()),
        });

        // Operator property (required)
        let op_prop = format!("{}/operator", shape_uri);
        triples.push(Triple {
            subject: shape_uri.clone(),
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(op_prop.clone()),
        });

        triples.push(Triple {
            subject: op_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:operator".to_string()),
        });

        triples.push(Triple {
            subject: op_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: op_prop,
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        // Value property (required, must be non-negative)
        let value_prop = format!("{}/value", shape_uri);
        triples.push(Triple {
            subject: shape_uri,
            predicate: "sh:property".to_string(),
            object: RdfValue::Uri(value_prop.clone()),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:path".to_string(),
            object: RdfValue::Uri("legalis:value".to_string()),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:minCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:maxCount".to_string(),
            object: RdfValue::integer(1),
        });

        triples.push(Triple {
            subject: value_prop.clone(),
            predicate: "sh:datatype".to_string(),
            object: RdfValue::Uri("xsd:integer".to_string()),
        });

        triples.push(Triple {
            subject: value_prop,
            predicate: "sh:minInclusive".to_string(),
            object: RdfValue::integer(0),
        });

        triples
    }

    /// Exports SHACL shapes as Turtle.
    pub fn export_shapes_turtle(&self) -> String {
        let shapes = self.generate_statute_shapes();
        let mut output = String::new();

        // Prefixes
        for (prefix, uri) in Namespaces::standard_prefixes() {
            output.push_str(&format!("@prefix {}: <{}> .\n", prefix, uri));
        }
        output.push_str("@prefix sh: <http://www.w3.org/ns/shacl#> .\n");
        output.push_str(&format!("@base <{}> .\n", self.namespaces.base));
        output.push('\n');

        // Group triples by subject
        let mut by_subject: std::collections::HashMap<&str, Vec<&Triple>> =
            std::collections::HashMap::new();
        for triple in &shapes {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }

        for (subject, subject_triples) in by_subject {
            output.push_str(&format!("<{}>\n", subject));
            for (i, triple) in subject_triples.iter().enumerate() {
                let sep = if i == 0 { "    " } else { " ;\n    " };
                output.push_str(sep);
                output.push_str(&triple.predicate);
                output.push(' ');
                output.push_str(&value_to_turtle(&triple.object));
            }
            output.push_str(" .\n\n");
        }

        output
    }
}

/// Helper function to convert RdfValue to Turtle representation.
fn value_to_turtle(value: &RdfValue) -> String {
    match value {
        RdfValue::Uri(uri) => {
            if uri.contains(':') && !uri.starts_with("http://") && !uri.starts_with("https://") {
                uri.clone()
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

/// Escapes a string for Turtle.
fn escape_string(s: &str) -> String {
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
    fn test_generate_statute_shapes() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.generate_statute_shapes();

        assert!(!shapes.is_empty());
        assert!(shapes.iter().any(|t| t.predicate == "sh:targetClass"));
    }

    #[test]
    fn test_statute_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_statute_shape();

        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "sh:NodeShape"))
        );
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:Statute"))
        );
    }

    #[test]
    fn test_effect_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_effect_shape();

        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:Effect"))
        );
        assert!(shapes.iter().any(|t| t.predicate == "sh:in"));
    }

    #[test]
    fn test_age_condition_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_age_condition_shape();

        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:AgeCondition"))
        );
        assert!(shapes.iter().any(|t| t.predicate == "sh:minInclusive"));
    }

    #[test]
    fn test_export_shapes_turtle() {
        let generator = ShaclShapeGenerator::new();
        let turtle = generator.export_shapes_turtle();

        assert!(turtle.contains("@prefix sh:"));
        assert!(turtle.contains("sh:NodeShape"));
        assert!(turtle.contains("sh:targetClass"));
    }

    #[test]
    fn test_comprehensive_shapes() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.generate_comprehensive_shapes();

        // Should have more shapes than basic statute shapes
        let basic_count = generator.generate_statute_shapes().len();
        assert!(shapes.len() > basic_count);

        // Should contain custom ontology shapes
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("DiscretionZone")))
        );
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("SimulationResult")))
        );
    }

    #[test]
    fn test_discretion_zone_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_discretion_zone_shape();

        assert!(!shapes.is_empty());
        assert!(shapes.iter().any(|t| t.predicate == "sh:targetClass"));
        assert!(shapes.iter().any(|t| t.predicate == "sh:property"));
    }

    #[test]
    fn test_simulation_result_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_simulation_result_shape();

        assert!(!shapes.is_empty());
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:SimulationResult"))
        );
    }

    #[test]
    fn test_temporal_snapshot_shape() {
        let generator = ShaclShapeGenerator::new();
        let shapes = generator.create_temporal_snapshot_shape();

        assert!(!shapes.is_empty());
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "legalis:TemporalSnapshot"))
        );
        assert!(
            shapes
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "xsd:date"))
        );
    }
}
