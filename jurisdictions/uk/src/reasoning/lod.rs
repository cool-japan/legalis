//! UK Linked Open Data Integration (legislation.gov.uk).
//!
//! This module provides integration with legislation.gov.uk, the official UK legislation database,
//! enabling RDF export and linking to official UK legislation.
//!
//! ## Key Features
//! - Map UK Acts and Regulations to legislation.gov.uk URIs
//! - Export UK statutes to RDF/Turtle format
//! - Link to official legislation.gov.uk URIs with owl:sameAs
//! - Support for Acts of Parliament, Statutory Instruments, Scottish Acts

use legalis_core::{EffectType, Statute};
use legalis_lod::external::legislation_gov_uk::{self, LegislationType};
use legalis_lod::{LodResult, RdfValue, Triple};
use std::collections::HashMap;

/// UK legislation URI mapper
pub struct LegislationMapper {
    /// Mapping from statute ID to (type, year, number)
    mappings: HashMap<String, (LegislationType, u16, u32)>,
}

impl LegislationMapper {
    /// Creates a new legislation mapper with common UK employment law mappings
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        // Employment Rights Act 1996
        mappings.insert(
            "ERA_1996_S1".to_string(),
            (LegislationType::UkPublicGeneralAct, 1996, 18),
        );
        mappings.insert(
            "ERA_1996_S86".to_string(),
            (LegislationType::UkPublicGeneralAct, 1996, 18),
        );
        mappings.insert(
            "ERA_1996_S98".to_string(),
            (LegislationType::UkPublicGeneralAct, 1996, 18),
        );
        mappings.insert(
            "ERA_1996_S162".to_string(),
            (LegislationType::UkPublicGeneralAct, 1996, 18),
        );

        // National Minimum Wage Act 1998
        mappings.insert(
            "NMWA_1998".to_string(),
            (LegislationType::UkPublicGeneralAct, 1998, 39),
        );

        // Working Time Regulations 1998
        mappings.insert(
            "WTR_1998_Reg4".to_string(),
            (LegislationType::UkStatutoryInstrument, 1998, 1833),
        );
        mappings.insert(
            "WTR_1998_Reg12".to_string(),
            (LegislationType::UkStatutoryInstrument, 1998, 1833),
        );
        mappings.insert(
            "WTR_1998_Reg13".to_string(),
            (LegislationType::UkStatutoryInstrument, 1998, 1833),
        );

        // Equality Act 2010
        mappings.insert(
            "EA_2010".to_string(),
            (LegislationType::UkPublicGeneralAct, 2010, 15),
        );

        // Data Protection Act 2018
        mappings.insert(
            "DPA_2018".to_string(),
            (LegislationType::UkPublicGeneralAct, 2018, 12),
        );

        Self { mappings }
    }

    /// Gets the legislation.gov.uk info for a statute ID
    pub fn get_legislation_info(&self, statute_id: &str) -> Option<&(LegislationType, u16, u32)> {
        self.mappings.get(statute_id)
    }

    /// Adds a custom mapping
    pub fn add_mapping(
        &mut self,
        statute_id: String,
        leg_type: LegislationType,
        year: u16,
        number: u32,
    ) {
        self.mappings.insert(statute_id, (leg_type, year, number));
    }
}

impl Default for LegislationMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// legislation.gov.uk exporter for UK statutes
pub struct LegislationExporter {
    mapper: LegislationMapper,
    base_namespace: String,
}

impl LegislationExporter {
    /// Creates a new legislation.gov.uk exporter
    pub fn new() -> Self {
        Self {
            mapper: LegislationMapper::new(),
            base_namespace: "http://legalis.rs/uk/".to_string(),
        }
    }

    /// Creates a new exporter with custom namespace
    pub fn with_namespace(namespace: String) -> Self {
        Self {
            mapper: LegislationMapper::new(),
            base_namespace: namespace,
        }
    }

    /// Exports a statute to RDF with legislation.gov.uk linking
    pub fn export_statute(&self, statute: &Statute) -> LodResult<Vec<Triple>> {
        let mut triples = Vec::new();

        let subject_uri = format!("{}{}", self.base_namespace, statute.id);

        // Add basic statute metadata
        triples.push(Triple {
            subject: subject_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("legalis:Statute".to_string()),
        });

        triples.push(Triple {
            subject: subject_uri.clone(),
            predicate: "dcterms:identifier".to_string(),
            object: RdfValue::string(&statute.id),
        });

        triples.push(Triple {
            subject: subject_uri.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&statute.title),
        });

        // Link to legislation.gov.uk if mapping exists
        if let Some((leg_type, year, number)) = self.mapper.get_legislation_info(&statute.id) {
            triples.extend(legislation_gov_uk::create_legislation_link(
                &subject_uri,
                *leg_type,
                *year,
                *number,
            ));

            // Add legislation type metadata
            triples.push(Triple {
                subject: subject_uri.clone(),
                predicate: "legalis:legislationType".to_string(),
                object: RdfValue::string(leg_type.label()),
            });
        }

        // Add jurisdiction
        triples.push(Triple {
            subject: subject_uri.clone(),
            predicate: "dcterms:coverage".to_string(),
            object: RdfValue::string(statute.jurisdiction.as_ref().unwrap_or(&"UK".to_string())),
        });

        // Add effect type
        let effect_type = match &statute.effect.effect_type {
            EffectType::Grant => "legalis:Grant",
            EffectType::Revoke => "legalis:Revoke",
            EffectType::Obligation => "legalis:Obligation",
            EffectType::Prohibition => "legalis:Prohibition",
            EffectType::MonetaryTransfer => "legalis:MonetaryTransfer",
            EffectType::StatusChange => "legalis:StatusChange",
            EffectType::Custom => "legalis:CustomEffect",
        };

        triples.push(Triple {
            subject: subject_uri,
            predicate: "legalis:effectType".to_string(),
            object: RdfValue::Uri(effect_type.to_string()),
        });

        Ok(triples)
    }

    /// Exports statutes to Turtle format
    pub fn export_to_turtle(&self, statutes: &[Statute]) -> LodResult<String> {
        let mut output = String::new();

        // Prefixes
        output.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
        output.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
        output.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
        output.push_str("@prefix dcterms: <http://purl.org/dc/terms/> .\n");
        output.push_str("@prefix uklegis: <http://www.legislation.gov.uk/id/> .\n");
        output.push_str("@prefix legalis: <http://legalis.rs/ontology#> .\n");
        output.push('\n');

        // Export each statute
        for statute in statutes {
            let triples = self.export_statute(statute)?;
            for triple in triples {
                output.push_str(&self.triple_to_turtle(&triple));
                output.push('\n');
            }
            output.push('\n');
        }

        Ok(output)
    }

    /// Converts a triple to Turtle format
    fn triple_to_turtle(&self, triple: &Triple) -> String {
        let object_str = match &triple.object {
            RdfValue::Uri(uri) => format!("<{}>", uri),
            RdfValue::Literal(lit, None) => format!("\"{}\"", lit.replace('\"', "\\\"")),
            RdfValue::Literal(lit, Some(lang)) => {
                format!("\"{}\"@{}", lit.replace('\"', "\\\""), lang)
            }
            RdfValue::TypedLiteral(lit, datatype) => {
                format!("\"{}\"^^<{}>", lit.replace('\"', "\\\""), datatype)
            }
            RdfValue::BlankNode(id) => format!("_:{}", id),
        };

        format!("<{}> {} {} .", triple.subject, triple.predicate, object_str)
    }
}

impl Default for LegislationExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to export UK employment statutes with legislation.gov.uk linking
pub fn export_employment_statutes_to_legislation(statutes: &[Statute]) -> LodResult<String> {
    let exporter = LegislationExporter::new();
    exporter.export_to_turtle(statutes)
}

/// Helper function to get legislation.gov.uk URI for a statute
pub fn get_legislation_uri(statute_id: &str) -> Option<String> {
    let mapper = LegislationMapper::new();
    mapper
        .get_legislation_info(statute_id)
        .map(|(leg_type, year, number)| {
            format!(
                "{}{}/{}/{}",
                legislation_gov_uk::LINKED_DATA_URI,
                leg_type.path_segment(),
                year,
                number
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::Effect;

    #[test]
    fn test_legislation_mapper() {
        let mapper = LegislationMapper::new();

        assert!(mapper.get_legislation_info("ERA_1996_S1").is_some());
        assert!(mapper.get_legislation_info("NMWA_1998").is_some());
        assert!(mapper.get_legislation_info("WTR_1998_Reg4").is_some());
        assert_eq!(mapper.get_legislation_info("NonExistent"), None);
    }

    #[test]
    fn test_era_mapping() {
        let mapper = LegislationMapper::new();
        let info = mapper.get_legislation_info("ERA_1996_S1").unwrap();

        assert_eq!(info.0, LegislationType::UkPublicGeneralAct);
        assert_eq!(info.1, 1996);
        assert_eq!(info.2, 18);
    }

    #[test]
    fn test_legislation_exporter() {
        let exporter = LegislationExporter::new();
        let statute = Statute::new(
            "ERA_1996_S1",
            "Employment Rights Act 1996 Section 1: Written particulars",
            Effect::new(
                EffectType::Obligation,
                "Employer must provide written statement of terms",
            ),
        )
        .with_jurisdiction("UK");

        let triples = exporter.export_statute(&statute).unwrap();

        // Should have basic metadata + legislation.gov.uk links
        assert!(!triples.is_empty());
        assert!(
            triples.iter().any(|t| t.predicate == "owl:sameAs"
                && t.object.to_string().contains("legislation.gov.uk"))
        );
    }

    #[test]
    fn test_turtle_export() {
        let exporter = LegislationExporter::new();
        let statute = Statute::new(
            "ERA_1996_S1",
            "ERA 1996 Section 1",
            Effect::new(EffectType::Obligation, "Written statement required"),
        );

        let turtle = exporter.export_to_turtle(&[statute]).unwrap();

        assert!(turtle.contains("@prefix"));
        assert!(turtle.contains("owl:sameAs"));
        assert!(turtle.contains("legislation.gov.uk"));
    }

    #[test]
    fn test_get_legislation_uri() {
        let uri = get_legislation_uri("ERA_1996_S1");
        assert!(uri.is_some());
        assert!(uri.unwrap().contains("legislation.gov.uk/id/ukpga/1996/18"));
    }

    #[test]
    fn test_wtr_mapping() {
        let uri = get_legislation_uri("WTR_1998_Reg4");
        assert!(uri.is_some());
        assert!(
            uri.unwrap()
                .contains("legislation.gov.uk/id/uksi/1998/1833")
        );
    }

    #[test]
    fn test_custom_mapping() {
        let mut mapper = LegislationMapper::new();
        mapper.add_mapping(
            "Custom_Act".to_string(),
            LegislationType::UkPublicGeneralAct,
            2024,
            42,
        );

        let info = mapper.get_legislation_info("Custom_Act").unwrap();
        assert_eq!(info.1, 2024);
        assert_eq!(info.2, 42);
    }
}
