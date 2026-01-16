//! EU Linked Open Data Integration (EUR-Lex).
//!
//! This module provides integration with EUR-Lex, the official legal database of the EU,
//! enabling RDF export and linking to official EU legislation.
//!
//! ## Key Features
//! - Map GDPR articles to CELEX numbers (EUR-Lex identifiers)
//! - Export EU statutes to RDF/Turtle format
//! - Link to official EUR-Lex URIs with owl:sameAs
//! - SPARQL-ready triple generation

use legalis_core::{EffectType, Statute};
use legalis_lod::external::eurlex;
use legalis_lod::{LodResult, RdfValue, Triple};
use std::collections::HashMap;

/// CELEX number mapper for EU legislation
pub struct CelexMapper {
    /// Mapping from statute ID to CELEX number
    mappings: HashMap<String, String>,
}

impl CelexMapper {
    /// Creates a new CELEX mapper with GDPR and treaty mappings
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        // GDPR Articles (CELEX: 32016R0679)
        mappings.insert("GDPR_Art5".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art6".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art7".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art8".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art9".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art13".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art14".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art15".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art17".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art22".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art25".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art32".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art33".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art35".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art44".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art45".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art46".to_string(), "32016R0679".to_string());
        mappings.insert("GDPR_Art83".to_string(), "32016R0679".to_string());

        // EU Treaties
        mappings.insert("TFEU_Art101".to_string(), "12012E101".to_string()); // Competition
        mappings.insert("TFEU_Art102".to_string(), "12012E102".to_string());
        mappings.insert("TFEU_Art107".to_string(), "12012E107".to_string()); // State Aid

        // Consumer Rights Directive (CELEX: 32011L0083)
        mappings.insert("CRD_Art3".to_string(), "32011L0083".to_string());
        mappings.insert("CRD_Art6".to_string(), "32011L0083".to_string());
        mappings.insert("CRD_Art9".to_string(), "32011L0083".to_string());
        mappings.insert("CRD_Art14".to_string(), "32011L0083".to_string());

        Self { mappings }
    }

    /// Gets the CELEX number for a statute ID
    pub fn get_celex(&self, statute_id: &str) -> Option<&str> {
        self.mappings.get(statute_id).map(|s| s.as_str())
    }

    /// Adds a custom mapping
    pub fn add_mapping(&mut self, statute_id: String, celex: String) {
        self.mappings.insert(statute_id, celex);
    }
}

impl Default for CelexMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// EUR-Lex exporter for EU statutes
pub struct EurLexExporter {
    mapper: CelexMapper,
    base_namespace: String,
}

impl EurLexExporter {
    /// Creates a new EUR-Lex exporter
    pub fn new() -> Self {
        Self {
            mapper: CelexMapper::new(),
            base_namespace: "http://legalis.rs/eu/".to_string(),
        }
    }

    /// Creates a new exporter with custom namespace
    pub fn with_namespace(namespace: String) -> Self {
        Self {
            mapper: CelexMapper::new(),
            base_namespace: namespace,
        }
    }

    /// Exports a statute to RDF with EUR-Lex linking
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

        // Link to EUR-Lex if mapping exists
        if let Some(celex) = self.mapper.get_celex(&statute.id) {
            triples.extend(eurlex::add_eurlex_metadata(
                &subject_uri,
                celex,
                Some(&statute.title),
                None,
            ));
        }

        // Add jurisdiction
        triples.push(Triple {
            subject: subject_uri.clone(),
            predicate: "dcterms:coverage".to_string(),
            object: RdfValue::string(statute.jurisdiction.as_ref().unwrap_or(&"EU".to_string())),
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
        output.push_str("@prefix eli: <http://data.europa.eu/eli/ontology#> .\n");
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

impl Default for EurLexExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to export GDPR statutes with EUR-Lex linking
pub fn export_gdpr_statutes_to_eurlex(statutes: &[Statute]) -> LodResult<String> {
    let exporter = EurLexExporter::new();
    exporter.export_to_turtle(statutes)
}

/// Helper function to get EUR-Lex URI for a statute
pub fn get_eurlex_uri(statute_id: &str) -> Option<String> {
    let mapper = CelexMapper::new();
    mapper
        .get_celex(statute_id)
        .map(|celex| format!("{}{}", eurlex::BASE_URI, celex))
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_celex_mapper() {
        let mapper = CelexMapper::new();

        assert_eq!(mapper.get_celex("GDPR_Art6"), Some("32016R0679"));
        assert_eq!(mapper.get_celex("GDPR_Art15"), Some("32016R0679"));
        assert_eq!(mapper.get_celex("TFEU_Art101"), Some("12012E101"));
        assert_eq!(mapper.get_celex("NonExistent"), None);
    }

    #[test]
    fn test_eurlex_exporter() {
        let exporter = EurLexExporter::new();
        let statute = Statute::new(
            "GDPR_Art6",
            "GDPR Article 6: Lawfulness of processing",
            Effect::new(
                EffectType::Obligation,
                "Must have lawful basis for processing",
            ),
        )
        .with_jurisdiction("EU");

        let triples = exporter.export_statute(&statute).unwrap();

        // Should have basic metadata + EUR-Lex links
        assert!(!triples.is_empty());
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "owl:sameAs" && t.object.to_string().contains("europa.eu"))
        );
    }

    #[test]
    fn test_turtle_export() {
        let exporter = EurLexExporter::new();
        let statute = Statute::new(
            "GDPR_Art6",
            "GDPR Article 6",
            Effect::new(EffectType::Obligation, "Lawful basis required"),
        );

        let turtle = exporter.export_to_turtle(&[statute]).unwrap();

        assert!(turtle.contains("@prefix"));
        assert!(turtle.contains("owl:sameAs"));
        assert!(turtle.contains("32016R0679"));
    }

    #[test]
    fn test_get_eurlex_uri() {
        let uri = get_eurlex_uri("GDPR_Art6");
        assert!(uri.is_some());
        assert!(uri.unwrap().contains("data.europa.eu/eli/32016R0679"));
    }

    #[test]
    fn test_custom_mapping() {
        let mut mapper = CelexMapper::new();
        mapper.add_mapping("Custom_Directive".to_string(), "32024L1234".to_string());

        assert_eq!(mapper.get_celex("Custom_Directive"), Some("32024L1234"));
    }
}
